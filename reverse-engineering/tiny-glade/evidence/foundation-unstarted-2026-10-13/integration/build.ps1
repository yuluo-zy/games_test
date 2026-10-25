param([ValidateSet('check','test','clippy','metadata','fetch','fmt')][string]$Action='check',[string]$Packages='',[switch]$UpdateLock,[string]$Manifest='D:/game/reverse-engineering/tiny-glade/reconstruction/shared-foundation/Cargo.toml')
$ErrorActionPreference='Stop'
$msvcTaskRoot='C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC/14.38.33130'
$sdkTaskRoot='C:/Program Files (x86)/Windows Kits/10'
$env:PATH="$msvcTaskRoot/bin/Hostx64/x64;$sdkTaskRoot/bin/10.0.22621.0/x64;"+$env:PATH
$env:LIB="$msvcTaskRoot/lib/x64;$sdkTaskRoot/Lib/10.0.22621.0/um/x64;$sdkTaskRoot/Lib/10.0.22621.0/ucrt/x64"
$env:INCLUDE="$msvcTaskRoot/include;$sdkTaskRoot/Include/10.0.22621.0/ucrt;$sdkTaskRoot/Include/10.0.22621.0/um;$sdkTaskRoot/Include/10.0.22621.0/shared"
$taskCargoArgs=@($Action,'--manifest-path',$Manifest)
if($Action -notin @('fmt','fetch')){$taskCargoArgs+=@('--offline')}
if(-not $UpdateLock -and $Action -ne 'fmt'){$taskCargoArgs+=@('--locked')}
if($Action -eq 'metadata'){$taskCargoArgs+=@('--format-version','1','--no-deps')} elseif($Action -eq 'fetch') {} else {foreach($taskPackage in ($Packages -split ',')){if($taskPackage){$taskCargoArgs+=@('-p',$taskPackage)}}}
if($Action -eq 'clippy'){$taskCargoArgs+=@('--all-targets','--','-D','warnings')}
& 'C:/Users/liyu/.cargo/bin/cargo.exe' @taskCargoArgs
exit $LASTEXITCODE
