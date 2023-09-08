mod utils;

use crate::utils::run_executor;
use std::future::Future;

/// Blocking Execution of Asynchronous Futures
///
/// The `block_on` function is used to block the current thread and wait for the provided asynchronous
/// future to complete its execution, returning the result once it's finished.
///
/// # Usage
///
/// You can use this function to execute asynchronous code synchronously when necessary. It is
/// particularly useful in situations where you need to integrate asynchronous code into synchronous
/// code or want to wait for the result of an asynchronous operation before continuing.
///
/// To use `block_on`, simply pass an asynchronous future to it as an argument. The function will block
/// the current thread until the future completes and return its output.
///
/// # Example
/// ```rust
/// use my_async_utils::block_on;
/// use async_std::task;
///
/// async fn my_async_function() -> i32 {
///     // Some asynchronous computation
///     42
/// }
///
/// fn main() {
///     let result = block_on(my_async_function());
///     println!("Result: {}", result);
/// }
/// ```
///
/// # Features
///
/// The `block_on` function is only available when the `"block_on"` feature is enabled. To use it, make
/// sure to include the following in your `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// future_utils = { version = "1.0", features = ["block_on"] }
/// ```
///
/// # Warning
///
/// Blocking on asynchronous code should be used with caution, as it can lead to performance issues
/// and should generally be avoided when possible. Consider alternative approaches like using async
/// functions and awaiting the results in an asynchronous context whenever feasible.
///
/// # Safety
///
/// This function is safe to use in Rust code, but keep in mind that it may introduce blocking behavior
/// and should be used judiciously.
///
#[cfg(feature = "block_on")]
pub fn block_on<F: Future>(f: F) -> F::Output {
    pin_mut!(f);
    run_executor(|cx| f.as_mut().poll(cx))
}
