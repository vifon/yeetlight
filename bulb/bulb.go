package bulb

import (
	"encoding/json"
	"log"
	"net/textproto"
)

type Bulb struct {
	Addr string
}

func (b Bulb) Connect() (*textproto.Conn, error) {
	return textproto.Dial("tcp", b.Addr + ":55443")
}

func (b Bulb) Send(c Command) (resp Response, err error) {
	conn, err := b.Connect()
	if err != nil {
		return
	}
	defer conn.Close()

	rawCommand, err := json.Marshal(c)
	if err != nil {
		return
	}
	log.Printf("Sending: %s", rawCommand)
	conn.PrintfLine("%s", rawCommand)

	resp, err = conn.ReadLineBytes()
	log.Printf("Received: %s", resp)

	return
}
