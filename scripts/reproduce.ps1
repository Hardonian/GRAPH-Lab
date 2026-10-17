# reproduce.ps1 <campaign-name> — replay a recorded campaign deterministically.
# Runs the campaign, re-verifies every witness independently, and compares
# witness counts against results/campaigns/<name>.expected (if present).
param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$Campaign
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$repoDir = Split-Path -Parent $scriptDir
Set-Location $repoDir

$toml = Join-Path $repoDir "campaigns\$Campaign.toml"
if (-not (Test-Path $toml)) {
    Write-Error "reproduce: no such campaign: $toml"
    exit 2
}

$bin = Join-Path $repoDir "target\release\graphlab.exe"
if (-not (Test-Path $bin)) {
    Write-Error "reproduce: build first (cargo build --release)"
    exit 2
}

$tmpDir = Join-Path $repoDir "results\tmp"
New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null
$out = Join-Path $tmpDir "repro-$Campaign-$([System.Guid]::NewGuid().ToString('N').Substring(0,8)).jsonl"

Write-Host "[reproduce] running $toml"
& $bin search --campaign $toml --output $out
if ($LASTEXITCODE -ne 0) {
    Write-Error "[reproduce] search campaign failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

$fail = 0
$n_wit = 0
$lines = Get-Content $out
foreach ($line in $lines) {
    if ($line -like '*"rejected"*' -or $line -like '*"manifest"*') {
        continue
    }
    $n_wit++
    $oneWitness = Join-Path $tmpDir "one-witness.jsonl"
    $line | Set-Content -Path $oneWitness -Encoding utf8
    & $bin verify --witness $oneWitness | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Write-Error "[reproduce] WITNESS $n_wit FAILED INDEPENDENT VERIFY"
        $fail = 1
    }
}

Write-Host "[reproduce] witnesses: $n_wit, independent verify failures: $fail"

$exp = Join-Path $repoDir "results\campaigns\$Campaign.expected"
if (Test-Path $exp) {
    $expected = (Get-Content $exp).Trim()
    $want = ($lines | Where-Object { $_ -match '^\{"schema"' }).Count
    if ("$want" -ne "$expected") {
        Write-Error "[reproduce] MISMATCH: got $want witnesses, expected $expected"
        exit 4
    }
    Write-Host "[reproduce] witness count matches recorded expectation ($expected)"
}

Remove-Item $out -Force -ErrorAction SilentlyContinue
Remove-Item (Join-Path $tmpDir "one-witness.jsonl") -Force -ErrorAction SilentlyContinue

if ($fail -eq 0) {
    Write-Host "[reproduce] OK" -ForegroundColor Green
} else {
    Write-Host "[reproduce] FAILED" -ForegroundColor Red
    exit 3
}
