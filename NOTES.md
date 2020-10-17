# Notes

Okay so now we have it parsed and handling functions in the form 

```rust
pipe!(expr, callable(x,y,x,...), ....);
```

but we don't handle cases where the top level args aren't callable or are methods.

We can accept these by parsing to the Callable enum but that doesn't solve how
we figure out where the `expr` goes in the args.

I think if we handle this on a case by case basic it should work. ie. first
methods are tackled then we handle chained methods. After that the `try` 
operator should be usable with the macro as which point it is released.