# Stops autobuild with cargo and tailwind

Get-Job -Name amx-cw | Remove-Job -Force
Get-Job -Name amx-tw | Remove-Job -Force
