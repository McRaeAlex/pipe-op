# Pipe Operator

Implementation of the pipe operator in Rust as a macro.

I wrote this because of 3 main things:

1. You cannot create methods on out of crate types. This means that chaining is harder.
2. [pipe_trait](https://crates.io/crates/pipe-trait) is nice but it requires currying for multi argument functions.
3. I want to write a plug like HTTP server

```rust
pipe!(
    value,
    function1(args),
    function2(args)?, // notice the operator here
    function3(args)
)
```

The pipe operator is common in languages like elixir and haskell, they provide clarity and convience.

```elixir
conn
|> send_resp(404, "Not found")
```

## Examples (treated as test cases)

The argument will always go into the first function or method call.

```rust
pipe!(value, a.f().m()) // is equivalent to a.f(x).m()
```

