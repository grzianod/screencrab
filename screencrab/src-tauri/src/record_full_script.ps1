param(
    [string]$filename,
    [int]$timer,
    [string]$audio,
    [string]$openfile
)

# Capture the output of ffmpeg listing devices
$ffmpegOutput = & ffmpeg -list_devices true -f dshow -i dummy 2>&1

# Parse the ffmpeg output to extract the microphone's name
$microphoneName = $ffmpegOutput | Where-Object { $_ -match '"(.+)" \(audio\)' } | ForEach-Object { $matches[1] }

# If a microphone name is found, use it; otherwise, use a default one
if (-not [string]::IsNullOrWhiteSpace($microphoneName)) {
    $audio_input = "audio=$microphoneName"
} else {
    $audio_input = "audio=Microphone (High Definition Audio Device)"  # Default
}

# ffmpeg -list_devices true -f dshow -i dummy
# Command to get current I/O devices

# powershell -ExecutionPolicy Bypass -File C:\Users\Antonio\Downloads\screencrab\screencrab\src-tauri\src\record_full_script.ps1 -filename "C:\Users\Antonio\Downloads\test.mp4" -timer 10 -audio "true" -openfile "true"
# Command by which script runs

# Set up recording parameters
$video_option = "gdigrab"
$video_input = "desktop"
$audio = [System.Boolean]::Parse($audio)
$openfile = [System.Boolean]::Parse($openfile)


# Check if audio needs to be recorded
if ($audio) {
    $audio_option = "dshow"
} else {
    $audio_option = "an"
    $audio_input = $null
}

# Construct the ffmpeg command
$ffmpeg_cmd = @("-y", "-f", $video_option, "-i", $video_input, "-t", $timer, $filename)

# Add audio options if needed
if ($audio) {
    $ffmpeg_cmd += "-f", $audio_option, "-i", $audio_input
}

# Start the recording
ffmpeg $ffmpeg_cmd
