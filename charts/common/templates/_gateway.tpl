{{/*
参数：dict "root" $ "components" <组件名列表>
HTTPS listener 按域名逐个生成（cert-manager 依据 listener 的 hostname 签发证书），共用 gateway.tls.secretName。
*/}}
{{- define "common.gateway" -}}
{{- $root := .root -}}
{{- $gw := $root.Values.gateway -}}
{{- $hosts := list -}}
{{- range .components -}}
{{- $ctx := dict "root" $root "name" . "app" (get $root.Values .) -}}
{{- if and $ctx.app.enabled (include "common.routeEnabled" $ctx) -}}
{{- $hosts = append $hosts (include "common.host" $ctx) -}}
{{- $hosts = concat $hosts (include "common.aliasHosts" $ctx | fromYamlArray) -}}
{{- end -}}
{{- end -}}
{{- $name := include "common.gatewayName" $root -}}
apiVersion: gateway.networking.k8s.io/v1
kind: Gateway
metadata:
  name: {{ $name }}
  labels:
    {{- include "common.chartLabels" $root | nindent 4 }}
  {{- with $gw.annotations }}
  annotations:
    {{- toYaml . | nindent 4 }}
  {{- end }}
spec:
  gatewayClassName: {{ required "gateway.className 不能为空" $gw.className }}
  listeners:
    - name: http
      protocol: HTTP
      port: 80
      allowedRoutes:
        namespaces:
          from: Same
    {{- if $gw.tls.enabled }}
    {{- range $hosts }}
    - name: {{ include "common.httpsListenerName" . }}
      protocol: HTTPS
      port: 443
      hostname: {{ . | quote }}
      tls:
        mode: Terminate
        certificateRefs:
          # 显式写出默认 group，避免 ArgoCD diff 显示 OutOfSync
          - group: ""
            kind: Secret
            name: {{ required "gateway.tls.secretName 不能为空" $gw.tls.secretName }}
      allowedRoutes:
        namespaces:
          from: Same
    {{- end }}
    {{- end }}
{{- if and $gw.tls.enabled $gw.tls.redirectHTTP $hosts }}
---
# HTTP 统一 301 跳转到 HTTPS（ACME HTTP-01 的 Exact 路由优先级更高，不受影响）
apiVersion: gateway.networking.k8s.io/v1
kind: HTTPRoute
metadata:
  name: {{ printf "%s-https-redirect" $name | trunc 63 | trimSuffix "-" }}
  labels:
    {{- include "common.chartLabels" $root | nindent 4 }}
spec:
  parentRefs:
    - group: gateway.networking.k8s.io
      kind: Gateway
      name: {{ $name }}
      sectionName: http
  hostnames:
    {{- range $hosts }}
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
            scheme: https
            statusCode: 301
{{- end }}
{{- end -}}
