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

# Import the iTextSharp libraries
Add-Type -TypeDefinition @"
    using System;
    using System.IO;
    using iTextSharp.text;
    using iTextSharp.text.pdf;
"@ -Language CSharp

function SaveImageToPdf($imagePath, $pdfPath) {
    $document = New-Object iTextSharp.text.Document
    $writer = [iTextSharp.text.pdf.PdfWriter]::GetInstance($document, (New-Object System.IO.FileStream($pdfPath, [System.IO.FileMode]::Create)))

    $document.Open()

    $image = [iTextSharp.text.Image]::GetInstance($imagePath)
    $image.ScaleToFit($document.PageSize)
    $image.Alignment = [iTextSharp.text.Image]::MIDDLE_ALIGN
    $document.Add($image)

    $document.Close()
}

if ($filetype -eq "pdf") {
    $tempImage = "$filename-temp.png"
    $Bitmap.Save($tempImage, [System.Drawing.Imaging.ImageFormat]::Png)
    SaveImageToPdf -imagePath $tempImage -pdfPath $filename
    Remove-Item $tempImage
}

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
