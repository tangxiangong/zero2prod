# 使用 rust 官方镜像作为构建环境
FROM rust:1.84 as builder

# 设置工作目录
WORKDIR /usr/src/webserver

# 复制项目文件
COPY . .
RUN cargo install --path .
RUN cargo install sqlx-cli 

# 构建项目
RUN cargo build --release

# 使用轻量级镜像作为运行环境
FROM debian:bookworm-slim

# 安装 SSL 证书和其他依赖
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m appuser
USER appuser

# 设置工作目录
WORKDIR /webserver

# 从构建阶段复制二进制文件
COPY --from=builder /usr/src/webserver/target/release/app /webserver/

# 从构建阶段复制配置文件
COPY --from=builder /usr/src/webserver/configration.yml /webserver/

# 暴露应用端口
EXPOSE 3000

# 启动应用
CMD ["./app"]