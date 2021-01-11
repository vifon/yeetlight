package main

import (
	"flag"
	"log"
	"net/http"

	"einval.eu/yeetlight/api"
)

func main() {
	iface := "0.0.0.0:8080"
	flag.StringVar(&iface, "iface", iface, "Network interface to bind to.")
	flag.Parse()

	api.Serve()

	log.Fatal(http.ListenAndServe(iface, nil))
}
