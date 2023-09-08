# Future Utils Crate

![Rust](https://img.shields.io/badge/Rust-1.55+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A collection of asynchronous/future utilities for Rust.

## Features

- **Blocking Execution of Asynchronous Futures:** This crate provides a utility function, `block_on`, that allows you to execute asynchronous code synchronously when necessary. This is only available with the `block_on` feature flag enabled.

## Usage

To use this crate in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
future_utils = "0.1.0"
```

### Blocking Execution of Asynchronous Futures

The block_on function is used to block the current thread and wait for the provided asynchronous future to complete its execution, returning the result once it's finished.

#### Example

```rs
use future_utils::block_on;

async fn my_async_function() -> i32 {
    // Some asynchronous computation
    42
}

fn main() {
    let result = block_on(my_async_function());
    println!("Result: {}", result);
}
```

Warning: Blocking on asynchronous code should be used with caution, as it can lead to performance issues and should generally be avoided when possible. Consider alternative approaches like using async functions and awaiting the results in an asynchronous context whenever feasible.

## Contributing

As this crate is still in very early development, proper contribution guidelines have not been established.

## License

This crate is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License 2.0](LICENSE-APACHE). You may choose either of them when using this crate.

- The MIT License: [LICENSE-MIT](LICENSE-MIT) or [here](https://opensource.org/licenses/MIT)
- The Apache License 2.0: [LICENSE-APACHE](LICENSE-APACHE) or [here](https://www.apache.org/licenses/LICENSE-2.0)
