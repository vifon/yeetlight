package api

import (
	"io/fs"
	"net/http"
)

// PowerOn turns the bulb on or off.
func PowerOn(power bool) http.Handler {
	var state string
	if power {
		state = "on"
	} else {
		state = "off"
	}
	return CallMethod(
		"set_power",
		ConstParam{state},
		ConstParam{"smooth"},
		ConstParam{500},
	)
}

// SetBrightness sets the brightness of a bulb.
func SetBrightness() http.Handler {
	return CallMethod(
		"set_bright",
		MapParam{
			func(c interface{}) interface{} {
				if c.(int64) == 0 {
					return 1
				} else {
					return c
				}
			},
			NewNumParam(QueryParam{"brightness"}),
		},
		ConstParam{"smooth"},
		ConstParam{500},
	)
}

// SetTemperature sets the color temperature of a bulb.
func SetTemperature() http.Handler {
	return CallMethod(
		"set_ct_abx",
		NewNumParam(QueryParam{"temperature"}),
		ConstParam{"smooth"},
		ConstParam{500},
	)
}

// SetColor sets the color of a bulb.
func SetColor() http.Handler {
	return CallMethod(
		"set_rgb",
		NewNumParamWithBase(QueryParam{"rgb"}, 16),
		ConstParam{"smooth"},
		ConstParam{500},
	)
}

// Config serves the config file.
func Config(config string) http.Handler {
	return Get(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// "config" is provided by the server operator, not read from
		// the request URL, so it should be safe to just feed it to
		// http.ServeFile.
		http.ServeFile(w, r, config)
	}))
}

// Handle sets up all the HTTP handlers.
func Handle(static fs.FS, config string) {
	http.Handle("/", WithLogging(http.FileServer(http.FS(static))))
	if len(config) > 0 {
		http.Handle("/config.json", WithLogging(Config(config)))
	}

	http.Handle("/on", WithLogging(PowerOn(true)))
	http.Handle("/off", WithLogging(PowerOn(false)))
	http.Handle("/brightness", WithLogging(SetBrightness()))
	http.Handle("/temperature", WithLogging(SetTemperature()))
	http.Handle("/color", WithLogging(SetColor()))
	http.Handle("/info", WithLogging(GetInfo(
		"power", "bright", "ct", "rgb", "color_mode",
	)))
	http.Handle("/toggle", WithLogging(Toggle()))
}
