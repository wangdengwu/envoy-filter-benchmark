package main

import (
	"encoding/base64"
	"strings"

	"github.com/envoyproxy/envoy/contrib/golang/filters/http/source/go/pkg/api"
)

type filter struct {
	callbacks api.FilterCallbackHandler
	config    *config
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

func (f *filter) verify(header api.RequestHeaderMap) (bool, string) {
	auth, ok := header.Get("authorization")
	if !ok {
		return false, "no Authorization"
	}
	username, password, ok := parseBasicAuth(auth)
	if !ok {
		return false, "invalid Authorization format"
	}
	if f.config.username == username && f.config.password == password {
		return true, ""
	}
	return false, "invalid username or password"
}

func (f *filter) DecodeHeaders(header api.RequestHeaderMap, _ bool) api.StatusType {
	if ok, msg := f.verify(header); !ok {
		f.callbacks.SendLocalReply(401, msg, map[string]string{}, 0, "bad-request")
		return api.LocalReply
	}
	return api.Continue
}

func (f *filter) DecodeData(api.BufferInstance, bool) api.StatusType {
	return api.Continue
}

func (f *filter) DecodeTrailers(api.RequestTrailerMap) api.StatusType {
	return api.Continue
}

func (f *filter) EncodeHeaders(api.ResponseHeaderMap, bool) api.StatusType {
	return api.Continue
}

func (f *filter) EncodeData(api.BufferInstance, bool) api.StatusType {
	return api.Continue
}

func (f *filter) EncodeTrailers(api.ResponseTrailerMap) api.StatusType {
	return api.Continue
}

func (f *filter) OnDestroy(api.DestroyReason) {
}
