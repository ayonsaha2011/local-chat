@echo off
echo ========================================
echo  LAN Chat - Windows Firewall Fix
echo ========================================
echo.
echo This will add firewall rules to allow:
echo   - UDP Port 37842 (Discovery)
echo   - TCP Port 37843 (Messaging)
echo   - TCP Port 37844 (File Transfer)
echo.
echo IMPORTANT: Run this as Administrator!
echo.
pause

echo.
echo Adding firewall rules...
echo.

netsh advfirewall firewall add rule name="LAN Chat Discovery (UDP In)" dir=in action=allow protocol=UDP localport=37842

netsh advfirewall firewall add rule name="LAN Chat Discovery (UDP Out)" dir=out action=allow protocol=UDP localport=37842

netsh advfirewall firewall add rule name="LAN Chat Messaging (TCP In)" dir=in action=allow protocol=TCP localport=37843

netsh advfirewall firewall add rule name="LAN Chat Transfer (TCP In)" dir=in action=allow protocol=TCP localport=37844

echo.
echo ========================================
echo  Firewall rules added successfully!
echo ========================================
echo.
echo You can verify with:
echo   netsh advfirewall firewall show rule name=all ^| findstr "LAN Chat"
echo.
pause
