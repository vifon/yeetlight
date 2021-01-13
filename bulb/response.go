package bulb

import "encoding/json"

type Response []byte

func (r Response) Decode() (p ParsedResponse, err error) {
	err = json.Unmarshal(r, &p)
	return
}

type ParsedResponse struct {
	Id int `json:"id"`
	Result []string `json:"result"`
}

func (p ParsedResponse) Success() bool {
	return len(p.Result) == 1 && p.Result[0] == "ok"
}
