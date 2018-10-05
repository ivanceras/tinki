# Traits

```rust

use std::f64::consts::PI;

struct Circle{
    x:f64,
    y:f64,
    radius:f64,
}

trait HasArea{
    fn area(&self)->f64;
}

impl HasArea for Circle{
    fn area(&self)->f64{
        PI * (self.radius * self.radius)
    }
}

fn main(){
    let c = Circle {x:0.0, y:1.0, radius:2.0};
    println!("The circle's area is {}", c.area());
}

```

```sh
$>cargo run
The circle's area is 12.566370614359172
```
