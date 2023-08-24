param (
    [string]$filename,
    [string]$filetype,
    [string]$area, # Format: "x1,y1,x2,y2"
    [int]$timer,
    [string]$pointer,
    [string]$clipboard,
    [string]$openfile
)

# Convert the string values to boolean
$pointerBool = ($pointer -eq "1")
$clipboardBool = ($clipboard -eq "1")
$openfileBool = ($openfile -eq "1")

# Split the area into coordinates
$coordinates = $area -split ','
$x1 = [int]$coordinates[0]
$y1 = [int]$coordinates[1]
$x2 = [int]$coordinates[2]
$y2 = [int]$coordinates[3]

# Width and Height of custom capture area
$Width = $x2 - $x1
$Height = $y2 - $y1

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

Add-Type -TypeDefinition @"
    using System.Runtime.InteropServices;

    public class DPI {
        [DllImport("user32.dll")]
        public static extern bool SetProcessDPIAware();
    }
"@

[DPI]::SetProcessDPIAware()

if ($timer -gt 0) {
    Start-Sleep -Seconds $timer
}

# Create a bitmap only for the custom capture area
$Bitmap = New-Object System.Drawing.Bitmap $Width, $Height
$Graphics = [System.Drawing.Graphics]::FromImage($Bitmap)

# Capture the custom area
$Graphics.CopyFromScreen($x1, $y1, 0, 0, $($Bitmap.Size))

# Add the mouse pointer to the screenshot
if ($pointerBool) {
    $cursorBounds = New-Object System.Drawing.Rectangle([System.Windows.Forms.Cursor]::Position, [System.Windows.Forms.Cursors]::Default.Size)
    [System.Windows.Forms.Cursors]::Default.Draw($Graphics, $cursorBounds)
}

$imageFormat = switch ($filetype) {
    "png" { [System.Drawing.Imaging.ImageFormat]::Png }
    "jpeg" { [System.Drawing.Imaging.ImageFormat]::Jpeg }
    "bmp" { [System.Drawing.Imaging.ImageFormat]::Bmp }
    "gif" { [System.Drawing.Imaging.ImageFormat]::Gif }
    "tiff" { [System.Drawing.Imaging.ImageFormat]::Tiff }
    default { [System.Drawing.Imaging.ImageFormat]::Png }
}

$Bitmap.Save($filename, $imageFormat)

if ($openfileBool) {
    Start-Process $filename
}

# Save the screenshot to clipboard
if ($clipboardBool) {
    [System.Windows.Forms.Clipboard]::SetImage($Bitmap)
}

$Graphics.Dispose()
$Bitmap.Dispose()
