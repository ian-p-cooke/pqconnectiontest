# pqconnectiontest
```
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
echo DATABASE_URL=postgres://test:test@test/test > .env"
```
repeatedly run `./target/release/pqconnectiontest.exe` until exit code is 0xc0000374 (-1073740940)
