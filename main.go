package main

import (
	"embed"
	"flag"
	"fmt"
	"io/fs"
	"log"
	"net/http"

	"einval.eu/yeetlight/api"
)

//go:embed public/*
var content embed.FS

func main() {
	iface := "0.0.0.0:8080"
	config := ""
	flag.StringVar(&iface, "iface", iface, "Network interface to bind to.")
	flag.StringVar(&config, "config", config, "Path to the config.")
	flag.Parse()

	static, err := fs.Sub(content, "public")
	if err != nil {
		log.Panic(
			"The embedded FS has no 'public' directory; it should never happen, please report!",
		)
	}

	api.Handle(static, config)

	fmt.Printf("Serving at http://%v\n", iface)
	log.Fatal(http.ListenAndServe(iface, nil))
}
