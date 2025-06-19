## Rust tutorial follow

Following this youtube [tutorial](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8)

#### Ownership Rules

1. Each value in Rust has a variable that's called it's owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

#### References Rules

1. At any given time, you can have either one mutable reference or any number of immutable references
2. References must always be valid
