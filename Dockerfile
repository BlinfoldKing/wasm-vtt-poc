FROM debian:bookworm

WORKDIR app

COPY dist dist

ENTRYPOINT [ "./dist/server" ]
