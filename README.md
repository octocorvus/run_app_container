# run_app_container
Run windows applications in [AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation)

## Examples

Running notepad:
```powershell
.\Start-IsolatedProcess.ps1 -Verbose -ContainerName MyContainer -ApplicationName C:\Windows\notepad.exe
```

Running PowerShell:
```powershell
.\Start-IsolatedProcess.ps1 -Verbose -ContainerName MyContainer -ApplicationName C:\Windows\System32\conhost.exe -Arguments powershell
```
