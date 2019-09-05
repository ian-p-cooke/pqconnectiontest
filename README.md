# pqconnectiontest
```
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
```
repeatedly run `./target/release/pqconnectiontest.exe` until
```
error: process didn't exit successfully: `target\release\pqconectiontest.exe` (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
```