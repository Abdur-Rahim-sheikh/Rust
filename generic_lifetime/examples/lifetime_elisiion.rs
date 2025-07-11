fn main() {}
// Though the function returning reference it does not need the lifetime

// because elision rule is determining this

//  1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime parameter,
//    that lifetime is assigned to all output lifetime parameters

// 3. If there are multiple input lifetime paramters, but one of them is &self or &mut self
//    the lifetime of self if assigned to all output lifetime paramters.

// by the way, parameters lifetime called input lifetime,
// returned values lifetime is called output lifetime.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
