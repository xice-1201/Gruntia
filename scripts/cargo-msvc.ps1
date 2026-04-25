param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $CargoArgs
)

$ErrorActionPreference = "Stop"

$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) {
    throw "vswhere.exe was not found. Install Visual Studio Build Tools first."
}

$vsPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $vsPath) {
    throw "Visual Studio C++ build tools were not found."
}

$devCmd = Join-Path $vsPath "Common7\Tools\VsDevCmd.bat"
if (-not (Test-Path $devCmd)) {
    throw "VsDevCmd.bat was not found under $vsPath."
}

$argsText = ($CargoArgs | ForEach-Object { '"' + ($_ -replace '"', '\"') + '"' }) -join " "
cmd /c "`"$devCmd`" -arch=x64 -host_arch=x64 && cargo $argsText"
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

