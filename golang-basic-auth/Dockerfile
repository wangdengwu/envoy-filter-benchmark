FROM golang:1.21.0-bullseye as builder

WORKDIR /go/src/envoy-go-filer.example/example-basic-auth
COPY . .
RUN go env -w GO111MODULE=on && go env -w CGO_ENABLED=1 && go env && go build -v -o golang.so -buildmode=c-shared .


FROM  envoyproxy/envoy:contrib-v1.26.4
COPY --from=builder /go/src/envoy-go-filer.example/example-basic-auth/golang.so /etc/envoy/golang.so
RUN  apt update && apt install -y curl