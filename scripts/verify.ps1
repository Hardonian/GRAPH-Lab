# verify.ps1 — single-command verification entry point for Windows (pwsh).
# Mirrors scripts/verify.sh with Windows environment configuration.
$ErrorActionPreference = "Continue"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$repoDir = Split-Path -Parent $scriptDir
Set-Location $repoDir

$FAIL = 0
function Run-Gate($name, [scriptblock]$action) {
    Write-Host "== $name ==" -ForegroundColor Cyan
    try {
        & $action
        if ($LASTEXITCODE -ne 0) {
            Write-Host "FAIL($LASTEXITCODE): $name" -ForegroundColor Red
            $script:FAIL = 1
        } else {
            Write-Host "PASS: $name" -ForegroundColor Green
        }
    } catch {
        Write-Host "FAIL(exception): $name : $_" -ForegroundColor Red
        $script:FAIL = 1
    }
}

# Ensure Lean / elan is on PATH
$elanBin = Join-Path $env:USERPROFILE ".elan\bin"
if ((Test-Path $elanBin) -and ($env:PATH -notlike "*$elanBin*")) {
    $env:PATH = "$elanBin;$env:PATH"
}

# Ensure libclang and shim headers are configured for Windows builds of nauty
$shimHeader = (Join-Path $repoDir "scripts\nauty_windows_shim.h").Replace("\", "/")
$llvmInclude = "C:/Users/scott/AppData/Local/Microsoft/WinGet/Packages/MartinStorsjo.LLVM-MinGW.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe/llvm-mingw-20260616-ucrt-x86_64/lib/clang/22/include"
$clangNative = "C:\Users\scott\AppData\Local\Programs\Python\Python312\Lib\site-packages\clang\native"
if (Test-Path $clangNative) {
    $env:LIBCLANG_PATH = $clangNative
}
$env:CFLAGS = "-include $shimHeader"
$env:BINDGEN_EXTRA_CLANG_ARGS = "-include $shimHeader -I $llvmInclude"

$target = "x86_64-pc-windows-gnullvm"

Run-Gate "fmt" { cargo fmt --all -- --check }
Run-Gate "clippy" { cargo clippy --workspace --all-targets --target $target -- -D warnings }
Run-Gate "test" { cargo test --workspace --all-targets --target $target }
Run-Gate "release" { cargo build --workspace --release --target $target }

# Copy release binary to standard target/release path
$releaseBin = Join-Path $repoDir "target\$target\release\graphlab.exe"
$destDir = Join-Path $repoDir "target\release"
if (Test-Path $releaseBin) {
    New-Item -ItemType Directory -Force -Path $destDir | Out-Null
    Copy-Item $releaseBin (Join-Path $destDir "graphlab.exe") -Force
}

Run-Gate "cli-doctor" { .\target\release\graphlab.exe doctor --json }
Run-Gate "py-ruff" { uv run ruff check python tests }
Run-Gate "py-mypy" { uv run mypy python/graphlab }
Run-Gate "py-pytest" { uv run pytest -q }

if (Test-Path (Join-Path $repoDir "lean")) {
    Run-Gate "lean-audit" { python scripts\lean_audit.py --run }
} else {
    Write-Host "== lean =="
    Write-Host "SKIP: lean/ not present"
}

Write-Host ""
if ($FAIL -eq 0) {
    Write-Host "verify: ALL GATES PASS" -ForegroundColor Green
} else {
    Write-Host "verify: FAILURES PRESENT" -ForegroundColor Red
}
exit $FAIL
