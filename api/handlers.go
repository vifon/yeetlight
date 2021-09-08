package api

import (
	"io/fs"
	"io/ioutil"
	"net/http"
)

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

func SetTemperature() http.Handler {
	return CallMethod(
		"set_ct_abx",
		NewNumParam(QueryParam{"temperature"}),
		ConstParam{"smooth"},
		ConstParam{500},
	)
}

func SetColor() http.Handler {
	return CallMethod(
		"set_rgb",
		NewNumParamWithBase(QueryParam{"rgb"}, 16),
		ConstParam{"smooth"},
		ConstParam{500},
	)
}

func Config(config string) http.Handler {
	return Get(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		f, err := ioutil.ReadFile(config)
		if err != nil {
			http.Error(w, "Cannot open the config file", http.StatusInternalServerError)
			return
		}
		w.Write(f)
	}))
}

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
}
