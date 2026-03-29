FROM node:20-alpine AS version-extractor
WORKDIR /repo
COPY CHANGELOG.md ./
COPY .backstage/set_version.cjs .backstage/
COPY docs/data/nav.yml docs/data/
RUN node .backstage/set_version.cjs > /tmp/version

FROM rust:1-bookworm AS builder

# Install Node.js and binaryen (provides wasm-opt)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs binaryen

# Install Hugo
ARG HUGO_VERSION=0.145.0
RUN wget -O hugo.tar.gz \
    "https://github.com/gohugoio/hugo/releases/download/v${HUGO_VERSION}/hugo_${HUGO_VERSION}_linux-amd64.tar.gz" && \
    tar -xzf hugo.tar.gz -C /usr/local/bin && \
    rm hugo.tar.gz

# Install just
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Install Rust tooling for WASM builds
RUN rustup target add wasm32-unknown-unknown && \
    rustup toolchain install nightly && \
    rustup component add rust-src --toolchain nightly && \
    cargo install wasm-pack --version 0.10.3 && \
    cargo install wasm-bindgen-cli --version 0.2.100

WORKDIR /repo
COPY . .
RUN just install-npm
RUN just build

# Build docs site
COPY --from=version-extractor /repo/docs/data/nav.yml docs/data/nav.yml
COPY --from=version-extractor /tmp/version /tmp/version
RUN cd docs && npm ci
RUN cd docs && export HUGO_PAGEFIND_DOCS_VERSION=$(cat /tmp/version) && hugo --minify
RUN ./target/release/pagefind --site docs/public --write-playground

FROM caddy:2-alpine
COPY docs/Caddyfile /etc/caddy/Caddyfile
COPY docs/redirects /etc/caddy/redirects
COPY --from=builder /repo/docs/public /srv
EXPOSE 8080
