{{/*
渲染一个组件的全部资源：Deployment + Service（+ HTTPRoute）。
参数：dict "root" $ "name" <组件名> "app" <.Values 中的组件 values>
*/}}
{{- define "common.app" -}}
{{- $app := include "common.appValues" . | fromYaml -}}
{{- $ctx := dict "root" .root "name" .name "app" $app -}}
{{ include "common.deployment" $ctx }}
---
{{ include "common.service" $ctx }}
{{- if include "common.routeEnabled" $ctx }}
---
{{ include "common.httproute" $ctx }}
{{- end }}
{{- end -}}
