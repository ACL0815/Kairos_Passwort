# Replays the separately assessed semantic acceptance sample, without a build.
# A passing result only covers this sample, never the whole vocabulary.
$ErrorActionPreference = 'Stop'
$root = Join-Path $PSScriptRoot '../data/languages'
$lists = @{
    en = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    la = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
}
foreach ($entry in @(@('en', 'english'), @('la', 'latin'))) {
    foreach ($word in [IO.File]::ReadAllLines((Join-Path $root "$($entry[1]).txt"))) {
        [void]$lists[$entry[0]].Add($word)
    }
}
$rows = [IO.File]::ReadAllLines((Join-Path $root 'own-final-sample-v2.tsv'))
if ($rows.Count -ne 200) { throw 'Expected 200 independently selected sample rows' }
$found = 0
foreach ($row in $rows) {
    $fields = $row -split "`t"
    if ($fields.Count -ne 7 -or $fields[2] -cnotin @('en', 'la') -or $fields[4] -cnotin @('allow', 'block')) {
        throw 'Invalid sample assessment'
    }
    if ($fields[4] -ceq 'block' -and $lists[$fields[2]].Contains($fields[1])) {
        "$($fields[2])/$($fields[1]): $($fields[5]); $($fields[6])"
        $found++
    }
}
"semantic_sample_findings=$found"
exit [int]($found -gt 0)
