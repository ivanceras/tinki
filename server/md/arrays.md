# Arrays

Arrays size is known at compile time, therefore immutable at all times.

```rust
let xs: [i32; 5] = [1, 2, 3, 4, 5];
println!("first element of the array: {}", xs[0]);
println!("second element of the array: {}", xs[1]);
println!("array size: {}", xs.len());
```
