# Variable mutability

```rust
fn main(){
    let x: u32 = 1;
    x = 10;
    println!("The value of x is {}", x);
}
```

```sh
$>cargo run
 error: re-assignment of immutable variable `x`
             x = 10;
             ^~~~~~
```

```rust
fn main(){
    let mut x: u32 = 1;
    x = 10;
    println!("The value of x is {}", x);
}
```

```sh
$>cargo run
 The value of x is 10

```

