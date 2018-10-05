# Ownership is singular

```rust
fn main(){
    let x:String = "hello".to_string();
    println!("x: {}", x);
    append_world(x);//x is transferred to append_world function
    //println!("x: {}", x);//x no longer exist here
  }

fn append_world(mut x: String){
    x.push_str(" world");
    println!("{}",x);

}
```
