# Notes

This is actually now solved but needs testing an to fix a lifetime error.

The error:

```rust
let temp = inital_value;
let tacker = None;

loop {
    match temp {
        Value1 => {
            tracker = Some(Enum::Var1(&mut value));
            break;
        },

        Value2 => {
            tracker = Some(Enum::Var2(&mut value));
            // obviously i cannot mutably borrow value here because I already borrowed the value
            temp = value.field.as_mut();
        },
        _ => {break;},
    }
}

// mutate the tracker
```

How do I rework this such that I don't need to borrow mutably twice.

I suppose if we ran left to right I could just choose the first one but how does
one convert it such that we go left to right.

