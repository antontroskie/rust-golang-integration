package handlers

import (
	"encoding/json"
	"fmt"

	"github.com/antontroskie/golang-lib/interfaces"
)

func HandleMessage(msg string, quitSignal chan int, sendToFeedbackChannel func(msg string)) {
	var u interfaces.Message
	err := json.Unmarshal([]byte(msg), &u)
	if err != nil {
		println(fmt.Sprintf("Error unmarshalling message: %s", err))
	}
	println(fmt.Sprintf("Golang lib received message: %s", msg))
	if u.Command != nil {
		switch *u.Command {
		case interfaces.Shutdown:
			println("Golang lib received shutdown command")
			quitSignal <- int(interfaces.Shutdown)
		default:
			println(fmt.Sprintf("Golang lib received unknown command: %d", *u.Command))
		}
		return
	} else if u.Msg != nil {
		sendToFeedbackChannel(*u.Msg)
	}
}
