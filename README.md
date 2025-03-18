
# Determines whether a given text-based value is printable

## Printable example

``` rust
    use is_printable::IsPrintable;

    let hello = "hello";
    let is_printable = hello.is_printable();

    assert_eq!(is_printable, true);
```

## Unprintable example

```rust
    use is_printable::IsPrintable;

    let bell = '\u{7}'
    let is_printable = bell.is_printable();

    assert_eq!(is_printable, false);
```
