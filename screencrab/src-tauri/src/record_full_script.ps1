param(
    [string]$filename,
    [int]$timer,
    [string]$audio,
    [string]$pointer
)

# ffmpeg -list_devices true -f dshow -i dummy
# Command to get current I/O devices

# powershell -ExecutionPolicy Bypass -File C:\Users\Antonio\Downloads\screencrab\screencrab\src-tauri\src\record_full_script.ps1 -filename "C:\Users\Antonio\Downloads\test.mp4" -timer 10 -audio "true" -openfile "true"
# Command by which script runs

# Set up recording parameters
$video_option = "gdigrab"
$video_input = "desktop"
$audio = [System.Boolean]::Parse($audio)


# Check if audio needs to be recorded
if ($audio) {
    $audio_option = "dshow"
    $audio_input = "audio=Microphone (High Definition Audio Device)" # Change 'Microphone' to the correct device name if different
} else {
    $audio_option = "an"
    $audio_input = $null
}

# Construct the ffmpeg command
$ffmpeg_cmd = @("-y", "-f", $video_option, "-i", $video_input, $filename)

# Add audio options if needed
if ($audio) {
    $ffmpeg_cmd += "-f", $audio_option, "-i", $audio_input
}

Start-Sleep -Seconds $timer

# Start the recording
ffmpeg $ffmpeg_cmd
