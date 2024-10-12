use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        // do stuff in a child thread
    });

    // do stuff simultaneously in the main thread

    // wait until the thread has exited
    handle.join().unwrap(); // this will pause and wait for the thread we're joining to finish first
}
