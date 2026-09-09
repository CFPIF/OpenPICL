# 只允许操作本地 kind 集群，避免误触其他环境
allow_k8s_contexts('kind-openpicl')

# 镜像构建
docker_build(
    'studio',
    context='./studio',
    dockerfile='./studio/Dockerfile',
)

docker_build(
    'website',
    context='./website',
    dockerfile='./website/Dockerfile',
)

docker_build(
    'docs',
    context='./docs',
    dockerfile='./docs/Dockerfile',
)

# 加载 K8s 清单
k8s_yaml([
    'k8s/studio.yaml',
    'k8s/website.yaml',
    'k8s/docs.yaml',
])

# 资源与端口转发
k8s_resource('studio', port_forwards=['3879:3879'])
k8s_resource('website', port_forwards=['3000:3000'])
k8s_resource('docs', port_forwards=['8080:80'])