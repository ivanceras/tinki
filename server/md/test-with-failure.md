# Test and expected failure

```rust
#[test]
#[should_fail]
fn adding_one(){
    let expected: u64 = 5;
    let actual: u64 = 4;
    assert_eq!(expected, actual);
}
```

```sh
$>cargo test
 running 1 test
       test adding_one ... OK

```
