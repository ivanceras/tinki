##  No fat classes, pay only what you use
```rust

use std::f64::consts::PI;

trait HasArea{
    fn area(&self)->f64;
    fn get_name(&self)->String;
}
trait HasSides{
  fn sides(&self)->f64;
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
   fn get_name(&self)->String{
      "a circle".to_string()
    }
}

impl HasSides for Circle{
   fn sides(&self)->f64{
      std::f64::INFINITY
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
      let b = self.x2 - self.x1;
      let h = self.y2 - self.y1;
      (b * h).abs()
    }
    fn get_name(&self)->String{
      "this rectangle".to_string()
    }

}

impl HasSides for Rectangle{
  fn sides(&self)->f64{
    4.0
  }
}



fn print_area<T: HasArea>(shape: &T){
    println!("This shape({}) has an area of {}",shape.get_name(), shape.area());
}

fn main(){
    let c = Circle {x:0.0, y:1.0, radius:2.0};
    let r = Rectangle {x1:0.0, y1:1.0, x2:2.0, y2:3.0};
    print_area(&c);
    print_area(&r);
    println!("circle has {} sides", c.sides());
    println!("rectangle has {} sides", r.sides());
}
```

```sh
$>cargo run

This shape(a circle) has an area of 12.566370614359172
This shape(this rectangle) has an area of 4
circle has inf sides
rectangle has 4 sides

```
