param(
    [ValidateSet('check','test','clippy','metadata')]
    [string] $Action,
    [string] $Packages
)
$ErrorActionPreference = 'Stop'
$msvcRoot = 'C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC/14.38.33130'
$sdkRoot = 'C:/Program Files (x86)/Windows Kits/10'
$env:PATH = "$msvcRoot/bin/Hostx64/x64;$sdkRoot/bin/10.0.22621.0/x64;" + $env:PATH
$env:LIB = "$msvcRoot/lib/x64;$sdkRoot/Lib/10.0.22621.0/um/x64;$sdkRoot/Lib/10.0.22621.0/ucrt/x64"
$env:INCLUDE = "$msvcRoot/include;$sdkRoot/Include/10.0.22621.0/ucrt;$sdkRoot/Include/10.0.22621.0/um;$sdkRoot/Include/10.0.22621.0/shared"
$manifestPath = 'D:/game/reverse-engineering/tiny-glade/evidence/small-libs-development-2026-10-13/tg-package-integrity/same-source-release/Cargo.toml'
$cargoArguments = @($Action, '--offline', '--locked', '--manifest-path', $manifestPath)
if ($Action -eq 'metadata') {
    $cargoArguments += @('--format-version', '1', '--no-deps')
} else {
    foreach ($packageName in ($Packages -split ",")) { $cargoArguments += @('-p', $packageName) }
    if ($Action -eq 'clippy') { $cargoArguments += @('--all-targets', '--', '-D', 'warnings') }
}
& 'C:/Users/liyu/.cargo/bin/cargo.exe' @cargoArguments
exit $LASTEXITCODE
