#![feature(
    arbitrary_self_types,
    async_await,
    await_macro,
    generator_trait,
    generators,
    proc_macro_hygiene
)]

use core::future::Future;

use embrio_async::{async_block, async_fn, async_stream_block, await};
use ergo_pin::ergo_pin;
use futures::{executor::block_on, stream::StreamExt};
use futures_test::future::FutureTestExt;

#[test]
fn smoke() {
    let future = async_block! {
        await!(async { 5 }.pending_once())
    };
    assert_eq!(block_on(future), 5);
}

#[test]
#[ergo_pin]
fn smoke_stream() {
    let mut stream = pin!(async_stream_block! {
        yield await!(async { 5 }.pending_once());
        yield await!(async { 6 }.pending_once());
    });
    assert_eq!(block_on(stream.next()), Some(5));
    assert_eq!(block_on(stream.next()), Some(6));
    assert_eq!(block_on(stream.next()), None);
}

#[derive(Eq, PartialEq, Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[async_fn]
fn a_number_and_string<'a>(n: &usize, s: &'a str) -> Either<usize, &'a str> {
    if *n % 2 == 0 {
        Either::Left(*n)
    } else {
        Either::Right(s)
    }
}

#[async_fn]
fn a_wait_thing() -> Either<usize, &'static str> {
    await!(a_number_and_string(&5, "Hello, world!"))
}

#[async_fn]
fn anonymous_lifetime(f: &mut core::fmt::Formatter<'_>) {
    let _ = write!(f, "Hello, world!");
}

#[test]
fn smoke_async_fn() {
    assert_eq!(block_on(a_wait_thing()), Either::Right("Hello, world!"));
}