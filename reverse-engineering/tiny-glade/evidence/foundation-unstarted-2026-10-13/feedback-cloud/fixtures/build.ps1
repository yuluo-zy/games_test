$ErrorActionPreference='Stop'
$msvcRoot='C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC/14.38.33130'
$sdkRoot='C:/Program Files (x86)/Windows Kits/10'
$env:PATH="$msvcRoot/bin/Hostx64/x64;$sdkRoot/bin/10.0.22621.0/x64;"+$env:PATH
$env:LIB="$msvcRoot/lib/x64;$sdkRoot/Lib/10.0.22621.0/um/x64;$sdkRoot/Lib/10.0.22621.0/ucrt/x64"
$env:INCLUDE="$msvcRoot/include;$sdkRoot/Include/10.0.22621.0/ucrt;$sdkRoot/Include/10.0.22621.0/um;$sdkRoot/Include/10.0.22621.0/shared"
& "$msvcRoot/bin/Hostx64/x64/cl.exe" /nologo /LD /O2 /Fo:$PSScriptRoot/gfn_runtime.obj $PSScriptRoot/gfn_runtime.c /link /OUT:$PSScriptRoot/GfnFixtureRuntime.dll /IMPLIB:$PSScriptRoot/gfn_runtime.lib
exit $LASTEXITCODE
