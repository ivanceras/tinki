# Traits with generics

```rust

use std::f64::consts::PI;

trait HasArea{
    fn area(&self)->f64;
}

struct Circle{
    x:f64,
    y:f64,
    radius:f64,
}

impl HasArea for Circle{
    fn area(&self)->f64{
        PI * (self.radius * self.radius)
    }
}

struct Rectangle{
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
}

impl HasArea for Rectangle{
    fn area(&self)->f64{
      let b = x2 - x1;
      let h = y2 - y1;
      (b * h).abs()
    }

}



fn print_area<T: HasArea>(shape: T){
    println!("This shape has an aread of {}", shape.area());
}

fn main(){
    let c = Circle {x:0.0, y:1.0, radius:2.0};
    let r = Rectangle {x1:0.0, y1:1.0, x2:2.0, y2:3.0};
    print_area(c);
    print_area(r);
}
```

