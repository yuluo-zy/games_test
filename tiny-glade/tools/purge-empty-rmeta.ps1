# Remove zero-length .rmeta files from the cargo target directory.
#
# A rustc process that is killed mid-compile (Ctrl-C, cargo aborting the other units
# after the first failure, a stopped agent) leaves an empty .rmeta behind. Windows
# cannot mmap a zero-length file, so a later dependent compile fails with
# "E0786 found invalid metadata files ... failed to mmap rmeta metadata". The files
# are pure build cache; deleting them makes cargo rebuild exactly those units.
#
# Usage: pwsh tools/purge-empty-rmeta.ps1 [target-dir]   (default: <workspace>/target)
param([string]$Target = (Join-Path $PSScriptRoot '..\target'))
if (-not (Test-Path -LiteralPath $Target)) { Write-Output "no target dir: $Target"; exit 0 }
$empty = Get-ChildItem -LiteralPath $Target -Recurse -Filter *.rmeta -File -ErrorAction SilentlyContinue |
    Where-Object { $_.Length -eq 0 }
$empty | Remove-Item -Force
Write-Output "removed $(@($empty).Count) empty .rmeta file(s) under $Target"
