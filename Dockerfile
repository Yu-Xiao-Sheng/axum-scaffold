FROM rust:1.81.0  AS builder
WORKDIR /app
# 拷贝项目
COPY . .
# 编译项目
RUN mkdir -p ~/.cargo && \
    echo '[source.crates-io]\nreplace-with = "ustc"\n\n[source.ustc]\nregistry = "git://mirrors.ustc.edu.cn/crates.io-index"' > $HOME/.cargo/config.toml
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN cargo update && \
    cargo build --target x86_64-unknown-linux-musl --release

# 使用更小的基础镜像运行服务
FROM scratch
# 复制构建产物到新的基础镜像
WORKDIR /server
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/axum-scaffold /server/axum-scaffold
COPY --from=builder /app/src/resources /server/src/resources

# 设置入口点
ENTRYPOINT ["/server/axum-scaffold"]