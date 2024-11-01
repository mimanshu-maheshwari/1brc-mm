# 1🐝🏎 (One Billion Row Challenge)
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

- After compiler optimizations, memory mapping: 
```cmd 
 hyperfine --runs 10 target\release\obrc-mm.exe
Benchmark 1: target\release\obrc-mm.exe
  Time (mean ± σ):      1.045 s ±  1.376 s    [User: 0.939 s, System: 0.935 s]
  Range (min … max):    0.157 s …  4.621 s    10 runs
```


