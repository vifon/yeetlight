package api

import (
	"encoding/json"
	"net/http"
)

func TurnOn() http.Handler {
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
func TurnOff() http.Handler {
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

func SetProperty(property string, queryItem string) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			cmd := bulb(r, property, r.URL.Query().Get(queryItem))
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
	http.Handle("/", WithLogging(http.FileServer(http.Dir("./public"))))
	http.Handle("/on", WithLogging(TurnOn()))
	http.Handle("/off", WithLogging(TurnOff()))
	http.Handle("/brightness", WithLogging(SetProperty("brightness", "brightness")))
	http.Handle("/temperature", WithLogging(SetProperty("temperature", "temperature")))
	http.Handle("/info", WithLogging(GetInfo()))
}
