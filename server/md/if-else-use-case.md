# if else common usage

```rust
let mut html = String::new();
html += "<ul>\n";
for i in 0..10{
    let class = if i % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    html += &format!("<li class=\"{}\">value_{}</li>\n",class,i);
}
html += "</ul>";
println!("{}", html);
```

# The old way (in other languages)

```rust
let mut html = String::new();
html += "<ul>\n";
for i in 0..10{
    let mut class = "";
    if i % 2 == 0 {
        class = "even";
    } else {
        class = "odd";
    }
    html += &format!("<li class=\"{}\">value_{}</li>\n",class,i);
}
html += "</ul>";
println!("{}", html);
```
