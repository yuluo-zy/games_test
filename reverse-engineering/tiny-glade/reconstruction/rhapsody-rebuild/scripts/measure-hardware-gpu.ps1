param([ValidateRange(1, 100)][int]$Runs = 3)
$ErrorActionPreference = 'Stop'

function Invoke-Cargo {
    param([string[]]$Arguments, [string]$Log)
    # 独立重定向 stdout/stderr，避免 Windows PowerShell 把正常编译进度当作异常。
    $stdoutFile = $Log + '.stdout.tmp'
    $stderrFile = $Log + '.stderr.tmp'
    $process = Start-Process -FilePath (Get-Command cargo).Source -ArgumentList $Arguments -WindowStyle Hidden -Wait -PassThru -RedirectStandardOutput $stdoutFile -RedirectStandardError $stderrFile
    $stdout = Get-Content -Raw -Encoding UTF8 -LiteralPath $stdoutFile
    $stderr = Get-Content -Raw -Encoding UTF8 -LiteralPath $stderrFile
    ($stderr + [Environment]::NewLine + $stdout) | Set-Content -Encoding UTF8 -LiteralPath $Log
    Remove-Item -LiteralPath $stdoutFile,$stderrFile
    return $process.ExitCode
}
$OutputEncoding = [System.Text.UTF8Encoding]::new()
[Console]::OutputEncoding = $OutputEncoding
Set-Location (Split-Path $PSScriptRoot -Parent)
$stamp = Get-Date -Format 'yyyy-MM-dd-HHmmss'
$destination = Join-Path (Get-Location) "evidence/hardware-gpu-$stamp"
New-Item -ItemType Directory -Path $destination | Out-Null
$envReport = @{}
Get-ChildItem Env: | Where-Object Name -Match '^(VK_|VULKAN|RHAPSODY_|CARGO_|RUST)' | ForEach-Object { $envReport[$_.Name] = $_.Value }
$metadata = [ordered]@{
    started_utc = (Get-Date).ToUniversalTime().ToString('o')
    gpu = @(Get-CimInstance Win32_VideoController | Select-Object Name, DriverVersion, AdapterRAM)
    cpu = @(Get-CimInstance Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors)
    os = @(Get-CimInstance Win32_OperatingSystem | Select-Object Caption, Version)
    environment = $envReport
    rustc = (& rustc -Vv | Out-String).Trim()
    cargo = (& cargo -V | Out-String).Trim()
    runs = $Runs
    warmup_runs = 1
}
$metadata | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 (Join-Path $destination 'host.json')
Get-ChildItem src,benches,examples,tests -Recurse -File | Where-Object Extension -EQ '.rs' | ForEach-Object {
    [ordered]@{path=$_.FullName.Substring((Get-Location).Path.Length+1); sha256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLower()}
} | ConvertTo-Json | Set-Content -Encoding UTF8 (Join-Path $destination 'source-hashes.json')
Get-FileHash Cargo.toml,Cargo.lock,../../tools/render-passes/self_test/blur_mod0.spv -Algorithm SHA256 | ConvertTo-Json | Set-Content -Encoding UTF8 (Join-Path $destination 'input-hashes.json')
$code = Invoke-Cargo -Arguments @('run','--locked','--release','--example','device_inventory') -Log (Join-Path $destination 'inventory-build.log')
if ($code -ne 0) { throw 'GPU inventory failed' }
$inventory = Get-Content -Encoding UTF8 (Join-Path $destination 'inventory-build.log') | Where-Object { $_.StartsWith('{') }
$inventory | Set-Content -Encoding UTF8 (Join-Path $destination 'device-inventory.json')
$devices = ($inventory | ConvertFrom-Json).devices
if ($devices.Count -ne 1 -or $devices[0].name -match 'llvmpipe|lavapipe|Software|Microsoft Basic') { throw 'Requires one unambiguous hardware Vulkan device' }
for ($i = 0; $i -le $Runs; $i++) {
    Write-Output "Benchmark run $i/$Runs ($($devices[0].name))"
    $previousWrite = if (Test-Path perf/last.json) { (Get-Item perf/last.json).LastWriteTimeUtc } else { $null }
    $code = Invoke-Cargo -Arguments @('bench','--locked','--bench','perf','--','--check') -Log (Join-Path $destination "perf-$i.log")
    if (-not (Test-Path perf/last.json) -or (Get-Item perf/last.json).LastWriteTimeUtc -eq $previousWrite) { throw "Run $i did not produce fresh results" }
    Copy-Item -LiteralPath perf/last.json -Destination (Join-Path $destination "perf-$i.json")
    $report = Get-Content -Raw perf/last.json | ConvertFrom-Json
    if (-not ($report.results.PSObject.Properties.Name -contains 'gpu/frame_loop/passes=8')) { throw 'GPU benchmarks were skipped' }
    if ($code -ne 0) { throw "Performance counters failed; see perf-$i.log" }
}
$correctnessCode = Invoke-Cargo -Arguments @('test','--locked','--release','--test','real_pass_blur','--test','perf_guards','--','--nocapture') -Log (Join-Path $destination 'correctness.log')
$probeCode = Invoke-Cargo -Arguments @('run','--locked','--release','--example','pipeline_probe') -Log (Join-Path $destination 'pipeline-probe.log')
$blurCode = Invoke-Cargo -Arguments @('run','--locked','--release','--example','blur_gpu_perf') -Log (Join-Path $destination 'blur-gpu-perf.log')
$clippyCode = Invoke-Cargo -Arguments @('clippy','--locked','--release','--example','blur_gpu_perf','--','-D','warnings') -Log (Join-Path $destination 'clippy-blur.log')
@{correctness=$correctnessCode; pipeline_probe=$probeCode; blur=$blurCode; clippy=$clippyCode} | ConvertTo-Json | Set-Content -Encoding UTF8 (Join-Path $destination 'exit-codes.json')
if ($blurCode -ne 0) { throw "Blur GPU measurement failed; logs preserved in $destination" }
& python scripts/summarize-hardware-gpu.py $destination
if ($LASTEXITCODE -ne 0) { throw 'Summary failed' }
Write-Output "RESULT_DIRECTORY=$destination"
if ($correctnessCode -ne 0 -or $probeCode -ne 0 -or $clippyCode -ne 0) { throw "Validation failed; performance results retained in $destination" }
