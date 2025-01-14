// As before, you may find these instructions easier to read in Rustdoc. You can generate
// the docs by running `cargo doc --no-deps --open` and navigating to this page.

//! P1: Async file read
//!
//! Rust supports async/await-style programming through the [`Future`] trait. In this problem, you
//! will implement a future for asynchronously reading a file. See the `read_test` function below
//! for an example of how such a usage of the future would look.
//!
//! The basic strategy is like this: given a `file` of type [`File`], then `file.read_async()` will return
//! a data structure [`ReadFile`] that represents a future, i.e. at some point it will return the bytes
//! that are read. The [`ReadFile`] future should launch a system thread which reads the file into a buffer.
//! When the thread is done, then [`Future::poll`] should return [`Poll::Ready`].
//!
//! Note that the future is responsible for "waking" itself once the thread has completed. For an example of
//! how to do this, see the Rust Async Book: <https://rust-lang.github.io/async-book/02_execution/03_wakeups.html>
//!
//! Your task is to implement the [`ReadFile`] data type and methods, specifically [`AsyncFile::read_async`] and
//! [`Future::poll`]. You can run `cargo test read` to check your solution.
//!
//! Beware: your design MUST not allow the promise to live longer than the `File` that it holds! You can double
//! check this is true by uncommenting `read_bad_scope_test` below, and ensuring it does not compile.

use std::{fs::File, future::Future, io, marker::PhantomData, pin::Pin, task::{Context, Poll}, thread};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::task::Waker;


/// Extension trait for asynchronous methods on [`File`].
pub trait AsyncFile {
    /// The type of the future returned by `read_async`.
    type ReadFuture<'a>: Future<Output = io::Result<Vec<u8>>>
    where
        Self: 'a;

    /// Asynchronously reads all of a file's contents into a buffer.
    fn read_async<'a>(&'a mut self) -> Self::ReadFuture<'a>;
}

/// The file reading future.
pub struct ReadFile<'a> {
    _marker: PhantomData<&'a ()>,
    shared_state: Arc<Mutex<SharedState>>
}

pub struct SharedState {
    file_data: Option<Vec<u8>>,
    ready: bool,
    waker: Option<Waker>
}

// This impl constructs the future when the user calls `file.read_async()`.
impl AsyncFile for File {
    type ReadFuture<'a> = ReadFile<'a>;

    fn read_async<'a>(&'a mut self) -> ReadFile<'a> {
        let shared_state = Arc::new(Mutex::new(SharedState {
            file_data: None,
            ready: false,
            waker: None
        }));

        let thread_shared_state = shared_state.clone();
        let mut file_copy = self.try_clone().unwrap();

        thread::spawn(move || {
            let mut shared_state = thread_shared_state.lock().unwrap();
            let mut buf = vec![];

            match file_copy.read_to_end(&mut buf) {
                Ok(_) => {
                    shared_state.file_data = Some(buf);
                    shared_state.ready = true;
                    if let Some(waker) = shared_state.waker.take() {
                        waker.wake()
                    }
                },
                Err(_) => panic!("Failed to read the file")
            }
        });

        ReadFile { _marker: Default::default(), shared_state }
    }
}

// This impl polls the future for completion, returning the value inside if it's ready.
impl<'a> Future for ReadFile<'a> {
    type Output = io::Result<Vec<u8>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let cloned_shared_state = self.shared_state.clone();
        let mut shared_state = cloned_shared_state.lock().unwrap();
        if shared_state.ready {
            match &shared_state.file_data {
                Some(v) => {
                    Poll::Ready(Ok(v.clone()))
                }
                None => {
                    Poll::Ready(Ok(vec![]))
                }
            }

        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn read_test() {
        let path = std::env::temp_dir().join("foo.txt");
        let contents = "hello world";
        fs::write(&path, contents).unwrap();
        let mut file = File::open(&path).unwrap();
        let buf = file.read_async().await.unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), contents);
    }

    // #[tokio::test]
    // async fn read_bad_scope_test() {
    //   fs::write("foo.txt", "hello world").unwrap();
    //   let future = {
    //     let mut file = File::open("foo.txt").unwrap();
    //     file.read_async()
    //   };
    //   let buf = future.await.unwrap();
    //   assert_eq!(String::from_utf8(buf).unwrap(), "hello world");
    // }
}
