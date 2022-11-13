ARG nodeVersion=16-alpine
ARG nginxVersion=1.23-alpine
ARG PKG

FROM node:${nodeVersion} as builder
ARG PKG
ADD /apps/${PKG} /builder
WORKDIR /builder
RUN yarn && yarn build
RUN mv /builder/public /app

FROM nginx:${nginxVersion}
COPY --from=builder /app /usr/share/nginx/html