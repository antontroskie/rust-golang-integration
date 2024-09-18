use std::{ffi::CStr, sync::Mutex};

// Create singleton channel
static RUST_SENDER: Mutex<Option<crossbeam::channel::Sender<String>>> = Mutex::new(None);
static RUST_RECEIVER: Mutex<Option<crossbeam::channel::Receiver<String>>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn get_receiver() -> *mut libc::c_void {
    let receiver = RUST_RECEIVER.lock().unwrap();
    let receiver = receiver.as_ref().unwrap();
    receiver as *const _ as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn get_sender() -> *mut libc::c_void {
    let sender = RUST_SENDER.lock().unwrap();
    let sender = sender.as_ref().unwrap();
    sender as *const _ as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn send_to_channel(message: *const libc::c_char) {
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message = message_cstr.to_str().unwrap();
    let sender = RUST_SENDER.lock().unwrap();
    let sender = sender.as_ref().unwrap();
    sender.send(message.to_string()).unwrap();
}

#[no_mangle]
pub extern "C" fn create_channels() {
    let (s, r) = crossbeam::channel::unbounded::<String>();
    let mut sender = RUST_SENDER.lock().unwrap();
    let mut receiver = RUST_RECEIVER.lock().unwrap();
    *sender = Some(s);
    *receiver = Some(r);
}
