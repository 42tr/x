FROM alpine
COPY ./x /tmp/x
WORKDIR /tmp/
RUN chmod +x x