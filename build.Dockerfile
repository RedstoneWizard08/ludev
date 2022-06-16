FROM ubuntu:22.04 as builder

ARG TARGET=x86_64-unknown-linux-gnu \
    TARGET_COMPILER=x86-64-linux-gnu

ENV RUSTUP_HOME=/usr/local/rust \
    CARGO_HOME=/usr/local/rust

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install curl bash \
        wget build-essential make \
        cmake gcc g++ clang libssl-dev \
        "gcc-${TARGET_COMPILER}" \
        "g++-${TARGET_COMPILER}"

SHELL [ "/bin/bash", "-c" ]

RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --no-modify-path && \
    source /usr/local/rust/env && \
    rustup default stable

RUN for FILE in ${RUSTUP_HOME}/bin/*; do \
        if [[ -f "/usr/local/bin/$(basename "${FILE}")" ]]; then \
            rm "/usr/local/bin/$(basename "${FILE}")"; \
        fi; \
        if [[ -f "${FILE}" ]]; then \
            ln -sf "${FILE}" "/usr/local/bin/$(basename "${FILE}")"; \
        fi; \
    done && \
    rustc --version && \
    cargo --version

ADD . /usr/src/rust
WORKDIR /usr/src/rust

RUN rustup default stable && \
    rustup target add "${TARGET}" && \
    cargo build --release --all-features \
    --target "${TARGET}" && \
    cp "target/${TARGET}/release/libgdlib_loader.so" \
    gdlib-loader.node

FROM scratch

COPY --from=builder "/usr/src/rust/gdlib-loader.node" /gdlib-loader.node

CMD [ "/gdlib-loader.node" ]
