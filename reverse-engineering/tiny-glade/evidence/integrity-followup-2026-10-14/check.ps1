param([ValidateSet('clippy', 'build')][string] $Action)
$ErrorActionPreference = 'Stop'
$msvcRoot = 'C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC/14.38.33130'
$sdkRoot = 'C:/Program Files (x86)/Windows Kits/10'
$env:PATH = "$msvcRoot/bin/Hostx64/x64;$sdkRoot/bin/10.0.22621.0/x64;" + $env:PATH
$env:LIB = "$msvcRoot/lib/x64;$sdkRoot/Lib/10.0.22621.0/um/x64;$sdkRoot/Lib/10.0.22621.0/ucrt/x64"
$manifestPath = Join-Path $PSScriptRoot 'same-source/Cargo.toml'
$cargoArgs = @($Action, '--offline', '--locked', '--manifest-path', $manifestPath)
if ($Action -eq 'clippy') { $cargoArgs += @('--all-targets', '--', '-D', 'warnings') }
else { $cargoArgs += @('--bin', 'fixture-driver', '--example', 'inspect_package') }
$rawOutput = & 'C:/Users/liyu/.cargo/bin/cargo.exe' @cargoArgs 2>&1
$exitCode = $LASTEXITCODE
$rawOutput | Set-Content -LiteralPath (Join-Path $PSScriptRoot "$Action.log") -Encoding utf8
$rawOutput
@{ action = $Action; exit_code = $exitCode; manifest = $manifestPath; current_source = 'D:/game/tiny-glade/crates/tg-package-integrity/src/lib.rs' } |
    ConvertTo-Json | Set-Content -LiteralPath (Join-Path $PSScriptRoot "$Action.json") -Encoding utf8
exit $exitCode
