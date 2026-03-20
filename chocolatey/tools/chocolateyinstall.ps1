$ErrorActionPreference = 'Stop'

$packageName = 'winky'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/citelao/winky/releases/download/v0.1.1/winky.exe'

$packageArgs = @{
  packageName   = $packageName
  unzipLocation = $toolsDir
  fileType      = 'exe'
  url64bit      = $url64
  softwareName  = 'winky*'
  checksum64    = ''  # This will be filled automatically by the workflow
  checksumType64= 'sha256'
  
  # For portable exe, we just download it directly
  validExitCodes= @(0)
}

# Download and place the exe in the tools directory
Get-ChocolateyWebFile @packageArgs

# The exe should be named winky.exe for proper PATH handling
$exePath = Join-Path $toolsDir 'winky.exe'
if (-not (Test-Path $exePath)) {
    # If downloaded with different name, rename it
    $downloadedFile = Get-ChildItem $toolsDir -Filter "*.exe" | Select-Object -First 1
    if ($downloadedFile) {
        Rename-Item $downloadedFile.FullName -NewName 'winky.exe'
    }
}