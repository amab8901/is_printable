
Determines whether a given type is printable.

Printable example:
``` rust
    let hello = "hello";
    let is_printable = hello.is_printable();
    assert_eq!(is_printable, true);
```

Unprintable example:
```rust
    let bell = '␇'
    let is_printable = bell.is_printable();
    assert_eq!(is_printable, false);
```