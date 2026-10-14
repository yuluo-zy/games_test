$ErrorActionPreference='Stop'
$reviewMsvc='C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC/14.38.33130'
$reviewSdk='C:/Program Files (x86)/Windows Kits/10'
$env:PATH="$reviewMsvc/bin/Hostx64/x64;$reviewSdk/bin/10.0.22621.0/x64;"+$env:PATH
$env:LIB="$reviewMsvc/lib/x64;$reviewSdk/Lib/10.0.22621.0/um/x64;$reviewSdk/Lib/10.0.22621.0/ucrt/x64"
$env:INCLUDE="$reviewMsvc/include;$reviewSdk/Include/10.0.22621.0/ucrt;$reviewSdk/Include/10.0.22621.0/um;$reviewSdk/Include/10.0.22621.0/shared"
& 'C:/Users/liyu/.cargo/bin/cargo.exe' run --release --offline --manifest-path (Join-Path $PSScriptRoot 'Cargo.toml')
exit $LASTEXITCODE
