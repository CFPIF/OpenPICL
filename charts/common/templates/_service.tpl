{{/* 参数：dict "root" $ "name" <组件名> "app" <已与 defaults 合并的组件 values> */}}
{{- define "common.service" -}}
{{- $app := .app -}}
apiVersion: v1
kind: Service
metadata:
  name: {{ include "common.componentName" . }}
  labels:
    {{- include "common.labels" . | nindent 4 }}
spec:
  type: {{ $app.service.type | default "ClusterIP" }}
  selector:
    {{- include "common.selectorLabels" . | nindent 4 }}
  ports:
    - name: http
      port: {{ $app.service.port | default $app.containerPort }}
      targetPort: http
      protocol: TCP
{{- end -}}
