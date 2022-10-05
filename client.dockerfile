ARG nodeVersion=16-alpine
ARG nginxVersion=1.23-alpine

FROM node:${nodeVersion} as builder
ADD client /builder
WORKDIR /builder
RUN yarn build
RUN mv /builder/public /app

FROM nginx:${nginxVersion}
COPY --from=builder /app /usr/share/nginx/html