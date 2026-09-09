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

# 静态站点
COPY --from=builder /app/doc_build /usr/share/nginx/html

# 使用独立 nginx 配置，支持 clean URL 访问
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD [ "nginx", "-g", "daemon off;" ]