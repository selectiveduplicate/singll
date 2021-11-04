# singll
A [singly linked list](https://en.wikipedia.org/wiki/Linked_list#Singly_linked_list) written while going through the book [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/).

## Features
 - `push`
 - `pop`
 - Mutable and immutable iteration
 - Mutable and immutable retrieval of head and tail.
 - Implementation of `FromIterator` for 32-bit integer types. So you can collect elements into another collection like `Vec` or another `SinglyLinkedList` using [`collect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect), or build a `SinglyLinkedList` from an iterator (see the examples in the examples folder). This only works for 32-bit integer types.
 
For more details, clone the repo and run `cargo doc --open` from the root of the project to generate the documentation.

You can run the associated unit tests with `cargo test` and the examples using `cargo run --example name_of_the_example`.