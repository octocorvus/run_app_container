param (
    [Parameter(Mandatory, Position = 0, ValueFromPipeline)]
    [string]$ContainerName,
    [Parameter(Mandatory, Position = 1, ValueFromPipeline)]
    [string]$ApplicationName,
    [Parameter(Position = 2, ValueFromPipeline)]
    [string[]]$Arguments,
    [Parameter(ValueFromPipeline)]
    [switch]$ReleaseBuild,
    [Parameter(ValueFromPipeline)]
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

$exePath = ".\target\debug\run_app_container.exe"
$cargoArgs = @("build")
if ($ReleaseBuild) {
    $cargoArgs += "--release"
    $exePath = ".\target\release\run_app_container.exe"
}
if (!$SkipBuild -or !(Test-Path $exePath)) {
    cargo $cargoArgs
}

$commandLine = "--container-name", $ContainerName, "--application-name", $ApplicationName
if ($VerbosePreference -eq [System.Management.Automation.ActionPreference]::Continue) {
    $commandLine += "--debug", "--debug"
}
elseif ($DebugPreference -eq [System.Management.Automation.ActionPreference]::Continue) {
    $commandLine += "--debug"
}
if ($Arguments.Count -ne 0) {
    $commandLine += "--command-line"
    $commandLine += $Arguments | ForEach-Object { $_.Replace("`"", "\`"") }
}

& $exePath $commandLine
