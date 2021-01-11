package api

import (
	"encoding/json"
	"net/http"
)

func TurnOnHandler() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			bulb(r, "turn", "on").Start()
		default:
			http.Error(w, "404 Not Found", http.StatusNotFound)
		}
	})
}
func TurnOffHandler() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			bulb(r, "turn", "off").Start()
		default:
			http.Error(w, "404 Not Found", http.StatusNotFound)
		}
	})
}

func Brightness() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			bulb(r, "brightness", r.URL.Query().Get("brightness")).Start()
		default:
			http.Error(w, "404 Not Found", http.StatusNotFound)
		}
	})
}
func Temperature() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			bulb(r, "temperature", r.URL.Query().Get("temperature")).Start()
		default:
			http.Error(w, "404 Not Found", http.StatusNotFound)
		}
	})
}

func GetInfo() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodGet:
			output, err := bulb(r, "status").Output()
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
			} else {
				info := parseInfo(string(output))
				infoJson, err := json.Marshal(info)
				if err != nil {
					http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				}
				w.Write(infoJson)
			}
		default:
			http.Error(w, "404 Not Found", http.StatusNotFound)
		}
	})
}

func Handle() {
	http.Handle("/", http.FileServer(http.Dir("./public")))
	http.Handle("/on", TurnOnHandler())
	http.Handle("/off", TurnOffHandler())
	http.Handle("/brightness", Brightness())
	http.Handle("/temperature", Temperature())
	http.Handle("/info", GetInfo())
}
