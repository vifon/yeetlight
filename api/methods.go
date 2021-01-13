package api

import (
	"encoding/json"
	"net/http"

	"einval.eu/yeetlight/bulb"
)

func CallMethod(property string, params... Param) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			addr := r.URL.Query().Get("bulb")
			if len(addr) == 0 {
				http.Error(w, "400 Bad Request", http.StatusBadRequest)
				return
			}

			methodParams := make([]interface{}, len(params))
			for i := range params {
				value, err := params[i].Get(r)
				if err != nil {
					http.Error(w, "400 Bad Request", http.StatusBadRequest)
					return
				}
				methodParams[i] = value
			}

			b := bulb.Bulb{Addr: addr}
			cmd := bulb.NewCommand(1, property, methodParams...)
			resp, err := b.Send(cmd)
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}
			parsed, err := resp.Decode()
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}

			if parsed.Success() == false {
				w.WriteHeader(http.StatusBadRequest)
			}
			w.Write(resp)
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
		}
	})
}

func GetInfo() http.Handler {
	requestedProps := []interface{}{"power", "bright", "ct"}

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodGet:
			addr := r.URL.Query().Get("bulb")
			if len(addr) == 0 {
				http.Error(w, "400 Bad Request", http.StatusBadRequest)
				return
			}

			b := bulb.Bulb{Addr: addr}
			cmd := bulb.NewCommand(1, "get_prop", requestedProps...)
			resp, err := b.Send(cmd)
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}
			parsed, err := resp.Decode()
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}

			if len(parsed.Result) != len(requestedProps) {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}

			result := make(map[string]string)
			for i := range requestedProps {
				result[requestedProps[i].(string)] = parsed.Result[i]
			}
			resultJson, err := json.Marshal(result)
			if err != nil {
				http.Error(w, "500 Internal Server Error", http.StatusInternalServerError)
				return
			}

			w.Write(resultJson)
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusNotFound)
		}
	})
}
