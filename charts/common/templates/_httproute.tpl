{{/*
HTTPRoute 中显式写出 API Server 会补全的默认字段（parentRefs/backendRefs 的 group、kind、weight 及默认路径匹配），
否则 ArgoCD 的客户端 diff 会一直显示 OutOfSync。
parentRefs：本 Chart 创建 Gateway 时，开启 TLS 则挂到对应域名的 HTTPS listener，否则挂到 http listener；
使用外部 Gateway 时不指定 sectionName。
参数：dict "root" $ "hosts" <域名列表>
*/}}
{{- define "common.parentRefs" -}}
{{- $root := .root -}}
{{- $gw := $root.Values.gateway -}}
{{- $name := include "common.gatewayName" $root -}}
{{- if and $gw.create $gw.tls.enabled -}}
{{- range .hosts }}
- group: gateway.networking.k8s.io
  kind: Gateway
  name: {{ $name }}
  sectionName: {{ include "common.httpsListenerName" . }}
{{- end }}
{{- else if $gw.create }}
- group: gateway.networking.k8s.io
  kind: Gateway
  name: {{ $name }}
  sectionName: http
{{- else }}
- group: gateway.networking.k8s.io
  kind: Gateway
  name: {{ $name }}
  {{- with $gw.namespace }}
  namespace: {{ . }}
  {{- end }}
{{- end }}
{{- end -}}

{{/* 参数：dict "root" $ "name" <组件名> "app" <已与 defaults 合并的组件 values> */}}
{{- define "common.httproute" -}}
{{- $root := .root -}}
{{- $app := .app -}}
{{- $host := include "common.host" . -}}
{{- $aliases := include "common.aliasHosts" . | fromYamlArray -}}
apiVersion: gateway.networking.k8s.io/v1
kind: HTTPRoute
metadata:
  name: {{ include "common.componentName" . }}
  labels:
    {{- include "common.labels" . | nindent 4 }}
spec:
  parentRefs:
    {{- include "common.parentRefs" (dict "root" $root "hosts" (list $host)) | trim | nindent 4 }}
  hostnames:
    - {{ $host | quote }}
  rules:
    - matches:
        - path:
            type: PathPrefix
            value: /
      backendRefs:
        - group: ""
          kind: Service
          name: {{ include "common.componentName" . }}
          port: {{ $app.service.port | default $app.containerPort }}
          weight: 1
{{- if $aliases }}
---
# 别名域名（如 www）301 跳转到主域名
apiVersion: gateway.networking.k8s.io/v1
kind: HTTPRoute
metadata:
  name: {{ printf "%s-alias" (include "common.componentName" .) | trunc 63 | trimSuffix "-" }}
  labels:
    {{- include "common.labels" . | nindent 4 }}
spec:
  parentRefs:
    {{- include "common.parentRefs" (dict "root" $root "hosts" $aliases) | trim | nindent 4 }}
  hostnames:
    {{- range $aliases }}
    - {{ . | quote }}
    {{- end }}
  rules:
    - matches:
        - path:
            type: PathPrefix
            value: /
      filters:
        - type: RequestRedirect
          requestRedirect:
            {{- if $root.Values.gateway.tls.enabled }}
            scheme: https
            {{- end }}
            hostname: {{ $host | quote }}
            statusCode: 301
{{- end }}
{{- end -}}
