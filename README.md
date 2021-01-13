# tou

## build 
For your linux pc
```bash
cd tou
cargo build --release
```
For Everyone linux pc
```bash
cd tou
docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder cargo build --release
```

