# integer-iterator

Implement Iterator over primitive integer types in Rust.
Also, this crate exposes the trait so one can implement it for any desired types.

# Examples

```rust
    let mut positive = 1234u32.digits();
    assert_eq!(positive.next(), Some(4));
    assert_eq!(positive.next(), Some(3));
    assert_eq!(positive.next(), Some(2));
    assert_eq!(positive.next(), Some(1));

    let mut negative = (-1234i32).digits();
    assert_eq!(negative.next(), Some(4));
    assert_eq!(negative.next(), Some(3));
    assert_eq!(negative.next(), Some(2));
    assert_eq!(negative.next(), Some(1));
```
