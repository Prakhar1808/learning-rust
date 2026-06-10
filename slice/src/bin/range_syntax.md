## Range Syntax ".."

```rust
fn main() {
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
}
```

```rust
fn range_syntax() {
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; // same as the above

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..]; // same as the above line
}
```

drop both values LOL
```rust

let s = String::from("hello");

let len = s.len();
let slice = &s[0..len];
let slice = &s[..]; //same as the line above
```
