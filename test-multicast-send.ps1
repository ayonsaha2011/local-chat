# Test script to send multicast packets
# This helps verify if multicast is working on your network

Write-Host "======================================"
Write-Host "  Multicast Sender Test"
Write-Host "======================================"
Write-Host ""
Write-Host "This will send test packets to 239.255.42.99:37842"
Write-Host "Run test-multicast-receive.ps1 on the other device to see if packets arrive"
Write-Host ""

try {
    # Create UDP client
    $udpClient = New-Object System.Net.Sockets.UdpClient
    $udpClient.Client.SetSocketOption([System.Net.Sockets.SocketOptionLevel]::Socket,
                                       [System.Net.Sockets.SocketOptionName]::ReuseAddress,
                                       $true)

    # Get local IP
    $localIP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.IPAddress -like "192.168.*"})[0].IPAddress
    Write-Host "Local IP: $localIP" -ForegroundColor Green

    # Set multicast interface
    $localIPBytes = [System.Net.IPAddress]::Parse($localIP).GetAddressBytes()
    $udpClient.Client.SetSocketOption([System.Net.Sockets.SocketOptionLevel]::IP,
                                       [System.Net.Sockets.SocketOptionName]::MulticastInterface,
                                       [System.BitConverter]::ToInt32($localIPBytes, 0))

    # Set TTL
    $udpClient.Client.SetSocketOption([System.Net.Sockets.SocketOptionLevel]::IP,
                                       [System.Net.Sockets.SocketOptionName]::MulticastTimeToLive,
                                       32)

    $multicastIP = "239.255.42.99"
    $port = 37842
    $endpoint = New-Object System.Net.IPEndPoint([System.Net.IPAddress]::Parse($multicastIP), $port)

    Write-Host ""
    Write-Host "Sending 10 test packets..." -ForegroundColor Cyan

    for ($i = 1; $i -le 10; $i++) {
        $message = "TEST_PACKET_$i from $localIP at $(Get-Date -Format 'HH:mm:ss')"
        $bytes = [System.Text.Encoding]::UTF8.GetBytes($message)

        $udpClient.Send($bytes, $bytes.Length, $endpoint) | Out-Null
        Write-Host "  [$i/10] Sent: $message" -ForegroundColor Yellow
        Start-Sleep -Milliseconds 500
    }

    Write-Host ""
    Write-Host "✅ Sent 10 packets to $multicastIP:$port" -ForegroundColor Green
    Write-Host ""
    Write-Host "Check the receiver device to see if packets arrived."

    $udpClient.Close()
}
catch {
    Write-Host ""
    Write-Host "❌ ERROR: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
}

Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
