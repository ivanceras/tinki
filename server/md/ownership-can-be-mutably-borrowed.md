# Ownership can be borrowed mutably

```rust
fn main(){
    let mut x:String = "hello".to_string();
    println!("x: {}", x);
    append_world(x);
    println!("x: {}", x);//we got x back
}

fn append_world(x: &mut String){
    x.push_str(" world");
}

```
