param([string]$PathsFile)
$ErrorActionPreference='Stop'
$paths=@(Get-Content -LiteralPath $PathsFile -Raw | ConvertFrom-Json)
& git -c safe.directory=D:/game add -- @paths
if ($LASTEXITCODE -ne 0) { throw 'git add failed' }
& git -c safe.directory=D:/game diff --cached --stat -- @paths
if ($LASTEXITCODE -ne 0) { throw 'staged review failed' }
& git -c safe.directory=D:/game commit --only -m 'feat: recover small foundation libraries and validate text GPU pipeline' -- @paths
if ($LASTEXITCODE -ne 0) { throw 'git commit failed' }
& git -c safe.directory=D:/game push origin main
if ($LASTEXITCODE -ne 0) { throw 'git push failed' }
& git -c safe.directory=D:/game rev-parse HEAD
