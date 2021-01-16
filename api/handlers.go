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
	return CallMethod("set_power", ConstParam{state})
}

func SetBrightness() http.Handler {
	return CallMethod(
		"set_bright",
		MapParam{
			func(c interface{}) interface{} {
				if c.(int) == 0 {
					return 1
				} else {
					return c
				}
			},
			NumParam{QueryParam{"brightness"}},
		},
	)
}

func SetTemperature() http.Handler {
	return CallMethod(
		"set_ct_abx",
		NumParam{QueryParam{"temperature"},},
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
	http.Handle("/info", WithLogging(GetInfo("power", "bright", "ct")))
}
