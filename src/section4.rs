// Section 4 (optional): The *Actual* future trait (deep dive)
// If we have time, we can walk through what the actual Future trait looks like on the projector.

// What are these?
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;


pub trait RealFuture {
    type Output;

    // this looks sort of like our original SimpleFuture, but it has some interesting changes
    // fn poll(&mut self                             ) -> Option<Self::Output>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// Pin<&mut Self>
// This type guarantees that (somehow) the type has guaranteed that it will *never*
// move between polls in a way that invalidates pointers
// This is because some futures are self-referential and hold pointers into themselves.
// Types that aren't self-referrential can apply and relax this condition easily.
// It requires a *very* deep dive to explain why Pin exists and how Pin works in practice.
// But the gist is just that it applies an additional constraint onto some references
// that ensures Futures are safe to use.


// Context is like a trait object that tells us how to remind tokio that our future can be polled again.
// It's a hint to make some async computations more responsive or less wasteful.
// Remember that Futures and coroutines describe a *cooperative* multitasking model.
// That means every future tries to be honest so the whole program can go faster!


// Poll is basically just a specialized Option,
// so our original version wasn't actually that far off.
pub enum RealPoll<T> {
    Ready(T),
    Pending,
}