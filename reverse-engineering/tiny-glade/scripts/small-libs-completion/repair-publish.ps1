$ErrorActionPreference='Stop'
$repairPaths=@('reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/.gitignore','reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/AGENTS.md')
& git -c safe.directory=D:/game -c core.protectNTFS=false add -- @repairPaths
if($LASTEXITCODE -ne 0){throw 'repair add failed'}
& git -c safe.directory=D:/game -c core.protectNTFS=false commit --only -m 'fix: preserve rhapsody workspace instructions and ignore rules' -- @repairPaths
if($LASTEXITCODE -ne 0){throw 'repair commit failed'}
& git -c safe.directory=D:/game push origin main
if($LASTEXITCODE -ne 0){throw 'repair push failed'}
& git -c safe.directory=D:/game rev-parse HEAD
