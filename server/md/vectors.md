##  Vectors

If you want to have a flexible array, then use vectors

```rust
let mut xs: Vec<i32> = vec![1, 2, 3, 4, 5];
xs.push(42);
println!("first element of the vector: {}", xs[0]);
println!("second element of the vector: {}", xs[1]);
println!("i just pushed somthing at index5 which is: {}", xs[5]);
println!("vector size: {}", xs.len());
```
