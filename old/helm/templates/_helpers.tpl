{{- define "app.name" -}}
{{- default .Chart.Name .Values.nameOverride }}
{{- end }}

{{- define "gateway.name" -}}
{{ include "app.name" . }}-gateway
{{- end }}

{{- define "frontend.name" -}}
{{ include "app.name" . }}-frontend
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "app.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride }}
{{- else }}
{{- $name := include "app.name" . }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 54 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 54 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{- define "gateway.fullname" -}}
{{ include "app.fullname" . }}-gateway
{{- end}}

{{- define "frontend.fullname" -}}
{{ include "app.fullname" . }}-frontend
{{- end}}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "app.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "app.labels" -}}
helm.sh/chart: {{ include "app.chart" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "gateway.labels" -}}
app.kubernetes.io/name: {{ include "gateway.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "frontend.labels" -}}
app.kubernetes.io/name: {{ include "frontend.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}