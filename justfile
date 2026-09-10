set windows-shell := ["powershell.exe"]
export RUST_BACKTRACE := "1"

# 展示可用的命令
@just:
    just --list

# 启动本地 kind 集群
cluster-up:
    ctlptl apply -f ctlptl-cluster.yaml

# 删除本地 kind 集群
cluster-down:
    ctlptl delete cluster kind-openpicl

# 启动 Tilt
tilt:
    tilt up