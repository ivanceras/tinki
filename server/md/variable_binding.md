# Variable binding

```rust
fn main(){
    let x: u8 = 1;                      //explicitly declare type
    let y = 2i32;                       //type inference
    let (a, b, c) = (1i32, 2i32, 3i32); //variable declaration via patterns
    let r = [1, 2, 3];                  //array literals
    let s = "hello";                    // string literal
    println!("*_^ x = {}, y = {}, a,b,c = {},{},{}", x,y,a,b,c);
}

```

```sh
$>cargo run
*_^ x = 1, y = 2, a,b,c = 1,2,3
```
