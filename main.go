package main

import (
	"embed"
	"flag"
	"fmt"
	"io/fs"
	"log"
	"net/http"
	"os/exec"

	"github.com/vifon/yeetlight/api"
)

//go:embed public/*
var content embed.FS

func Browse(iface string) {
	exec.Command("xdg-open", "http://" + iface).Run()
}

func main() {
	browse := flag.Bool("browse", false, "Launch a browser.")
	iface := flag.String("iface", "0.0.0.0:8080", "Network interface to bind to.")
	config := flag.String("config", "", "Path to the config.")
	flag.Parse()

	static, err := fs.Sub(content, "public")
	if err != nil {
		log.Panic(
			"The embedded FS has no 'public' directory; it should never happen, please report!",
		)
	}

	api.Handle(static, *config)

	fmt.Printf("Serving at http://%v\n", *iface)
	if browse != nil && *browse == true {
		go Browse(*iface)
	}
	log.Fatal(http.ListenAndServe(*iface, nil))
}
