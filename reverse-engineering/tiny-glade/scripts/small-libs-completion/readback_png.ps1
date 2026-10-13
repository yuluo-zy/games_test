param([string]$InputPath,[string]$OutputPath,[int]$Width,[int]$Height)
$ErrorActionPreference='Stop'
Add-Type -AssemblyName System.Drawing
$pixelBytes=[IO.File]::ReadAllBytes($InputPath)
if ($pixelBytes.Length -ne $Width*$Height*4) { throw 'RGBA byte length differs from extent' }
for ($pixelIndex=0; $pixelIndex -lt $pixelBytes.Length; $pixelIndex+=4) {
    $redByte=$pixelBytes[$pixelIndex]
    $pixelBytes[$pixelIndex]=$pixelBytes[$pixelIndex+2]
    $pixelBytes[$pixelIndex+2]=$redByte
}
$bitmap=New-Object Drawing.Bitmap($Width,$Height,[Drawing.Imaging.PixelFormat]::Format32bppArgb)
try {
    $rect=New-Object Drawing.Rectangle(0,0,$Width,$Height)
    $locked=$bitmap.LockBits($rect,[Drawing.Imaging.ImageLockMode]::WriteOnly,[Drawing.Imaging.PixelFormat]::Format32bppArgb)
    try {
        if ($locked.Stride -ne $Width*4) { throw 'Unexpected bitmap row stride' }
        [Runtime.InteropServices.Marshal]::Copy($pixelBytes,0,$locked.Scan0,$pixelBytes.Length)
    } finally { $bitmap.UnlockBits($locked) }
    $bitmap.Save($OutputPath,[Drawing.Imaging.ImageFormat]::Png)
} finally { $bitmap.Dispose() }
