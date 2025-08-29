Write-Output "Testing Python Plugin System for Rest Reminder"
Write-Output "================================================="

Write-Output ""
Write-Output "Plugin directory contents:"
Set-Location ..
Get-ChildItem -Path .\plugins\

Write-Output ""
Write-Output "Python plugins found:"
Get-ChildItem -Path .\plugins\ -Filter *.py | ForEach-Object { $_.BaseName }

Write-Output ""
Write-Output "Starting Rest Reminder with plugin system..."
Write-Output "   (Will monitor for 'NonExistentApp' - should show plugin initialization)"
Write-Output "   Press Ctrl+C to stop after seeing the plugin output"
Write-Output ""

# Run Rest Reminder
& .\target\debug\rest-reminder.exe rest -t 10 -a NonExistentApp