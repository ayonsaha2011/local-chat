# Test multicast reception on Windows
Write-Host "==================================="
Write-Host " Testing Multicast Reception"
Write-Host "==================================="
Write-Host ""
Write-Host "Listening for multicast packets on 239.255.42.99:37842"
Write-Host "Press Ctrl+C to stop"
Write-Host ""

$multicastIP = "239.255.42.99"
$port = 37842

try {
    $udpClient = New-Object System.Net.Sockets.UdpClient
    $udpClient.Client.SetSocketOption([System.Net.Sockets.SocketOptionLevel]::Socket, [System.Net.Sockets.SocketOptionName]::ReuseAddress, $true)
    
    $localEP = New-Object System.Net.IPEndPoint([System.Net.IPAddress]::Any, $port)
    $udpClient.Client.Bind($localEP)
    
    $multicastAddress = [System.Net.IPAddress]::Parse($multicastIP)
    $udpClient.JoinMulticastGroup($multicastAddress)
    
    Write-Host "✅ Successfully joined multicast group" -ForegroundColor Green
    Write-Host "Waiting for packets..." -ForegroundColor Yellow
    Write-Host ""
    
    $count = 0
    while ($true) {
        if ($udpClient.Available -gt 0) {
            $remoteEP = New-Object System.Net.IPEndPoint([System.Net.IPAddress]::Any, 0)
            $data = $udpClient.Receive([ref]$remoteEP)
            $count++
            
            $timestamp = Get-Date -Format "HH:mm:ss"
            Write-Host "[$timestamp] Packet #$count from $($remoteEP.Address):$($remoteEP.Port) - $($data.Length) bytes" -ForegroundColor Green
            
            # Try to decode first 100 bytes as text
            $text = [System.Text.Encoding]::UTF8.GetString($data[0..[Math]::Min(99, $data.Length-1)])
            Write-Host "  Preview: $text" -ForegroundColor Gray
        }
        Start-Sleep -Milliseconds 100
    }
}
catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
}
finally {
    if ($udpClient) {
        $udpClient.Close()
    }
}
