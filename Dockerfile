FROM ubuntu
# 安装 CA 证书
RUN apt-get update && apt-get install -y ca-certificates
COPY ./x /tmp/x
WORKDIR /tmp/
RUN chmod +x x