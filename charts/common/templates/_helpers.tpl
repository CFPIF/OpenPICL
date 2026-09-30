{{/*
约定：组件级模板统一接收 dict "root" $ "name" "<组件名>" "app" <组件 values>。
*/}}

{{/* 完整名称：release 名包含 chart 名时直接使用 release 名 */}}
{{- define "common.fullname" -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else if contains .Chart.Name .Release.Name -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" .Release.Name .Chart.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}

{{/* 组件资源名：<fullname>-<组件名> */}}
{{- define "common.componentName" -}}
{{- printf "%s-%s" (include "common.fullname" .root) .name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/* 选择器标签：创建后不可修改，只放稳定字段 */}}
{{- define "common.selectorLabels" -}}
app.kubernetes.io/name: {{ .name }}
app.kubernetes.io/instance: {{ .root.Release.Name }}
{{- end -}}

{{/* 通用标签 */}}
{{- define "common.labels" -}}
helm.sh/chart: {{ printf "%s-%s" .root.Chart.Name .root.Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{ include "common.selectorLabels" . }}
app.kubernetes.io/version: {{ include "common.imageTag" . | quote }}
app.kubernetes.io/part-of: {{ .root.Chart.Name }}
app.kubernetes.io/managed-by: {{ .root.Release.Service }}
{{- end -}}

{{/* Chart 级标签（Gateway 等不属于单个组件的资源） */}}
{{- define "common.chartLabels" -}}
helm.sh/chart: {{ printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/part-of: {{ .Chart.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{/* 镜像 tag，未设置时使用 Chart 的 appVersion */}}
{{- define "common.imageTag" -}}
{{- .app.image.tag | default .root.Chart.AppVersion | toString -}}
{{- end -}}

{{- define "common.image" -}}
{{- printf "%s:%s" .app.image.repository (include "common.imageTag" .) -}}
{{- end -}}

{{/*
组件 values 与 .Values.defaults 合并：按顶层键整体覆盖，组件中出现的键完全替换默认值。
不使用 merge/mergeOverwrite 深合并，因为它们会把 false / 0 / "" 当作空值忽略。
用法：$app := include "common.appValues" . | fromYaml
*/}}
{{- define "common.appValues" -}}
{{- $out := deepCopy .app -}}
{{- range $key, $value := .root.Values.defaults -}}
{{- if not (hasKey $out $key) -}}
{{- $_ := set $out $key $value -}}
{{- end -}}
{{- end -}}
{{- toYaml $out -}}
{{- end -}}

{{/* Gateway 名称：由本 Chart 创建时为 <fullname>-gateway，否则取 gateway.name */}}
{{- define "common.gatewayName" -}}
{{- if .Values.gateway.create -}}
{{- .Values.gateway.name | default (printf "%s-gateway" (include "common.fullname" .)) -}}
{{- else -}}
{{- .Values.gateway.name -}}
{{- end -}}
{{- end -}}

{{/* 组件主域名：subdomain 为空时使用一级域名 */}}
{{- define "common.host" -}}
{{- $domain := required "global.domain 不能为空" .root.Values.global.domain -}}
{{- if .app.route.subdomain -}}
{{- printf "%s.%s" .app.route.subdomain $domain -}}
{{- else -}}
{{- $domain -}}
{{- end -}}
{{- end -}}

{{/* 组件别名域名列表（跳转到主域名），输出 YAML 列表 */}}
{{- define "common.aliasHosts" -}}
{{- $hosts := list -}}
{{- range .app.route.aliases -}}
{{- $hosts = append $hosts (printf "%s.%s" . $.root.Values.global.domain) -}}
{{- end -}}
{{- toYaml $hosts -}}
{{- end -}}

{{/* 组件是否对外暴露：需同时开启组件路由且存在可挂载的 Gateway */}}
{{- define "common.routeEnabled" -}}
{{- if and .app.route .app.route.enabled (include "common.gatewayName" .root) -}}true{{- end -}}
{{- end -}}

{{/* HTTPS listener 名称 */}}
{{- define "common.httpsListenerName" -}}
{{- printf "https-%s" . | trunc 253 -}}
{{- end -}}
