param(
    [switch]$Draw,
    [string]$Seed = '20261025-kairos-stage2-own-ab-v2',
    [string]$Sample = 'stage2-own-sample.tsv',
    [int]$PerLanguage = 300
)

$ErrorActionPreference = 'Stop'
$root = Join-Path $PSScriptRoot '../data/languages'
$samplePath = Join-Path $root $Sample
$metadataPath = "$samplePath.json"
function Hash([string]$value) {
    [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes($value))).ToLowerInvariant()
}
function Read-Gzip([string]$path) {
    $file = [IO.File]::OpenRead($path)
    $zip = [IO.Compression.GZipStream]::new($file, [IO.Compression.CompressionMode]::Decompress)
    $reader = [IO.StreamReader]::new($zip, [Text.Encoding]::UTF8)
    try { $reader.ReadToEnd() } finally { $reader.Dispose(); $file.Dispose() }
}
$sources = @{ en = @{}; la = @{} }
foreach ($row in ((Read-Gzip (Join-Path $root 'upstream/wordnet/candidate-senses.tsv.gz')) -split "`n")) {
    if ($row) {
        $fields = $row -split "`t", 2
        $sources.en[$fields[0]] = $fields[1]
    }
}
foreach ($row in [IO.File]::ReadAllLines((Join-Path $root 'latin-glosses.tsv'))) {
    $fields = $row -split "`t", 2
    $sources.la[$fields[0]] = $fields[1]
}
if (!$Draw) {
    $metadata = Get-Content -LiteralPath $metadataPath -Raw | ConvertFrom-Json
    $Seed = $metadata.seed
    $PerLanguage = $metadata.perLanguage
}
if (!$Seed -or $PerLanguage -lt 1) { throw 'Invalid sample configuration' }
$selected = [Collections.Generic.Dictionary[string, object]]::new([StringComparer]::Ordinal)
$listHashes = @{}
foreach ($entry in @(@('en', 'english'), @('la', 'latin'))) {
    $language, $name = $entry
    $path = Join-Path $root "$name.txt"
    $listHashes[$language] = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
    if (!$Draw -and $metadata.listHashes.$language -cne $listHashes[$language]) {
        throw "Sample is stale for $language; assess the final list, do not reuse old verdicts silently"
    }
    $ranked = foreach ($word in [IO.File]::ReadAllLines($path)) {
        [pscustomobject]@{ word = $word; language = $language; rank = Hash "$Seed|$language|$word" }
    }
    if ($ranked.Count -lt $PerLanguage) { throw 'Sample exceeds vocabulary size' }
    foreach ($row in ($ranked | Sort-Object rank -CaseSensitive | Select-Object -First $PerLanguage)) {
        $selected.Add("$language/$($row.word)", $row)
    }
}
if ($Draw) {
    if (Test-Path -LiteralPath $samplePath) { throw 'Refusing to overwrite an existing assessment' }
    $rows = foreach ($key in ($selected.Keys | Sort-Object -CaseSensitive)) {
        $row = $selected[$key]
        "$($row.rank)`t$($row.word)`t$($row.language)`t$($sources[$row.language][$row.word])`tpending`t-`t-"
    }
    [IO.File]::WriteAllLines($samplePath, @($rows), [Text.UTF8Encoding]::new($false))
    $metadata = [ordered]@{ seed = $Seed; perLanguage = $PerLanguage; listHashes = $listHashes }
    [IO.File]::WriteAllText($metadataPath, (($metadata | ConvertTo-Json -Depth 3) + "`n"), [Text.UTF8Encoding]::new($false))
    "drawn=$($rows.Count); seed=$Seed; assess every pending row before running the gate"
    exit 0
}
$seen = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
$counts = @{ en = @{ A = 0; B = 0; N = 0 }; la = @{ A = 0; B = 0; N = 0 } }
foreach ($line in [IO.File]::ReadAllLines($samplePath)) {
    $fields = $line -split "`t"
    if ($fields.Count -ne 7 -or $fields[2] -cnotin @('en', 'la') -or $fields[4] -cnotin @('A', 'B', 'N') -or !$fields[6].Trim()) {
        throw 'Invalid or unassessed sample row'
    }
    $rank, $word, $language, $source, $verdict, $category, $reason = $fields
    if ($verdict -cin @('A', 'B') -and (!$category.Trim() -or $category.Trim() -eq '-' -or $reason.Trim() -eq '-')) {
        throw 'A/B verdict requires an explicit category and reason'
    }
    if ($category.StartsWith('Nebenbedeutung:', [StringComparison]::Ordinal)) {
        $secondaryClass = $category.Substring('Nebenbedeutung:'.Length)
        $expected = switch -CaseSensitive ($secondaryClass) {
            'Sexualitaet' { 'A' }
            'Vulgaer' { 'A' }
            'Slur' { 'A' }
            'Tier-Gegenstandsmetapher' { 'B' }
            default { throw "Unrecognized secondary-sense category: $category" }
        }
        if ($verdict -cne $expected) {
            throw "Secondary-sense rubric requires $expected for $category`: $language/$word"
        }
    }
    $key = "$language/$word"
    if (!$seen.Add($key) -or !$selected.ContainsKey($key) -or $selected[$key].rank -cne $rank -or $sources[$language][$word] -cne $source) {
        throw "Sample selection or meaning mismatch: $key"
    }
    $counts[$language][$verdict]++
    if ($verdict -cin @('A', 'B')) { "$key $verdict $category; $reason" }
}
if ($seen.Count -ne $selected.Count) { throw 'Incomplete semantic sample' }
foreach ($language in 'en', 'la') {
    "semantic_sample $language seed=$Seed size=$PerLanguage A=$($counts[$language].A) B=$($counts[$language].B) N=$($counts[$language].N)"
}
$found = $counts.en.A + $counts.la.A
"semantic_sample_findings=$found"
exit [int]($found -gt 0)
