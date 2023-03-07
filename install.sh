#!/usr/bin/env bash

dpkg --add-architecture arm64
apt-get update && apt-get install -y clang pip python3.8 libpython3.8-dev && apt-get install -y libpython3.8-dev:arm64 && ln -sf /usr/bin/python3.8 /usr/bin/python3
pip install maturin
