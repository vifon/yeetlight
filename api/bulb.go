package api

import (
	"encoding/json"
	"net/http"

	"github.com/vifon/yeetlight/bulb"
)

// CallMethod handles the requests that sets a given smart light
// property according to the given parameters.  It's the caller's job
// to make sure they conform to the Yeelight API.
//
// The bulb address is specified in the "bulb" query parameter.
func CallMethod(property string, params... Param) http.Handler {
	return Post(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		addr := r.URL.Query().Get("bulb")
		if len(addr) == 0 {
			http.Error(w, "No bulb address", http.StatusBadRequest)
			return
		}

		methodParams := make([]interface{}, len(params))
		for i := range params {
			value, err := params[i].Get(r)
			if err != nil {
				http.Error(w, err.Error(), http.StatusBadRequest)
				return
			}
			methodParams[i] = value
		}

		b := bulb.Bulb{Addr: addr}
		cmd := bulb.NewCommand(1, property, methodParams...)
		resp, err := b.Send(cmd)
		if err != nil {
			http.Error(w, "Cannot send message", http.StatusInternalServerError)
			return
		}
		parsed, err := resp.Decode()
		if err != nil {
			http.Error(w, "Cannot decode response", http.StatusInternalServerError)
			return
		}

		if parsed.Success() == false {
			w.WriteHeader(http.StatusBadRequest)
		}
		w.Write(resp)
	}))
}

// GetInfo serves the info about the requested properties of a bulb.
//
// The bulb address is specified in the "bulb" query parameter.
func GetInfo(requestedProps... interface{}) http.Handler {
	return Get(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		addr := r.URL.Query().Get("bulb")
		if len(addr) == 0 {
			http.Error(w, "No bulb address", http.StatusBadRequest)
			return
		}

		b := bulb.Bulb{Addr: addr}
		cmd := bulb.NewCommand(1, "get_prop", requestedProps...)
		resp, err := b.Send(cmd)
		if err != nil {
			http.Error(w, "Cannot send message", http.StatusInternalServerError)
			return
		}
		parsed, err := resp.Decode()
		if err != nil {
			http.Error(w, "Cannot decode response", http.StatusInternalServerError)
			return
		}

		if len(parsed.Result) != len(requestedProps) {
			http.Error(w, "Response invalid: bad array length", http.StatusInternalServerError)
			return
		}

		result := make(map[string]string)
		for i := range requestedProps {
			result[requestedProps[i].(string)] = parsed.Result[i]
		}
		resultJson, err := json.Marshal(result)
		if err != nil {
			http.Error(w, "Cannot encode response", http.StatusInternalServerError)
			return
		}

		w.Write(resultJson)
	}))
}

// Toggle turns the bulb on and off
//
// The bulb address is specified in the "bulb" query parameter.
func Toggle() http.Handler {
	return Post(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		addr := r.URL.Query().Get("bulb")
		if len(addr) == 0 {
			http.Error(w, "No bulb address", http.StatusBadRequest)
			return
		}

		b := bulb.Bulb{Addr: addr}
		cmd := bulb.NewCommand(1, "get_prop", "power")
		resp, err := b.Send(cmd)
		if err != nil {
			http.Error(w, "Cannot send message", http.StatusInternalServerError)
			return
		}
		parsed, err := resp.Decode()
		if err != nil {
			http.Error(w, "Cannot decode response", http.StatusInternalServerError)
			return
		}

		state := parsed.Result[0] == "on"
		PowerOn(!state).ServeHTTP(w, r)
	}))
}
