# Build Prerequisites
## libopenraw
- `$ sudo apt install automake autoconf autoconf-archive libboost-dev libboost-test-dev libjpeg-dev libtool libxml2-dev pkg-config`
The following are also invoked automatically by `build.rs`.
- `$ git submodule update --init`
- `$ ./autogen.sh`
- `$ ./configure --prefix $(pwd)-outdir`
- `$ cd lib`
- `$ make -j 8`
- `$ make install`

## Crate
- `$ sudo apt install libclang-dev`
- `$ cargo build`