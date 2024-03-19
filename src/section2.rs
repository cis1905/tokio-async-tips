// Section 2: Async Building Blocks

// So now we know what a Future is (roughly, we can do an optional deep dive on that later),
//  we can think about how to build applications with them.
// Should we build a new Future any time we ever want to do something? Absolutely not!
//  Libraries will have many Futures already made that you can use to build up your applications.
// Here's a loop where it sleeps, for example.
pub async fn sleepy_loop() {
    println!("starting!");
    for i in 0..5 {
        // Similar to the `sleep` from std, this takes a Duration and suspends your code.
        // The diffference is that with this, other code in your process can run while it's `await`ing its turn.
        // What happens if you remove the .await? What do you think happens?
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        println!("{i}");
    }
}

#[tokio::test]
async fn test_sleepy_loop() {
    sleepy_loop().await;
}

// Crates that might be of interest for getting pre-made Futures to use:
// your executor (in this case tokio)
// reqwest: HTTP requests
// futures: combining a bunch of other futures together (we'll cover this next)

// Combining Futures
// If you want to wait for multiple Futures to end, you'll have to decide what you want to happen.
// If you want to get the response from the first one, use select from tokio or the futures crate
// https://docs.rs/tokio/latest/tokio/macro.select.html#examples
// https://docs.rs/futures/latest/futures/future/fn.select.html

// Try using select to get the one that finishes first.
#[tokio::test]
async fn test_select() {
    let mut fut1 = async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        1
    };
    let mut fut2 = async {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        2
    };
    todo!("This function should use select to wait for the first one of these")
}

// If you want to get all the responses, you can use the join from tokio or the futures crate
// https://docs.rs/tokio/latest/tokio/macro.join.html#examples
// https://docs.rs/futures/latest/futures/future/fn.join.html

// Try joining the two futures here, or construct your own futures!
#[tokio::test]
async fn test_join() {
    let mut fut1 = async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        1
    };
    let mut fut2 = async {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        2
    };
    todo!("This function should use join to wait for both of these")
}

// The futures crate also lets you choose how the results should be collected with FuturesUnordered and FuturesOrdered
// On top of that, it has utilities for describing Futures that happen one after another
// One of these is an async-style iterator called Stream.
// The other is an api like the Promises that you might find in Javascript, where you can chain methods to describe
//  what happens after each future with .then
