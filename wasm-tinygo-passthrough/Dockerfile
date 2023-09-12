FROM tinygo/tinygo:latest as builder
WORKDIR /go/src/wasm-tinygo.com/passthrough
COPY . .
RUN go env -w GO111MODULE=on
RUN go env -w GOPROXY=https://goproxy.cn,direct
RUN tinygo build -o tinygo.wasm -scheduler=none -target=wasi ./main.go


FROM  envoyproxy/envoy:contrib-v1.26.4
COPY --from=builder /go/src/wasm-tinygo.com/passthrough/tinygo.wasm /etc/envoy/tinygo.wasm
RUN  apt update && apt install -y curl