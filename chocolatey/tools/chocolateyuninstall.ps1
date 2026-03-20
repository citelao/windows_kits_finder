$ErrorActionPreference = 'Stop'

$packageName = 'winky'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove the exe from tools directory
$exePath = Join-Path $toolsDir 'winky.exe'
if (Test-Path $exePath) {
    Remove-Item $exePath -Force
}

Write-Host "winky has been uninstalled."