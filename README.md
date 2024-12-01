# 1🐝🏎 (One Billion Row Challenge)
## System Configuration: 
```powershell
 Get-WmiObject -Class Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors

Name                                     NumberOfCores NumberOfLogicalProcessors
----                                     ------------- -------------------------
Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz             6                        12


 Get-WmiObject -Class Win32_ComputerSystem | Select-Object TotalPhysicalMemory                                                                                                                                                                                                                                         TotalPhysicalMemory
-------------------
        12717314048


 systeminfo | findstr /C:"Total Physical Memory"
Total Physical Memory:     12,128 MB
 echo $env:NUMBER_OF_PROCESSORS
12
```
## Optimization links: 
### Compilation: 
- https://deterministic.space/high-performance-rust.html
- https://nnethercote.github.io/perf-book/build-configuration.html

## Links: 
- [Gunnar Morling Git](https://github.com/gunnarmorling/1brc)
- [1brc.dev](https://1brc.dev/)
- [tumdum rust solution](https://github.com/tumdum/1brc/blob/main/src/main.rs)

## Testing
- Base line code tested with hyperfine
```cmd
 hyperfine --runs 3 target\release\obrc-mm.exe
Benchmark 1: target\release\obrc-mm.exe
  Time (mean ± σ):     291.009 s ±  9.233 s    [User: 254.047 s, System: 5.234 s]
  Range (min … max):   280.752 s … 298.656 s    3 runs
```
