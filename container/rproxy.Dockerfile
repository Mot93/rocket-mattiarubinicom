FROM docker.io/nginx:1-alpine

RUN rm -rf /etc/nginx/conf.d/

COPY rproxy/homepage.conf /etc/nginx/conf.d/