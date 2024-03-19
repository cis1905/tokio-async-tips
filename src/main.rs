// Almost every async program that you see in Rust will start
//  a little something like this.
//  What are these extra parts? How do they work? Why do we need them?
//  This exercise will walk through all the steps of that.
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
// move to each section in order to see the explanations
pub mod section1;
pub mod section2;
pub mod section3;
pub mod section4;
