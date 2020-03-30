## Learning Rust
Collection of code snippets written while learning rust.

## Run
```
cargo build
cargo test
```

```
cargo bench

     Running target\release\deps\synchronization-f70b8c2e41fa3d7d.exe
Gnuplot not found, using plotters backend
add                     time:   [738.24 ps 744.63 ps 752.29 ps]
                        change: [-14.961% -11.819% -8.7886%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe

mutex_add               time:   [29.043 ns 29.288 ns 29.594 ns]
                        change: [-45.211% -41.914% -38.692%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe

atomic_add              time:   [11.590 ns 11.692 ns 11.812 ns]
                        change: [-14.478% -12.149% -9.8086%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

```