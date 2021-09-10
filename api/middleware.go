package api

import (
	"log"
	"net/http"
)

// WithLogging logs the info about each handled request (but not response).
func WithLogging(h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%v: %v %v", r.RemoteAddr, r.Method, r.URL.String())
		h.ServeHTTP(w, r)
	})
}

// HttpMethod checks for the HTTP method of each handled request,
// rejecting all the other HTTP methods.
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

// Get allows only HTTP GET requests.
func Get(h http.Handler) http.Handler {
	return HttpMethod(http.MethodGet, h)
}

// Post allows only HTTP POST requests.
func Post(h http.Handler) http.Handler {
	return HttpMethod(http.MethodPost, h)
}
