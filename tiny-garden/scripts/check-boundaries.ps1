$ErrorActionPreference = 'Stop'
$workspaceRoot = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $workspaceRoot 'Cargo.toml'
$metadataJson = & cargo metadata --manifest-path $manifestPath --format-version 1 --all-features --locked
if ($LASTEXITCODE -ne 0) { throw 'cargo metadata failed' }
$metadata = ($metadataJson -join "`n") | ConvertFrom-Json

$packagesById = @{}
$nodesById = @{}
foreach ($package in $metadata.packages) { $packagesById[$package.id] = $package }
foreach ($node in $metadata.resolve.nodes) { $nodesById[$node.id] = $node }

$allowed = @{
    'garden-geometry' = @()
    'garden-domain' = @('garden-geometry')
    'garden-application' = @('garden-domain', 'garden-geometry')
    'garden-generation' = @('garden-domain', 'garden-geometry')
    'garden-bevy' = @('garden-domain', 'garden-application', 'garden-generation', 'garden-geometry', 'bevy_app', 'bevy_ecs', 'bevy_tasks')
    'garden-presentation' = @('garden-domain', 'garden-application', 'garden-generation', 'garden-geometry', 'garden-bevy', 'bevy', 'serde', 'serde_json')
}

foreach ($name in $allowed.Keys) {
    $package = @($metadata.packages | Where-Object { $_.name -eq $name })
    if ($package.Count -ne 1) { throw "Expected one workspace package: $name" }
    foreach ($dependency in $package[0].dependencies) {
        if ($dependency.name -notin $allowed[$name]) {
            throw "Forbidden direct dependency: $name -> $($dependency.name). Update architecture decision before adding it."
        }
    }

    if ($name -notin @('garden-bevy', 'garden-presentation')) {
        $pendingIds = [System.Collections.Generic.Stack[string]]::new()
        $visitedIds = [System.Collections.Generic.HashSet[string]]::new()
        $pendingIds.Push($package[0].id)
        while ($pendingIds.Count -gt 0) {
            $nodeId = $pendingIds.Pop()
            if (-not $visitedIds.Add($nodeId)) { continue }
            $dependencyName = $packagesById[$nodeId].name
            if ($dependencyName -match '^bevy') { throw "Engine leaked into pure core: $name -> $dependencyName" }
            foreach ($dependencyId in $nodesById[$nodeId].dependencies) { $pendingIds.Push($dependencyId) }
        }
    }
}

foreach ($node in $metadata.resolve.nodes) {
    $name = $packagesById[$node.id].name
    if ($name -in @('bevy_gilrs', 'gilrs', 'gilrs-core')) { throw "Gamepad dependency is out of scope: $name" }
}
Write-Output 'PASS: 6 crate dependency directions; 4 engine-independent cores; no gilrs/gamepad dependency.'
