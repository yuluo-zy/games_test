param([ValidateSet('all','dialog','integrity','gpu','vulkan','crash')][string]$Case='all',[switch]$Glyphs)
$ErrorActionPreference='Stop'
$probeRoot='D:/game/reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui'
$exeRoot='D:/game/reverse-engineering/tiny-glade/reconstruction/shared-foundation/target/debug/examples'
$packageRoot="$probeRoot/fixtures/package-root"
$cases=@(
@{name='dialog';exe='tg_iced_native_probe';extra=@()},
@{name='integrity';exe='tg_integrity_report_native_probe';extra=@($packageRoot)},
@{name='gpu';exe='tg_gpu_selector_native_probe';extra=@()},
@{name='vulkan';exe='tg_gpu_selector_native_probe';extra=@('config')},
@{name='crash';exe='crash_reporter_gui_native_probe';extra=@($packageRoot)}
)
foreach($probe in $cases){
if($Glyphs -and $probe.name -notin @("dialog","integrity","crash")){continue}
if($Case -ne 'all' -and $Case -ne $probe.name){continue}
$arguments=@("$probeRoot/$($probe.name).rgba")+$probe.extra
$process=Start-Process -FilePath "$exeRoot/$($probe.exe).exe" -ArgumentList $arguments -WorkingDirectory 'D:/game/ljxsj_92385/Tiny Glade' -WindowStyle Hidden -RedirectStandardOutput "$probeRoot/native-$($probe.name).stdout.log" -RedirectStandardError "$probeRoot/native-$($probe.name).stderr.log" -PassThru
$null=$process.Handle
if(-not $process.WaitForExit(15000)){$process.Kill();throw "Probe timeout: $($probe.name) PID $($process.Id)"}
$process.Refresh()
Write-Output "$($probe.name): PID $($process.Id) exit $($process.ExitCode)"
if($process.ExitCode -ne 0){Get-Content -LiteralPath "$probeRoot/native-$($probe.name).stderr.log";throw 'Native capture failed'}
}
