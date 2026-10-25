$ErrorActionPreference = 'Stop'
$root = Join-Path $PSScriptRoot '../data'
$langRoot = Join-Path $root 'languages'
function Hash([string]$text) {
    [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes($text))).ToLowerInvariant()
}
function Lemma([string]$line) {
    if ($line.Length -lt 110) { return }
    $stem = $line.Substring(0,19).Trim()
    if ($stem -eq 'zzz' -or $stem -cnotmatch '^[a-z]+$') { return }
    $g = $line.Substring(76,24).Trim() -split '\s+'
    $suffix = $null
    if ($g[0] -eq 'N' -and $g.Count -eq 5 -and $g[4] -ne 'N') {
        switch ("$($g[1]) $($g[2])") {
            '1 1' {$suffix='a'} '2 1' {$suffix='us'} '4 1' {$suffix='us'} '2 2' {$suffix='um'}
            '3 1' {$suffix=''} '3 2' {$suffix=''} '3 3' {$suffix=''} '3 4' {$suffix=''} '5 1' {$suffix='es'}
        }
    } elseif ($g[0] -eq 'ADJ' -and $g.Count -eq 4) {
        switch ("$($g[1]) $($g[2])") { '1 1' {$suffix='us'} '3 1' {$suffix=''} '3 3' {$suffix=''} '3 2' {$suffix='is'} }
    } elseif ($g[0] -eq 'V' -and $g.Count -eq 4) {
        if ($g[3] -in 'DEP','X','TRANS','INTRANS') {
            if ("$($g[1]) $($g[2])" -in '1 1','3 1','3 4') { if($g[3] -eq 'DEP'){$suffix='or'}else{$suffix='o'} }
            elseif ("$($g[1]) $($g[2])" -eq '2 1') { if($g[3] -eq 'DEP'){$suffix='eor'}else{$suffix='eo'} }
        }
    } elseif (($g[0] -eq 'ADV' -and $g[1] -in 'POS','X') -or $g[0] -in 'CONJ','PREP','INTERJ') { $suffix='' }
    if ($null -ne $suffix -and "$stem$suffix" -cmatch '^[a-z]{3,9}$') { "$stem$suffix" }
}
$en = [Collections.Generic.SortedSet[string]]::new([StringComparer]::Ordinal)
foreach($name in 'large','short1','short2') {
    foreach($line in [IO.File]::ReadAllLines((Join-Path $langRoot "upstream/eff/$name.txt"))) {
        $word=($line -split "`t")[1]
        if($word -cmatch '^[a-z]{3,9}$'){[void]$en.Add($word)}
    }
}
foreach($word in [IO.File]::ReadAllLines((Join-Path $langRoot 'english-supplement.txt'))){[void]$en.Add($word)}
$stream=[IO.File]::OpenRead((Join-Path $langRoot 'upstream/whitaker/DICTLINE.GEN.gz'))
$zip=[IO.Compression.GZipStream]::new($stream,[IO.Compression.CompressionMode]::Decompress)
$reader=[IO.StreamReader]::new($zip,[Text.Encoding]::Latin1)
$raw=$reader.ReadToEnd();$reader.Dispose();$stream.Dispose()
$gloss=@{};$eligible=[Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal);$index=0
foreach($line in ($raw -split "`n")) {
    $index++;$line=$line.TrimEnd("`r");$word=Lemma $line
    if(!$word){continue}
    if(!$gloss.ContainsKey($word)){$gloss[$word]=[Collections.Generic.List[string]]::new()}
    $freq=$line.Substring(106,1)
    $gloss[$word].Add("$index $freq $($line.Substring(110))")
    if($freq -cmatch '^[A-D]$'){[void]$eligible.Add($word)}
}
$rows=[Collections.Generic.List[string]]::new()
$stream=[IO.File]::OpenRead((Join-Path $langRoot 'upstream/wordnet/candidate-senses.tsv.gz'))
$zip=[IO.Compression.GZipStream]::new($stream,[IO.Compression.CompressionMode]::Decompress)
$reader=[IO.StreamReader]::new($zip,[Text.Encoding]::UTF8)
$englishSources=@{}
try {
    foreach($line in ($reader.ReadToEnd() -split "`n")) {
        if($line){$fields=$line -split "`t",2;$englishSources[$fields[0]]=$fields[1]}
    }
} finally {$reader.Dispose();$stream.Dispose()}
foreach($word in $en){
    $source=$englishSources[$word]
    if(!$source){throw "Missing English meaning source: $word"}
    $rows.Add("$word`ten`t$(Hash $source)`t$source")
}
foreach($word in $eligible){$meaning=$gloss[$word] -join ' | ';$rows.Add("$word`tla`t$(Hash $meaning)`t$meaning")}
$german=[IO.File]::ReadAllLines((Join-Path $root 'german.txt'))
foreach($word in $german){$rows.Add("$word`tde`t$(Hash $word)`t$word")}
$sorted=@($rows.ToArray() | Sort-Object -CaseSensitive)
$mid=[int][Math]::Ceiling($sorted.Count/2)
[IO.File]::WriteAllLines((Join-Path $langRoot 'review-input-1.tsv'),$sorted[0..($mid-1)],[Text.UTF8Encoding]::new($false))
[IO.File]::WriteAllLines((Join-Path $langRoot 'review-input-2.tsv'),$sorted[$mid..($sorted.Count-1)],[Text.UTF8Encoding]::new($false))
"EN=$($en.Count) LA=$($eligible.Count) DE=$($german.Count) total=$($sorted.Count) half=$mid"
