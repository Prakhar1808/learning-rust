## other slices
array slicing:
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3])
```

+ slice is of &[i32] type
+ storing is similar to string slices
    1. reference to the first element and length
    2. will be picked up in _ch-8 vectors_
