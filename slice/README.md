# Slice
+ a kind of reference (hence doesn't have ownership)
+ contiguous sequence of elements in a collection.
+ string slice is written as `&str`

## Usage
+ to sync values with the data it is calculated from
+ for example: code which gives back index of 2nd word from a string
    it could be that the string changes in value
    but we still have these indexes we derived from it,
    slices aim to solve this
