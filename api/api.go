package api

import (
	"encoding/json"
	"net/http"
	"os/exec"
	"regexp"
	"strings"
)

func bulb(r *http.Request, args ...string) *exec.Cmd {
	bulb := r.URL.Query().Get("bulb")
	if len(bulb) > 0 {
		args = append([]string{"--ip", bulb}, args...)
	}
	return exec.Command("yee", args...)
}

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

func parseInfo(info string) map[string]*string {
	lines := strings.Split(info, "\n")

	parsed := make(map[string]*string)
	for _, line := range lines {
		re := regexp.MustCompile(`\* ([^:]+): (.+)`)
		matches := re.FindStringSubmatch(line)
		if matches != nil {
			if matches[2] == "None" {
				parsed[matches[1]] = nil
			} else {
				parsed[matches[1]] = &matches[2]
			}
		}
	}
	return parsed
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

func Serve() {
	http.Handle("/", http.FileServer(http.Dir("./public")))
	http.Handle("/on", TurnOnHandler())
	http.Handle("/off", TurnOffHandler())
	http.Handle("/brightness", Brightness())
	http.Handle("/temperature", Temperature())
	http.Handle("/info", GetInfo())
}
