package bulb

import "encoding/json"

// Response is a raw smart light response to a command.
type Response []byte

// Decode parses a Response into a ParsedResponse.
func (r Response) Decode() (p ParsedResponse, err error) {
	err = json.Unmarshal(r, &p)
	return
}

// ParsedResponse is an already parsed smart light response.
type ParsedResponse struct {
	Id int `json:"id"`
	Result []string `json:"result"`
}

// Success indicates whether a Command finished without errors.
func (p ParsedResponse) Success() bool {
	return len(p.Result) == 1 && p.Result[0] == "ok"
}
