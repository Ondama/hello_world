FROM liuchong/rustup:stable

RUN apt-get update && apt-get install -y clang

RUN rustup target add x86_64-apple-darwin

RUN curl -sLo osxcross.tar.gz https://codeload.github.com/tpoechtrager/osxcross/tar.gz/master && \
    tar --strip=1 -xzf osxcross.tar.gz && \
    rm -f osxcross.tar.gz && \
    tools/get_dependencies.sh && \
    curl -sLo tarballs/MacOSX10.11.sdk.tar.xz https://s3.dockerproject.org/darwin/v2/MacOSX10.11.sdk.tar.xz && \
    UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh && \
    mv target /usr/local/osx-ndk-x86

ENV PATH /usr/local/osx-ndk-x86/bin:$PATH

RUN printf '[target.x86_64-apple-darwin]\nlinker = "x86_64-apple-darwin15-cc"\nar = "x86_64-apple-darwin15-ar"' > ~/.cargo/config

CMD ["cargo","build","--target=x86_64-apple-darwin","--release"]
