# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository layout

OpenPICL (Open Photon Intelligence Comprehensive Laboratory) is a polyrepo-style monorepo: **there is no root Cargo workspace**. Each top-level directory is an independent project with its own `Cargo.toml`/`Cargo.lock` (or `package.json`) and its own `justfile`. Always `cd` into the relevant project before running cargo/pnpm/just commands.

| Dir | What it is |
| --- | --- |
| `rust/` | Core Rust library workspace (`crates/*`), published to a private Nexus Cargo registry |
| `website/`, `studio/`, `webide/` | Three Leptos 0.8 SSR + hydrate web apps with identical structure |
| `docs/` | Rspress 2 documentation site (pnpm), bilingual `docs/docs/{en,zh}` |
| `pixi/` | Pixi workspace; `packages/pyphotensor-unit` is a PyO3/maturin Python binding for `photensor` |
| `charts/` | Helm: `common` (library chart of shared templates) + `openpicl` (umbrella app chart) |
| `deploy/` | Prod kind config, ArgoCD values, app-of-apps (`bootstrap.yaml` → `apps/`: self-managed ArgoCD, Envoy Gateway, cert-manager + Let's Encrypt ClusterIssuers, GatewayClass/EnvoyProxy, `charts/openpicl` with `values-prod.yaml`) synced from `main`; order via sync-wave |

Code comments, CLI output, justfile descriptions and commit messages are written in Chinese. Commits follow gitmoji + conventional style, e.g. `✨ feat(website): ...`, `🔧 chore(build): ...`.

## Commands

Every project's justfile lists its recipes with `just`. Common ones in `rust/`, `website/`, `studio/`, `webide/`:

```bash
just test      # cargo test
just check     # cargo check -p <each crate>
just fmt       # cargo fmt --all -- --check  (check only; run `cargo fmt --all` to apply)
just clippy    # cargo clippy -p <each crate> -- -D warnings  (warnings are errors)
```

Single test: `cargo test -p <crate> <test_name>` from inside the workspace dir.

Leptos apps (`website/`, `studio/`, `webide/`) need `cargo-leptos` (0.3.7 in Dockerfiles) and the `wasm32-unknown-unknown` target:

```bash
just run       # cargo leptos serve
just reload    # cargo leptos watch (hot reload)
cargo leptos end-to-end   # Playwright tests in end2end/
```

Docs: `pnpm dev`, `pnpm build`, `pnpm run format` (Prettier), `pnpm run lint` (Rslint).

Pixi: `just build <package>` in `pixi/` (runs `pixi build`); Python lint via `ruff`.

Charts: `just lint` (lints default/minimal/prod values with `--strict`) / `just template values-prod.yaml` in `charts/`; both run `helm dependency build openpicl` first.

Whole stack: `docker compose up --build` (root). Dev k8s: `just cluster-up` (ctlptl + kind cluster `kind-openpicl-dev`) then `just tilt` (renders the Helm chart, port-forwards 3100-3106). Prod k8s is a separate kind cluster `kind-openpicl`: `just prod-cluster-up`, `just argocd-install`, `just argocd-bootstrap`.

## Build environment gotchas

- `rust/.cargo/config.toml` replaces crates.io with a private Nexus mirror (`117.50.75.30:8083`) and sets `linker = "clang"` + `-fuse-ld=lld`. Builds require clang/lld installed and access to that mirror. The same applies to `pixi/packages/pyphotensor-unit`, which depends on `photensor` from that registry.
- `rust/.cargo/config.toml` sets `CARGO_WORKSPACE_DIR`; `nebula-core` uses it in `include_dir!("$CARGO_WORKSPACE_DIR/assets/templates")`, so templates are embedded at compile time and building outside the `rust/` workspace breaks.
- Leptos apps' `.cargo/config.toml` sets `getrandom_backend="wasm.js"` for the wasm target.
- Rust edition is 2024 everywhere. rustfmt: `max_width = 100`, `match_block_trailing_comma = true`. clippy: no `unwrap`/`expect`/`dbg` allowances in tests.
- The prod host is in mainland China: docker.io is unreachable and ghcr.io is flaky. kind nodes read registry mirrors from `deploy/containerd/certs.d` (synced to the host's `/etc/containerd/certs.d` by `just prod-registry-setup`; `containerdConfigPatches` in `deploy/kind-cluster.yaml` enables `config_path`). Helm charts come from mirrors/OCI (`docker.m.daocloud.io/envoyproxy`, `ghcr.io/argoproj/argo-helm`), not docker.io or argoproj.github.io.
- Artifacts are published to self-hosted Harbor (images + Helm OCI, `:8080`), Nexus (Cargo, `:8083`) and Quetz (conda, `:8090`) via `just login` / `just publish` recipes.

## Architecture

### `rust/` crates

- **nebula** (bin): clap CLI. `nebula template list` / `nebula template init <template> --name <n> [--var k=v] [--dry-run] [--force] [--non-interactive]`. Thin layer over `nebula-core`.
- **nebula-core**: template engine. Templates live in `rust/assets/templates/<name>/`, each with a `template.toml` (metadata + `[variables]` with prompt/default) and a file tree whose paths and contents are rendered with Tera (e.g. `{{project-name}}/`). Variable names with `-` are also exposed under a safe `_` alias for Tera.
- **nebula-protos**: shared serde types (`TemplateMeta`, `InitOptions`, `InitReport`, `OverwriteMode`...).
- **photensor\***: photonic tensor library split into `-core`, `-sim` (uses `oxiphoton`), `-hw`, `-burn` (Burn integration), `-macros`, with `photensor` as the facade. Currently stubs.

Intra-workspace deps are declared with both `path` and `version` + `registry = "cargo-hosted"` so they can be published.

### Leptos apps (`website`, `studio`, `webide`)

Same six-crate layout; `[[workspace.metadata.leptos]]` in each root `Cargo.toml` wires it together:

- **app**: all UI (routes, pages, components). Features `ssr` / `hydrate` gate server vs. client code.
- **server** (bin-package): Axum server, enables `app/ssr`, serves Leptos routes plus `GET /healthz` (used by k8s probes), graceful shutdown on SIGTERM with a 30s timeout.
- **frontend** (lib-package, cdylib): wasm entry `hydrate()` enabling `app/hydrate`.
- **protocol**, **renderer**, **worker**: currently empty placeholders.

`website` additionally uses `leptos_i18n`: `app/build.rs` generates the i18n module from `website/assets/locales/{zh-CN,en}.json` (default locale `zh-CN`); new UI strings must be added to both files.

Ports — dev (`cargo leptos`): studio 3006, webide 3008, website 3010. Container/k8s: studio 3000, webide 3002, website 3004, docs 3006 (nginx, see `docs/nginx.conf`). Dockerfiles are multi-stage (rust alpine → alpine) and run as non-root uid 10001; when adding a crate to an app workspace, also add its `Cargo.toml` copy and stub source lines in that app's Dockerfile dependency-caching stage.

### Helm

`charts/openpicl/values.yaml` holds all defaults (every service `enabled: false`); `values-minimal.yaml` / `values-prod.yaml` are overlays containing only diffs, validated by `values.schema.json`. Each service template is a thin `include "common.app"` that renders Deployment + Service + HTTPRoute from `charts/common/templates/`. Per-service values fall back to `.Values.defaults` per top-level key (whole-key replacement, no deep merge). Exposure is via Gateway API: hosts are `<route.subdomain>.<global.domain>` (`openpicl.com`; website on the apex, `www` redirects). HTTPRoutes are only rendered when `gateway.create` or `gateway.name` is set. Pods run as uid 10001 with a read-only root FS and an emptyDir on `/tmp`. When adding a service, add it to `openpicl.components` in `charts/openpicl/templates/_helpers.tpl` and to the schema.
