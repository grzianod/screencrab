param (
    [string]$filename,
    [string]$filetype,
    [int]$timer,
    [string]$pointer,
    [string]$clipboard,
    [string]$openfile
)

# Convert the string values to boolean
$pointerBool = ($pointer -eq "1")
$clipboardBool = ($clipboard -eq "1")
$openfileBool = ($openfile -eq "1")

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

$Screen = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$Width = $Screen.Width
$Height = $Screen.Height

$Bitmap = New-Object System.Drawing.Bitmap $Width, $Height
$Graphics = [System.Drawing.Graphics]::FromImage($Bitmap)
$Graphics.CopyFromScreen(0, 0, 0, 0, $Screen.Size)

# Add the mouse pointer to the screenshot
if ($pointerBool) {
    $cursorBounds = New-Object System.Drawing.Rectangle([System.Windows.Forms.Cursor]::Position, [System.Windows.Forms.Cursors]::Default.Size)
    [System.Windows.Forms.Cursors]::Default.Draw($Graphics, $cursorBounds)
}

if ($filetype -eq "pdf") {
    $tempImage = "$filename-temp.png"
    $Bitmap.Save($tempImage, [System.Drawing.Imaging.ImageFormat]::Png)
    if (-not (Test-Path $tempImage)) {
        Write-Error "Failed to save the temporary image: $tempImage"
        exit 1
    }

    try {
        # Use ImageMagick to set a white background and then convert the image to PDF
        $output = & magick convert $tempImage -background white -alpha remove -alpha off pdf:$filename 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Error "ImageMagick failed with message: $output"
            exit 1
        }
    } catch {
        Write-Error "There was an issue using ImageMagick: $_. Exception details: $($_.Exception.Message)"
        exit 1
    } finally {
        if (Test-Path $tempImage) {
            Remove-Item $tempImage
        }
    }
} else {
    $imageFormat = switch ($filetype) {
        "png" { [System.Drawing.Imaging.ImageFormat]::Png }
        "jpeg" { [System.Drawing.Imaging.ImageFormat]::Jpeg }
        "bmp" { [System.Drawing.Imaging.ImageFormat]::Bmp }
        "gif" { [System.Drawing.Imaging.ImageFormat]::Gif }
        "tiff" { [System.Drawing.Imaging.ImageFormat]::Tiff }
        default { [System.Drawing.Imaging.ImageFormat]::Png }
    }

    $Bitmap.Save($filename, $imageFormat)
}

# Save the screenshot to clipboard
if ($clipboardBool) {
    [System.Windows.Forms.Clipboard]::SetImage($Bitmap)
}

$Graphics.Dispose()
$Bitmap.Dispose()