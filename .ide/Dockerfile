FROM cnbcool/default-dev-env:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install build-essential clang lld -y

RUN curl -fsSL https://pixi.sh/install.sh | sh

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable \
    && . $HOME/.cargo/env \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos \
    && cargo install just

RUN npm install -g pnpm

RUN curl -fsSL https://code-server.dev/install.sh | sh && \
    code-server --install-extension cloudstudio.live-server && \
    code-server --install-extension cnbcool.cnb-welcome && \
    code-server --install-extension mhutchie.git-graph && \
    code-server --install-extension donjayamanne.githistory && \
    code-server --install-extension bierner.markdown-mermaid && \
    code-server --install-extension tencent-cloud.coding-copilot && \
    code-server --install-extension ms-ceintl.vscode-language-pack-zh-hans && \
    code-server --install-extension fill-labs.dependi && \
    code-server --install-extension tamasfe.even-better-toml && \
    code-server --install-extension skellock.just && \
    code-server --install-extension shd101wyy.markdown-preview-enhanced && \
    code-server --install-extension renan-r-santos.pixi-code && \
    code-server --install-extension ms-playwright.playwright && \
    code-server --install-extension ms-python.vscode-pylance && \
    code-server --install-extension ms-python.python && \
    code-server --install-extension ms-python.debugpy && \
    code-server --install-extension ms-python.vscode-python-envs && \
    code-server --install-extension rust-lang.rust-analyzer && \
    code-server --install-extension mrmlnc.vscode-scss && \
    code-server --install-extension redhat.vscode-yaml

WORKDIR /workspace

ENV PATH="/root/.cargo/bin:${PATH}"
ENV PATH="/root/.pixi/bin:${PATH}"
ENV LANG C.UTF-8
ENV LANGUAGE C.UTF-8