@echo off
echo ========================================
echo  Fix Network Profile for LAN Chat
echo ========================================
echo.
echo Problem: Your network is set to "Public"
echo Windows blocks multicast on Public networks
echo.
echo This will change it to "Private" to allow:
echo   - Multicast discovery
echo   - Local network communication
echo.
echo IMPORTANT: Run this as Administrator!
echo.
pause

echo.
echo Current network profile:
echo.
powershell -Command "Get-NetConnectionProfile | Format-Table Name, NetworkCategory, InterfaceAlias"

echo.
echo Changing network category to Private...
echo.

powershell -Command "Get-NetConnectionProfile | Set-NetConnectionProfile -NetworkCategory Private"

echo.
echo ========================================
echo  Network profile updated!
echo ========================================
echo.
echo New network profile:
echo.
powershell -Command "Get-NetConnectionProfile | Format-Table Name, NetworkCategory, InterfaceAlias"

echo.
echo Next steps:
echo   1. Restart the LAN Chat app
echo   2. Devices should now be able to see each other
echo.
echo If still not working:
echo   - Check that Mac firewall is disabled
echo   - Check router AP Isolation is OFF
echo   - See CRITICAL_FIX.md for more troubleshooting
echo.
pause
