# Tuples

A tuple is a collection of values of different types. Expressed in the form of (T1, T2, ...)

```rust
let (a,b) = (1,2);
println!("a:{}", a);
println!("b:{}", b);
```

## Swapping variables using types

```rust
fn main() {
  let a = 1;
  let b = 2;
  println!("a: {}",a);
  println!("b: {}",b);
  let (a, b) = (b, a); //code for swapping
  println!("swaped a: {}",a);
  println!("swaped b: {}",b);

}
```

```sh
$>cargo run
a: 1
b: 2
swaped a: 2
swaped b: 1
```
<img src="troll-face.svg" alt="troll" style="width: 200px;"/>
