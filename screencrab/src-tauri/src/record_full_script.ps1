param(
    [string]$filename,
    [int]$timer,
    [int]$audio,
    [int]$openfile
)

# Set up recording parameters
$video_option = "gdigrab"
$video_input = "desktop"
$audio_option = if ($audio -eq 1) { "dshow" } else { "an" }
$audio_input = if ($audio -eq 1) { "audio=Microphone" } else { "" } # Change 'Microphone' to the correct device name if different

# Start the recording
& ffmpeg -y -f $video_option -i $video_input -f $audio_option -i $audio_input -t $timer $filename

# Open the file if required
if ($openfile -eq 1) {
    Start-Process $filename
}
