set windows-shell := ["powershell.exe"]
export RUST_BACKTRACE := "1"

# 展示可用的命令
@just:
    just --list

# 启动本地开发 kind 集群（Tilt 使用）
cluster-up:
    ctlptl apply -f ctlptl-cluster.yaml

# 删除本地开发 kind 集群
cluster-down:
    ctlptl delete cluster kind-openpicl-dev

# 启动 Tilt
tilt:
    tilt up

# 创建生产 kind 集群
prod-cluster-up:
    kind create cluster --config deploy/kind-cluster.yaml

# 删除生产 kind 集群
prod-cluster-down:
    kind delete cluster --name openpicl

# 在生产集群中安装 ArgoCD
argocd-install:
    helm repo add argo https://argoproj.github.io/argo-helm --force-update
    helm upgrade --install argocd argo/argo-cd --kube-context kind-openpicl -n argocd --create-namespace -f deploy/argocd-values.yaml --wait

# 部署根应用，由 ArgoCD 接管 deploy/apps 下的全部应用
argocd-bootstrap:
    kubectl --context kind-openpicl apply -f deploy/bootstrap.yaml

# 获取 ArgoCD 初始 admin 密码
argocd-password:
    kubectl --context kind-openpicl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d
