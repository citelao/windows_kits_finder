#!/usr/bin/env pwsh
# Script to test Chocolatey package locally

param(
    [string]$Version = "0.1.1",
    [switch]$Install,
    [switch]$Uninstall
)

$packagePath = "chocolatey"
$nupkgPattern = "winky.*.nupkg"

if ($Install) {
    Write-Host "🧪 Installing winky locally for testing..."
    
    # Build package first
    Push-Location $packagePath
    try {
        choco pack winky.nuspec
        $nupkgFile = Get-ChildItem -Filter $nupkgPattern | Select-Object -First 1
        
        if ($nupkgFile) {
            Write-Host "📦 Installing from: $($nupkgFile.Name)"
            choco install $nupkgFile.Name --source . --force -y
        } else {
            Write-Error "No .nupkg file found"
        }
    } finally {
        Pop-Location
    }
} elseif ($Uninstall) {
    Write-Host "🗑️ Uninstalling winky..."
    choco uninstall winky -y
} else {
    Write-Host "🔨 Building Chocolatey package..."
    
    Push-Location $packagePath
    try {
        # Clean up old packages
        Remove-Item -Path $nupkgPattern -Force -ErrorAction SilentlyContinue
        
        # Build package
        choco pack winky.nuspec
        
        $nupkgFile = Get-ChildItem -Filter $nupkgPattern | Select-Object -First 1
        if ($nupkgFile) {
            Write-Host "✅ Package built: $($nupkgFile.Name)"
            Write-Host ""
            Write-Host "To test locally:"
            Write-Host "  .\test-chocolatey.ps1 -Install"
            Write-Host ""
            Write-Host "To clean up:"
            Write-Host "  .\test-chocolatey.ps1 -Uninstall"
        }
    } finally {
        Pop-Location
    }
}