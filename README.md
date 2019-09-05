# pqconnectiontest
```
set RUSTFLAGS environment variable to "-C target-feature=+crt-static" 
cargo build --release
create .env file with contents like "DATABASE_URL=postgres://test:test@test/test"
```
repeatedly run `./target/release/pqconnectiontest.exe` until exit code is 0xc0000374 (-1073740940)
