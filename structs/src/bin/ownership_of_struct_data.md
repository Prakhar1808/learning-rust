# Ownership of Struct Data

> we used owned `String` type rather than the `&str` string slice type. This is a deliberate choice beacuse we want each instance of this struct to own all of it's data and for that data to be valid for as long as the entire struct is valid

> It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes
