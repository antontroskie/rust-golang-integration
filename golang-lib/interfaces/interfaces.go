package interfaces

type Command int

const (
	Shutdown Command = 0
)

type Message struct {
	Command *Command `json:"command"`
	Msg     *string  `json:"msg"`
}
