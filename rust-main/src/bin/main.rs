use rust_golang_integration::{
    bindings::{get_receiver, SendToMessageChannel, StartGolang},
    interfaces::{Command, Message},
};

#[tokio::main]
async fn main() {
    // Spawn Golang thread
    unsafe {
        StartGolang();
    }

    // Get receiver channel
    let receiver: crossbeam::channel::Receiver<String> = unsafe {
        let receiver = get_receiver();
        let receiver = receiver as *const crossbeam::channel::Receiver<String>;
        let receiver = &*receiver;
        receiver.clone()
    };

    // Spawn Tokio thread that will send a message to Golang
    let send_loop = tokio::spawn(async move {
        // Send message to channel 5 times
        for i in 0..5 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            unsafe {
                SendToMessageChannel(
                    serde_json::json!(Message {
                        command: None,
                        msg: Some(format!("message number {}", i))
                    })
                    .to_string()
                    .into(),
                );
            }
        }
        // Send quit signal
        unsafe {
            SendToMessageChannel(
                serde_json::json!(Message {
                    command: Some(Command::Shutdown),
                    msg: None
                })
                .to_string()
                .into(),
            );
        }
    });

    // Spawn Tokio thread that will receive a message from Golang
    let receive_loop = tokio::spawn(async move {
        // Receive message from channel
        loop {
            let msg = receiver.recv().unwrap();
            let message = serde_json::from_str::<Message>(&msg).unwrap();
            if let Some(message) = message.msg {
                println!("Received message from Golang lib: {}", message);
            }
            if let Some(command) = message.command {
                println!("Received command from Golang lib: {:?}", command);
                if matches!(command, Command::Shutdown) {
                    return;
                }
            }
        }
    });

    send_loop.await.unwrap();
    receive_loop.await.unwrap();

    println!("Rust main has finished");
}
