param(
    [string]$filename,
    [int]$timer,
    [string]$audio,
    [string]$pointer,
    [string]$ffmpegPath  # New parameter for ffmpeg binary path
)

# ffmpeg -list_devices true -f dshow -i dummy
# Command to get current I/O devices

# powershell -ExecutionPolicy Bypass -File C:\Users\Antonio\Downloads\screencrab\screencrab\src-tauri\src\record_full_script.ps1 -filename "C:\Users\Antonio\Downloads\test.mp4" -timer 10 -audio "true" -openfile "true"
# Command by which script runs

# Function to list and select an audio device
function Select-AudioDevice {
    # List available devices
    $deviceList = & $ffmpegPath -list_devices true -f dshow -i dummy 2>&1
    Write-Host "Available devices:" -ForegroundColor Green
    Write-Host $deviceList

    # Logic to select an audio device
    # Example: Select the first audio device found
    # Modify this part based on your requirements
    $selectedDevice = $deviceList | Where-Object { $_ -match "audio devices" -and $_ -match "Alternative name" } | Select-Object -First 1
    return $selectedDevice
}

# Set up recording parameters
$video_option = "gdigrab"
$video_input = "desktop"

# Use the provided ffmpeg binary path
$ffmpeg_cmd = @("-y", "-f", $video_option, "-i", $video_input, $filename)

# Check if audio needs to be recorded
if ($audio -eq "True") {
    $audio_device_name = Select-AudioDevice
    $audio_option = "dshow"
    $audio_input = "audio=$audio_device_name"
    $ffmpeg_cmd += "-f", $audio_option, "-i", $audio_input
}

if ($timer -gt 0) {
    Start-Sleep -Seconds $timer
}

# Start the recording with the specified ffmpeg binary
& $ffmpegPath $ffmpeg_cmd
