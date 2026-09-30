{{/* 参数：dict "root" $ "name" <组件名> "app" <已与 defaults 合并的组件 values> */}}
{{- define "common.deployment" -}}
{{- $app := .app -}}
{{- $root := .root -}}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ include "common.componentName" . }}
  labels:
    {{- include "common.labels" . | nindent 4 }}
spec:
  replicas: {{ $app.replicas }}
  revisionHistoryLimit: 3
  selector:
    matchLabels:
      {{- include "common.selectorLabels" . | nindent 6 }}
  template:
    metadata:
      labels:
        {{- include "common.labels" . | nindent 8 }}
      {{- with $app.podAnnotations }}
      annotations:
        {{- toYaml . | nindent 8 }}
      {{- end }}
    spec:
      automountServiceAccountToken: false
      enableServiceLinks: false
      {{- with $root.Values.global.imagePullSecrets }}
      imagePullSecrets:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with $app.podSecurityContext }}
      securityContext:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      terminationGracePeriodSeconds: {{ $app.terminationGracePeriodSeconds }}
      containers:
        - name: {{ .name }}
          image: {{ include "common.image" . | quote }}
          imagePullPolicy: {{ $app.image.pullPolicy | default "IfNotPresent" }}
          ports:
            - name: http
              containerPort: {{ $app.containerPort }}
              protocol: TCP
          {{- with $app.env }}
          env:
            {{- toYaml . | nindent 12 }}
          {{- end }}
          {{- with $app.securityContext }}
          securityContext:
            {{- toYaml . | nindent 12 }}
          {{- end }}
          {{- $probes := $app.probes | default dict }}
          {{- range $kind := list "startup" "readiness" "liveness" }}
          {{- with get $probes $kind }}
          {{ $kind }}Probe:
            httpGet:
              path: {{ $probes.path | default "/healthz" }}
              port: http
            {{- toYaml . | nindent 12 }}
          {{- end }}
          {{- end }}
          {{- with $app.resources }}
          resources:
            {{- toYaml . | nindent 12 }}
          {{- end }}
          {{- if $app.preStopSleepSeconds }}
          lifecycle:
            preStop:
              sleep:
                seconds: {{ $app.preStopSleepSeconds }}
          {{- end }}
          volumeMounts:
            - name: tmp
              mountPath: /tmp
      volumes:
        - name: tmp
          emptyDir:
            {{- with $app.tmpSizeLimit }}
            sizeLimit: {{ . }}
            {{- end }}
      {{- with $app.nodeSelector }}
      nodeSelector:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with $app.affinity }}
      affinity:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with $app.tolerations }}
      tolerations:
        {{- toYaml . | nindent 8 }}
      {{- end }}
{{- end -}}
