package api

import (
	"encoding/json"
	"net/http"
)

func TurnOnHandler() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			cmd := bulb(r, "turn", "on")
			if cmd == nil {
				http.Error(w, "400 Bad Request", http.StatusBadRequest)
				return
			}
			cmd.Start()
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
		}
	})
}
func TurnOffHandler() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			cmd := bulb(r, "turn", "off")
			if cmd == nil {
				http.Error(w, "400 Bad Request", http.StatusBadRequest)
				return
			}
			cmd.Start()
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
		}
	})
}

func Brightness() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			cmd := bulb(r, "brightness", r.URL.Query().Get("brightness"))
			if cmd == nil {
				http.Error(w, "400 Bad Request", http.StatusBadRequest)
				return
			}
			cmd.Start()
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
		}
	})
}
func Temperature() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			cmd := bulb(r, "temperature", r.URL.Query().Get("temperature"))
			if cmd == nil {
				http.Error(w, "400 Bad Request", http.StatusBadRequest)
				return
			}
			cmd.Start()
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
		}
	})
}

func GetInfo() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodGet:
			cmd := bulb(r, "status")
			if cmd == nil {
				http.Error(w, "400 Bad Request", http.StatusBadRequest)
				return
			}

			output, err := cmd.Output()
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}

			info := parseInfo(string(output))
			infoJson, err := json.Marshal(info)
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}
			w.Write(infoJson)
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
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
