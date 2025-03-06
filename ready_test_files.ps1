# this script has not yet been verified to Normal operation.
Write-Output "this script has not yet been verified to Normal operation."

Remove-Item -Path ".\assets_for_test\assets" -Recurse -Force -ErrorAction SilentlyContinue
Write-Output "removed old assets"

New-Item -Path ".\assets_for_test\assets" -ItemType Directory

Copy-Item -Path ".\assets_for_test\source\*" -Destination ".\assets_for_test\assets\" -Recurse
Write-Output "created new assets"
Write-Output "------------------------------"