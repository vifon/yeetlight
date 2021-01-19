package api

import (
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

func Handle() {
	http.Handle("/", WithLogging(http.FileServer(http.Dir("./public"))))
	http.Handle("/on", WithLogging(PowerOn(true)))
	http.Handle("/off", WithLogging(PowerOn(false)))
	http.Handle("/brightness", WithLogging(SetBrightness()))
	http.Handle("/temperature", WithLogging(SetTemperature()))
	http.Handle("/color", WithLogging(SetColor()))
	http.Handle("/info", WithLogging(GetInfo(
		"power", "bright", "ct", "rgb", "color_mode",
	)))
}
