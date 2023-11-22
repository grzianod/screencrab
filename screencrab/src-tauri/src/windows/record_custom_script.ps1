param(
    [string]$filename,
    [string]$area,
    [int]$timer,
    [string]$audio,
    [string]$openfile,
    [string]$ffmpegPath  # New parameter for ffmpeg binary path from the full script
)

# Extract the x, y, width, and height from the area string
$areaSplit = $area -split ","
$x = $areaSplit[0]
$y = $areaSplit[1]
$width = $areaSplit[2]
$height = $areaSplit[3]

# Function to list and select an audio device, similar to the full script
function Select-AudioDevice {
    # List available devices using the provided ffmpeg path
    $deviceList = & $ffmpegPath -list_devices true -f dshow -i dummy 2>&1
    Write-Host "Available devices:" -ForegroundColor Green
    Write-Host $deviceList

    # Select the first audio device found
    $selectedDevice = $deviceList | Where-Object { $_ -match "audio devices" -and $_ -match "Alternative name" } | Select-Object -First 1
    return $selectedDevice
}

# Set up recording parameters
$video_option = "gdigrab"
# Modify the $video_input to capture only the selected area
$video_input_options = @("-video_size", "${width}x${height}", "-offset_x", "$x", "-offset_y", "$y")
$video_input = "desktop"

$audio = [System.Boolean]::Parse($audio)
$openfile = [System.Boolean]::Parse($openfile)


# Check if audio needs to be recorded
if ($audio -eq "True") {
    $audio_device_name = Select-AudioDevice
    $audio_option = "dshow"
    $audio_input = "audio=$audio_device_name"
    $ffmpeg_cmd += "-f", $audio_option, "-i", $audio_input
}

# Construct the ffmpeg command
$ffmpeg_cmd = @("-y", "-f", $video_option) + $video_input_options + @("-i", $video_input, $filename)

if ($timer -gt 0) {
    Start-Sleep -Seconds $timer
}

# Start the recording with the specified ffmpeg binary
& $ffmpegPath $ffmpeg_cmd
