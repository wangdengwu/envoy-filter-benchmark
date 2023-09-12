package main

import (
	"encoding/base64"
	"github.com/tetratelabs/proxy-wasm-go-sdk/proxywasm"
	"github.com/tetratelabs/proxy-wasm-go-sdk/proxywasm/types"
	"strings"
)

const (
	expectedUsername = "dengwu.wang"
	expectedPassword = "password"
)

func main() {
	proxywasm.SetVMContext(&vmContext{})
}

type vmContext struct {
	types.DefaultVMContext
}

func (*vmContext) NewPluginContext(uint32) types.PluginContext {
	return &pluginContext{}
}

type pluginContext struct {
	types.DefaultPluginContext
}

func (p *pluginContext) NewHttpContext(contextID uint32) types.HttpContext {
	return &httpHeaders{
		contextID: contextID,
	}
}

type httpHeaders struct {
	types.DefaultHttpContext
	contextID uint32
}

func parseBasicAuth(auth string) (username, password string, ok bool) {
	const prefix = "Basic "
	if len(auth) < len(prefix) || !strings.EqualFold(auth[:len(prefix)], prefix) {
		return "", "", false
	}
	c, err := base64.StdEncoding.DecodeString(auth[len(prefix):])
	if err != nil {
		return "", "", false
	}
	cs := string(c)
	username, password, ok = strings.Cut(cs, ":")
	if !ok {
		return "", "", false
	}
	return username, password, true
}

func (ctx *httpHeaders) OnHttpRequestHeaders(int, bool) types.Action {
	auth, err := proxywasm.GetHttpRequestHeader("authorization")
	if err != nil {
		_ = proxywasm.SendHttpResponse(403, nil, []byte("forbidden"), 0)
		return types.ActionContinue
	}

	username, password, ok := parseBasicAuth(auth)
	if !ok {
		_ = proxywasm.SendHttpResponse(403, nil, []byte("forbidden"), 0)
		return types.ActionContinue
	}
	if username != expectedUsername || password != expectedPassword {
		_ = proxywasm.SendHttpResponse(403, nil, []byte("forbidden"), 0)
	}

	return types.ActionContinue
}
