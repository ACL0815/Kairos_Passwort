param(
    [Parameter(Mandatory)][string]$Before,
    [Parameter(Mandatory)][string]$After,
    [string]$SampleFile
)

# Scan artifacts as byte-preserving Latin-1 strings, including unaligned UTF-16LE.
# Search samples live only in this parent process, never in embedded-probe.exe.
$ErrorActionPreference = 'Stop'
$kairosRoot = Split-Path -Parent $PSScriptRoot
$kairosLists = @(
    @{ Language = 'de'; Path = 'data/german.txt'; Count = 7 },
    @{ Language = 'en'; Path = 'data/languages/english.txt'; Count = 7 },
    @{ Language = 'la'; Path = 'data/languages/latin.txt'; Count = 6 }
)
if ($SampleFile) {
    $kairosSamples = @((Get-Content -LiteralPath $SampleFile -Raw | ConvertFrom-Json).samples)
} else {
    $kairosSamples = @(foreach ($kairosList in $kairosLists) {
        # Long words avoid incidental matches in runtime/library diagnostic strings.
        $kairosPool = @(Get-Content -LiteralPath (Join-Path $kairosRoot $kairosList.Path) |
            Where-Object { $_.Length -ge 7 })
        $kairosSelected = [Collections.Generic.HashSet[int]]::new()
        while ($kairosSelected.Count -lt $kairosList.Count) {
            $kairosIndex = [Security.Cryptography.RandomNumberGenerator]::GetInt32($kairosPool.Count)
            if ($kairosSelected.Add($kairosIndex)) {
                [pscustomobject]@{ language = $kairosList.Language; word = $kairosPool[$kairosIndex] }
            }
        }
    })
}
if ($kairosSamples.Count -ne 20 -or @($kairosSamples.word | Select-Object -Unique).Count -ne 20) {
    throw 'Expected 20 distinct sample words'
}
foreach ($kairosList in $kairosLists) {
    $kairosDictionary = @(Get-Content -LiteralPath (Join-Path $kairosRoot $kairosList.Path))
    $kairosLanguageSamples = @($kairosSamples | Where-Object language -EQ $kairosList.Language)
    if ($kairosLanguageSamples.Count -ne $kairosList.Count) { throw 'Invalid language sample count' }
    foreach ($kairosSample in $kairosLanguageSamples) {
        if ($kairosSample.word.Length -lt 7 -or $kairosSample.word -cnotin $kairosDictionary) {
            throw 'Sample is not a long word in the reviewed dictionary'
        }
    }
}
$kairosBefore = [Text.Encoding]::Latin1.GetString([IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $Before)))
$kairosAfter = [Text.Encoding]::Latin1.GetString([IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $After)))
$kairosRows = @(foreach ($kairosSample in $kairosSamples) {
    $kairosWide = [Text.Encoding]::Latin1.GetString([Text.Encoding]::Unicode.GetBytes($kairosSample.word))
    [pscustomobject]@{
        language = $kairosSample.language
        word = $kairosSample.word
        beforeAscii = $kairosBefore.Contains($kairosSample.word, [StringComparison]::Ordinal)
        afterAscii = $kairosAfter.Contains($kairosSample.word, [StringComparison]::Ordinal)
        afterUtf16LE = $kairosAfter.Contains($kairosWide, [StringComparison]::Ordinal)
    }
})
$kairosLabels = @('kairos.wordlists.de.sha256.v1', 'kairos.wordlists.en.sha256.v1', 'kairos.wordlists.la.sha256.v1')
$kairosLabelHits = @($kairosLabels | Where-Object { $kairosAfter.Contains($_, [StringComparison]::Ordinal) })
[pscustomobject]@{
    measuredUtc = [DateTime]::UtcNow.ToString('o')
    selection = 'OS RNG, without replacement; DE 7 / EN 7 / LA 6, length >= 7'
    beforeSha256 = (Get-FileHash -LiteralPath $Before -Algorithm SHA256).Hash.ToLowerInvariant()
    afterSha256 = (Get-FileHash -LiteralPath $After -Algorithm SHA256).Hash.ToLowerInvariant()
    beforeBytes = (Get-Item -LiteralPath $Before).Length
    afterBytes = (Get-Item -LiteralPath $After).Length
    beforeAsciiHits = @($kairosRows | Where-Object beforeAscii).Count
    afterAsciiHits = @($kairosRows | Where-Object afterAscii).Count
    afterUtf16LEHits = @($kairosRows | Where-Object afterUtf16LE).Count
    afterIntegrityLabelHits = $kairosLabelHits.Count
    samples = $kairosRows
} | ConvertTo-Json -Depth 4
if (@($kairosRows | Where-Object { -not $_.beforeAscii -or $_.afterAscii -or $_.afterUtf16LE }).Count -gt 0 -or $kairosLabelHits.Count -gt 0) {
    throw 'Embedded-word scan failed: baseline must contain every sample; masked artifact must contain none'
}
