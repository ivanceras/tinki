# Struct

```rust

struct Point{
    x: f32,
    y: f32,
}

```

## Adding methods to a struct

```rust

impl Point{
    
    fn new(x: f32, y: f32) -> Self {
        Point{
            x: x, 
            y: y
        }
    }
    
    fn origin() -> Self {
    
        Point{
            x: 0.0,
            y: 0.0
        }
    }
}

```

```rust

fn main(){
    let point = Point::origin();
    println!("point: {}, {}", point.x, point.y);
}

```

```sh

$> cargo run
point: 0, 0

```
