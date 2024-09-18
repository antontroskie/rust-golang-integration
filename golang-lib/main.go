package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

/*
#cgo LDFLAGS: ./libbridge.a -ldl
#include "./libbridge.h"
*/
import (
	"C"
)

import (
	"encoding/json"
	"fmt"

	"github.com/antontroskie/golang-lib/config"
	"github.com/antontroskie/golang-lib/handlers"
	"github.com/antontroskie/golang-lib/interfaces"
)

var (
	quitSignal     = make(chan int, 1)
	messageChannel = make(chan string, 100)
)

//export StartGolang
func StartGolang() {
	defer func() {
		if r := recover(); r != nil {
			println(fmt.Sprintf("Golang lib panicked: %s", r))
		}
	}()

	// Create reference to feedback channel
	sendToFeedbackChannel := func(msg interfaces.Message) {
		json, err := json.Marshal(msg)
		if err != nil {
			println(fmt.Sprintf("Error marshalling message: %s", err))
		}
		C.send_to_channel(C.CString(string(json)))
	}

	// Create rust feedback channel
	C.create_channels()

	// Print version info
	config.PrintVersionInfo()

	// Continuously read from the channel
	go func() {
		for {
			select {
			case <-quitSignal:
				command := interfaces.Shutdown
				sendToFeedbackChannel(interfaces.Message{Command: &command})
				return // Break out of the outer loop
			case msg := <-messageChannel:
				go handlers.HandleMessage(msg, quitSignal, func(msg string) {
					sendToFeedbackChannel(interfaces.Message{Msg: &msg})
				})
			}
		}
	}()
}

//export SendToMessageChannel
func SendToMessageChannel(msg string) {
	messageChannel <- msg
}

//export QuitGolang
func QuitGolang() {
	println("Golang quit")
	quitSignal <- int(interfaces.Shutdown)
}

func main() {}
