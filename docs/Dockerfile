# syntax=docker/dockerfile:1

# 构建阶段
FROM node:22-alpine AS builder

# 全局安装 pnpm 9
RUN corepack enable && corepack prepare pnpm@9 --activate

WORKDIR /app

# 先复制依赖清单以利用缓存
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# 再复制文档源码
COPY ./ ./

# 构建，产物输出到 doc_build
RUN pnpm build

# 运行阶段
FROM nginx:alpine

# 创建非特权用户/组，UID/GID 固定为 10001，便于 K8s securityContext 对齐
RUN addgroup -S -g 10001 docs \
    && adduser -S -D -H -u 10001 -G docs docs

# 覆盖 nginx 主配置：pid、temp、日志都改为非 root 可写
COPY nginx.main.conf /etc/nginx/nginx.conf

# 站点 server 配置，监听 3006
COPY nginx.conf /etc/nginx/conf.d/default.conf

# 静态文件复制给 docs 用户
COPY --from=builder --chown=docs:docs /app/doc_build usr/share/nginx/html

# 预创建 nginx 临时目录并授权给 docs
RUN mkdir -p /tmp/nginx-client_temp \
    /tmp/nginx-proxy_temp \
    /tmp/nginx-fastcgi_temp \
    /tmp/nginx-uwsgi_temp \
    /tmp/nginx-scgi_temp \
    && chown -R docs:docs /tmp/nginx-*

EXPOSE 3006

USER 10001:10001

CMD [ "nginx", "-g", "daemon off;" ]