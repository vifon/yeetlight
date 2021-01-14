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

func HttpMethod(method string, h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case method:
			h.ServeHTTP(w, r)
		default:
			log.Printf("Bad method: %v", r.Method)
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
		}
	})
}

func Get(h http.Handler) http.Handler {
	return HttpMethod(http.MethodGet, h)
}

func Post(h http.Handler) http.Handler {
	return HttpMethod(http.MethodPost, h)
}
