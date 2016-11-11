FROM ubuntu:latest

RUN apt-get update
RUN apt-get install -y curl libssl-dev

ENV dir /ts_restart

WORKDIR ${dir}
ADD . $foo
COPY target/debug/ts_restart .
COPY info.json .
COPY auth.json .
CMD ["./ts_restart"]
