# `@silicon-soldier/windows-psapi`

High-level interface over Windows' Process Status API (`Win32::System::ProcessStatus` and `psapi.h`).

## Usage

### `pidPath`

```js
import psapi from "@silicon-soldier/windows-psapi";

// Current process (immune to `process.execPath` manipulation)
console.info(psapi.pidPath(process.pid));
// C:\Program Files\nodejs\node.exe

// Parent process
console.info(psapi.pidPath(process.ppid));
// C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe
```

## Status

### Windows PSAPI Coverage

The crate `windows@0.48.0` provides projection of the Windows API to Rust. This is what is used behind the scenes.

| PSAPI | |
| - | - |
| `EmptyWorkingSet` | ✗ |
| `EnumDeviceDrivers` | ✗ |
| `EnumPageFilesA` | ✗ |
| `EnumPageFilesW` | ✗ |
| `EnumProcessModules` | ✗ |
| `EnumProcessModulesEx` | ✗ |
| `EnumProcesses` | ✗ |
| `GetDeviceDriverBaseNameA` | ✗ |
| `GetDeviceDriverBaseNameW` | ✗ |
| `GetDeviceDriverFileNameA` | ✗ |
| `GetDeviceDriverFileNameW` | ✗ |
| `GetMappedFileNameA` | ✗ |
| `GetMappedFileNameW` | ✗ |
| `GetModuleBaseNameA` | ✗ |
| `GetModuleBaseNameW` | ✗ |
| `GetModuleFileNameExA` | ✓ |
| `GetModuleFileNameExW` | ✗ |
| `GetModuleInformation` | ✗ |
| `GetPerformanceInfo` | ✗ |
| `GetProcessImageFileNameA` | ✗ |
| `GetProcessImageFileNameW` | ✗ |
| `GetProcessMemoryInfo` | ✗ |
| `GetWsChanges` | ✗ |
| `GetWsChangesEx` | ✗ |
| `InitializeProcessForWsWatch` | ✗ |
| `QueryWorkingSet` | ✗ |
| `QueryWorkingSetEx` | ✗ |

## See Also

- [@silicon-soldier/darwin-libproc](https://github.com/Silic0nS0ldier/darwin-libproc-nodejs) for macOS
- `proc/{pid}/` for Linux
