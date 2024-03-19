// Section 1: Async Basics
// In this file, we'll learn what a Future is and how we start making a program out of it.

// Let's visualize what a future is first.
//  A while ago, we had an enum in an exercise where we modeled the
//  transition of a traffic light as it changed color.
pub enum TrafficLightFuture {
    Green,
    Yellow,
    Red,
}
impl TrafficLightFuture {
    pub fn next_state(&mut self) {
        match self {
            TrafficLightFuture::Green => *self = TrafficLightFuture::Yellow,
            TrafficLightFuture::Yellow => *self = TrafficLightFuture::Red,
            TrafficLightFuture::Red => {} // suppose we want it to stay stopped
        }
    }
}
// Let's say that we wanted to do that, except for any arbitrary state machine.
//  So we define a trait for it!
//  std does this, but it looks a little more complex.
//  We've simplified the trait to get the basics down
pub trait SimpleFuture {
    type Output;
    // returns None if it's still doing stuff, and returns Output if it's done.
    fn poll(&mut self) -> Option<Self::Output>;
}

// So now what do we do with it? Well, the point of a state machine is that we
//  can poll it to progress its state if we can. We call the algorithm
//  that polls futures an executor. Depending on the executor you choose,
//  that will allow you to alternate which state machine is progressed, or
//  even run multiple of them in different threads.

// TODO:
//  Try writing a function to run a bunch of SimpleFuture instances and get the outputs in a Vec.
//  Some ideas to try are:
//   - finish each one in a loop before moving onto the next
//   - alternate between each one and return that
// You *are* allowed to change the mutability to `mut input`. You *are not* allowed to change the input or output types.
pub fn example_round_robin_executor<T>(input: Vec<Box<dyn SimpleFuture<Output = T>>>) -> Vec<T> {
    todo!()
}

// this test should pass when you're done.
#[test]
pub fn test_round_robin() {
    impl SimpleFuture for TrafficLightFuture {
        type Output = ();

        fn poll(&mut self) -> Option<Self::Output> {
            match self {
                TrafficLightFuture::Green => *self = TrafficLightFuture::Yellow,
                TrafficLightFuture::Yellow => *self = TrafficLightFuture::Red,
                TrafficLightFuture::Red => return Some(()),
            }
            None
        }
    }

    let input = vec![
        Box::new(TrafficLightFuture::Green) as Box<dyn SimpleFuture<Output = ()>>,
        Box::new(TrafficLightFuture::Red) as Box<dyn SimpleFuture<Output = ()>>,
        Box::new(TrafficLightFuture::Yellow) as Box<dyn SimpleFuture<Output = ()>>,
    ];
    assert_eq!(&example_round_robin_executor(input), &[(), (), ()]);

    struct CountdownFuture(u32, u32);
    impl SimpleFuture for CountdownFuture {
        type Output = u32;

        fn poll(&mut self) -> Option<Self::Output> {
            self.1 += self.0;
            if self.0 == 0 {
                Some(self.1)
            } else {
                self.0 -= 1;
                None
            }
        }
    }

    let input = vec![
        Box::new(CountdownFuture(4, 0)) as Box<dyn SimpleFuture<Output = u32>>,
        Box::new(CountdownFuture(2, 0)) as Box<dyn SimpleFuture<Output = u32>>,
        Box::new(CountdownFuture(3, 0)) as Box<dyn SimpleFuture<Output = u32>>,
    ];
    assert_eq!(&example_round_robin_executor(input), &[10, 3, 6]);
}

// okay so now we have an idea of what an async program might look like
pub fn fake_main() {
    // 1. make a series of tasks that we can do at once

    // 2. set up an executor to poll them with the strategy we want

    // 3. wait for that to be done

    // 4. base your whole program's control flow around that! (we'll get to that)
}

// When we follow these steps, at least the first 3 will be helped by making an async function to run our code.
pub async fn foo() {}
// this function reads roughly the same as
pub fn bar() -> impl std::future::Future {
    async {}
}

// and then to use it, you call it like normal and then `await` the result
pub async fn baz() {
    foo().await;
    bar().await;
}

// but then we need something to run the future that we have at the top level.
// Most async programs use a pre-defined executor from a crate.
//  And the most common crate for that is tokio.
//  tokio includes a multi-threaded and a single-threaded executor
//  and it's optimized for general use cases.
// Other executors from other crates make different tradeoffs like:
//  - being way smaller at the cost of possibly not being as good at balancing work (smol)
//  - working when there isn't an operating system, but without any of the nice features of building on top of an OS (embassy)
//  - working with an alternative ecosystem to tokio (async-std) (this one is less common, since keeping everything within tokio is way more convenient)

// Normally you'd have to have to use the crate's api to build up a
pub fn manual_main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
        })
}
// but tokio's main macro lets us skip that and just write what we *would* write in the async block as our function

#[tokio::main]
// this function should just be named `main`. I write macro_main here because we already have a `main` in this project.
// otherwise, this function (with the macro attached) is the same as the one above
pub async fn macro_main() {
    println!("Hello, world!");
}

// similarly, there's a tokio test macro
#[tokio::test]
pub async fn test_the_test_macro() {}
