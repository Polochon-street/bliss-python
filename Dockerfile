FROM ghcr.io/pyo3/maturin

RUN yum -y localinstall --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-7.noarch.rpm \
    && yum -y install clang epel-release ffmpeg ffmpeg-devel centos-release-scl llvm-toolset-7 \
    && echo "scl enable llvm-toolset-7 bash" > /etc/profile.d/enablellvm.sh

WORKDIR /io
ENTRYPOINT ["/usr/bin/maturin"]
