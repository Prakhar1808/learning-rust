a bug can be found when trying to find the first word of the given string without slices
this means that the string might go out of scope and the index is invalid now, slices solve this, for better reference i am quoting the rust book here

>_when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know much sooner that we have a problem with our code. Using the slice version of first_word will throw a compile-time error:_

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}
```
