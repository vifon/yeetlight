package api

import (
	"log"
	"net/http"
)

func WithLogging(h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%v: %v %v", r.RemoteAddr, r.Method, r.URL.String())
		h.ServeHTTP(w, r)
	})
}
