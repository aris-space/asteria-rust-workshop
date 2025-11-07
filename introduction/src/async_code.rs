// =============================================
// Section 5: async
// =============================================

// The CPU often has to wait for the operating system.
// It would be nice if it could do work in the meantime.
// For this, the OS offers non-blocking interactions.

// However, then we always need to handle the case where the OS isn't ready yet.
// Introducing: `Futures`

// Future is a trait, that says that this type represents a computation.
// That computation can be paused, e.g., if the OS isn't ready yet.
// Because this is hard to represent in code, Rust has nice syntax for it, using `async` and `await`.

#[cfg(test)]
use std::time::Duration;

/// This creates a computation to print, and then wait for the printing to be completed
async fn print_something() {
    use tokio::io::AsyncWriteExt as _;
    let mut console = tokio::io::stdout();
    console
        .write_all(b"Hello world\n")
        .await
        .expect("failed to write");
}

/// If we now use print_something in normal code, we have no way of running the computation.
#[test]
fn sync_print_something() {
    let _future = print_something();
}

/// We need an `executor`, in the most common case [`tokio`].
/// Then we can run it using `await`.
#[tokio::test]
async fn test_print_something() {
    let future = print_something();
    future.await;
}

// So far, we haven't really gained anything, but it allows some very interesting stuff.

/// You can easily do stuff at the same time (concurrently actually)
#[tokio::test]
async fn test_print_something_twice() {
    // This will be started at the same time
    tokio::join!(print_something(), print_something());
}

/// And also in parallel, if you spawn tasks
#[tokio::test]
async fn test_background_task() {
    // Spawned tasks will just run in the background
    tokio::spawn(async {
        loop {
            print_something().await;
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    // While we do other "work"
    println!("A");
    tokio::time::sleep(Duration::from_millis(600)).await;
    println!("B");
    tokio::time::sleep(Duration::from_millis(600)).await;
    println!("C");
    tokio::time::sleep(Duration::from_millis(600)).await;
}

/// You can also easily cancel tasks
#[tokio::test]
async fn test_timeout() {
    async fn maybe_frozen_computation() {
        tokio::time::sleep(Duration::from_millis(200)).await;
        println!("done");
    }

    assert!(
        tokio::time::timeout(Duration::from_millis(100), maybe_frozen_computation())
            .await
            .is_err(),
        "100ms is not enough"
    );
    // The computation was cancelled
    assert!(
        tokio::time::timeout(Duration::from_millis(300), maybe_frozen_computation())
            .await
            .is_ok(),
        "300ms is enought"
    );
    // but completed now
}

// async also brings in some complexities, as with multiple threads, more bugs could occur,
// and rust's type system prevents some of them.
// The rule is then: use owned variables, not references.
// This is exactly opposite of synchronous Rust, and is sparking some critisism.

// But in most cases, what we learned now is enough.

// Now let's move to the last topic, lets give the main function some access to something we did:
pub struct MyPublicStruct {
    pub public_field: i32,
    private_field: u16,
}

impl MyPublicStruct {
    pub fn new(public_field: i32) -> Self {
        Self {
            public_field,
            private_field: 0,
        }
    }
}
