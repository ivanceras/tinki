# Ownership can be transferred back

```rust
fn main(){
    let mut x:String = "hello".to_string();
    println!("x: {}", x);
    x = append_world(x);
    println!("x: {}", x);//we got x back
}

fn append_world(mut x: String)->String{
    x.push_str(" world");
    x
}
```

now THAT sums up to the promised memory safety and data race prevention
