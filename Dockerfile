FROM ghcr.io/pyo3/maturin

RUN yum -y localinstall --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-7.noarch.rpm \
    && yum -y install clang epel-release ffmpeg ffmpeg-devel centos-release-scl llvm-toolset-7 \
    && echo "scl enable llvm-toolset-7 bash" > /etc/profile.d/enablellvm.sh


ENV LD_LIBRARY_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64:$LD_LIBRARY_PATH

RUN cd /tmp && \
      curl -O https://www.nasm.us/pub/nasm/releasebuilds/2.16.03/nasm-2.16.03.tar.gz && \
      tar xzf nasm-2.16.03.tar.gz && \
      cd nasm-2.16.03 && \
      ./configure --prefix=/usr && \
      make && \
      make install && \
      rm -rf /tmp/nasm-2.16.03*

WORKDIR /io
ENTRYPOINT ["/usr/bin/maturin"]
