ARG PLAYWRIGHT_IMAGE_TAG=v1.59.1-noble
FROM mcr.microsoft.com/playwright:${PLAYWRIGHT_IMAGE_TAG}

ARG RUST_TOOLCHAIN=stable

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:${PATH}
ENV CI=true

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain "${RUST_TOOLCHAIN}" \
    && chmod -R a+w "${RUSTUP_HOME}" "${CARGO_HOME}" \
    && rustc --version \
    && cargo --version \
    && node --version \
    && npm --version

WORKDIR /workspace
