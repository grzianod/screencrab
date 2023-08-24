Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

# Make the process DPI aware to ensure the screenshot captures the entire screen
Add-Type -TypeDefinition @"
    using System.Runtime.InteropServices;

    public class DPI {
        [DllImport("user32.dll")]
        public static extern bool SetProcessDPIAware();
    }
"@ 

[DPI]::SetProcessDPIAware()

$Screen = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$Width = $Screen.Width
$Height = $Screen.Height

$Bitmap = New-Object System.Drawing.Bitmap $Width, $Height
$Graphics = [System.Drawing.Graphics]::FromImage($Bitmap)
$Graphics.CopyFromScreen(0, 0, 0, 0, $Screen.Size)
$Bitmap.Save('C:\Users\Antonio\Downloads\screenshot.png', [System.Drawing.Imaging.ImageFormat]::Png)
