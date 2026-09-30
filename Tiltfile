# 只允许操作本地开发 kind 集群，避免误触生产集群 kind-openpicl
allow_k8s_contexts('kind-openpicl-dev')

# 镜像构建：名称与 Chart 中的 image.repository 一致，Tilt 据此替换为本地构建的镜像
docker_build(
    '117.50.75.30:8080/cfpif/openpicl.studio',
    context='./studio',
    dockerfile='./studio/Dockerfile',
)

docker_build(
    '117.50.75.30:8080/cfpif/openpicl.webide',
    context='./webide',
    dockerfile='./webide/Dockerfile',
)

docker_build(
    '117.50.75.30:8080/cfpif/openpicl.website',
    context='./website',
    dockerfile='./website/Dockerfile',
)

docker_build(
    '117.50.75.30:8080/cfpif/openpicl.docs',
    context='./docs',
    dockerfile='./docs/Dockerfile',
)

# 使用 Helm Chart 渲染清单：开启全部组件，不创建 Gateway，通过端口转发访问
local('helm dependency build charts/openpicl', quiet=True)
k8s_yaml(helm(
    'charts/openpicl',
    name='openpicl',
    values=['charts/openpicl/values-minimal.yaml'],
    set=['website.enabled=true'],
))

# 资源与端口转发
k8s_resource('openpicl-studio', port_forwards=['3100:3000'])
k8s_resource('openpicl-webide', port_forwards=['3102:3002'])
k8s_resource('openpicl-website', port_forwards=['3104:3004'])
k8s_resource('openpicl-docs', port_forwards=['3106:3006'])
