# Primitive types, Memory safety & zero cost abstractions

Length | Signed | Unsigned
-------|--------|---------
8-bit  |  i8    | u8
16-bit |  i16   | u16
32-bit |  i32   | u32
64-bit |  i64   | u64
128-bit|  i128  | u128
arch   |  isize | usize

```rust
fn main(){
    //integers
    let i: i8 = 1; //i16, i32, i64, and isize are available
    //unsigned
    let u: u8 = 2; //u16, u32, u64, and usize are available
    //floats
    let f: f32 = 1.0; // f64 also available
    //boleans
    let b: bool = true; //false also availabele, duh
    //string and characters
    let c: char = 'a';
    let s: &str = "hello world";
}
```
