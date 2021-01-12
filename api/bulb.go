package api

import (
	"net/http"
	"os/exec"
)

func bulb(r *http.Request, args ...string) *exec.Cmd {
	bulb := r.URL.Query().Get("bulb")
	if len(bulb) == 0 {
		return nil
	}
	args = append([]string{"--ip", bulb}, args...)
	return exec.Command("yee", args...)
}
