{{/* 本 Chart 包含的全部组件，新增组件时同步添加 */}}
{{- define "openpicl.components" -}}
{{- toYaml (list "studio" "webide" "website" "docs") -}}
{{- end -}}
