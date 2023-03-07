Python bindings for [bliss-rs](https://github.com/Polochon-street/bliss-rs/)
============================================================================

This repo contains the bare minimum to create and upload a wheel to
[Pypi](https://pypi.org/project/bliss-audio/).

It is more of a personal reminder on how to do things, but can also be used
as inspiration for other projects.

All of the below commands assume that you're in a fresh clone of this
repository (and some docker commands may need to be run with `sudo` if docker is
not configured to work as your current user).

Local build for your own python version
=======================================

```
maturin build --release
```

Should be enough. You can then proceed to install the built wheel with
```
pip install target/wheels/<wheel_name>.whl
```

Local build for all python versions (3.6+)
==========================================

Build the maturin docker image, with a couple additions (clang, ffmpeg...)
```
docker build -t polochon-street/bliss-python .
```

Then, still in the clone (replace "build --release" by "publish --no-sdist -f" if
you have access to the Pypi repository and want to publish a new version):
```
docker run --rm -v $(pwd):/io polochon-street/bliss-python build --release -f
```

Finally, you can install the wheel with (or simply `pip install
bliss_audio`, if you just used `maturin publish`).
```
pip install target/wheels/<wheel_name>.whl
```

Local cross-compilation from x64 to arm64:
==========================================

You can make use of [cross](https://github.com/cross-rs/cross) to cross-compile
a wheel to a desired architecture.

For example, for arm64, install cross, and run

```
~/.cargo/bin/cross build --target aarch64-unknown-linux-gnu --verbose
```

as a test to check that you can build it properly.

Then copy/paste the last command - something like this:
```
sudo /usr/bin/docker run --userns host -e 'PKG_CONFIG_ALLOW_CROSS=1' -e 'XARGO_HOME=/root/.xargo' -e 'CARGO_HOME=/root/.cargo' -e 'CROSS_RUST_SYSROOT=/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu' -e 'CARGO_TARGET_DIR=/target' -e 'CROSS_RUNNER=' -e TERM -e 'USER=root' -e 'CROSS_RUSTC_MAJOR_VERSION=1' -e 'CROSS_RUSTC_MINOR_VERSION=67' -e 'CROSS_RUSTC_PATCH_VERSION=1' --name cross-stable-x86_64-unknown-linux-gnu-53e1a-d5a82bbd2-aarch64-unknown-linux-gnu-e7d5d-1677439406313 --rm --user 0:0 -v /root/.xargo:/root/.xargo:z -v /root/.cargo:/root/.cargo:z -v /root/.cargo/bin -v path:path:z -v /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu:/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu:z,ro -v path/target:/target:z -w patch -t localhost/cross-rs/cross-custom-bliss-python:aarch64-unknown-linux-gnu-e7d5d-pre-build sh -c 'PATH="$PATH":"/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin" cargo build --release --target aarch64-unknown-linux-gnu --verbose'
```

Don't forget to `cargo clean`! Otherwise cargo's previous
build will be used, and the linking will fail.

Replace that last `cargo` with `maturin` (either keep `maturin build --release -f` or
use `maturin publish -f` if you want to publish it directly - you probably want to
build it and then test it on a virtual machine with the correct architecture).

Notice that we also need to add `--features=pyo3/abi3-py38,pyo3/extension-module`
to prevent linking against libpython and make the build succeed.

It is ugly, and I'd be glad if someone has a better option to run custom
commands with `cross`.

To install the wheel, as usual run 
```
pip install target/wheels/<wheel_name>.whl
```

from a computer / virtual machine with the right architecture.

For me, following this https://wiki.ubuntu.com/ARM64/QEMU, replacing the
command with
```
sudo qemu-system-aarch64 -m 1024 -cpu cortex-a57 -M virt -nographic -pflash /usr/share/AAVMF/AAVMF_CODE.fd -pflash flash1.img -drive if=none,file=jammy-server-cloudimg-arm64.img,id=hd0 -device virtio-blk-device,drive=hd0 -netdev type=tap,id=net0 -device virtio-net-device,netdev=net0,mac=08:00:27:33:5b:40
```

And then installing python3, pip and ffmpeg in the container worked a charm to
test arm64.

Final check
===========

To check that everything is working, you can run the following code:
```
python -c 'from bliss_audio import Song; print(Song(<path/to/a/valid/song>).analysis)'
```

If it completes and prints some numbers, congratulations! If is working
properly.
