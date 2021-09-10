package bulb

// Command is a a JSON-serializable command in the Yeelight API format.
type Command struct {
	Id int `json:"id"`
	Method string `json:"method"`
	Params []interface{} `json:"params"`
}

// NewCommand creates a new Command.
func NewCommand(id int, method string, params... interface{}) Command {
	return Command{
		Id: id,
		Method: method,
		Params: params,
	}
}
