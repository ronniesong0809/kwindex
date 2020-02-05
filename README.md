# kwindex
[![Build Status](https://travis-ci.com/ronniesong0809/kwindex.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=master)](https://travis-ci.com/ronniesong0809/kwindex)
![Rust](https://github.com/ronniesong0809/kwindex/workflows/Rust/badge.svg)

Copyright (c) 2020 Ronnie Song

This is a Rust based "keyword index" library crate that maintaining an index of words from texts.

`pub struct KWIndex<'a>(Vec<&'a str>);`

## Usages

##### - Use crate to access functions
extern crate kwindex;
use kwindex::*;

##### - Make a new empty target words index:
```
let mut index = KWIndex::new();
```

##### - Parse text and add each valid word to index:
```
index = index.extend_from_text("Hello world.");
```

##### - Check if the index is empty:
```
assert_eq!(true, index.is_empty());
```

##### - Count the number of words:
```
assert_eq!(2, index.len());
```

##### - Count the number of occurrences of the `keyword`:
```
assert_eq!(1, index.count_matches("world"));
```

## Run Example
To run the example program, type the command below:
```
cargo run --example example
```
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\examples\example.exe`
is_empty(): true

[Hey] is add to KWIndex index
[!] is removed from [world!]
[world] is add to KWIndex index

is_empty(): false
len(): 2
count_matches('world'): 1

KWIndex { word: ["Hey", "world"] }
```
Everything went well! It parse the text and add to the list without issue, and successfully print its is_empty check, length, and the number of matches.

## Test
To test the library crate, type the command below:
```
cargo test
```

```
   Compiling kwindex v0.1.0 (C:\Users\ronsong\Desktop\Docs\rust\kwindex)
    Finished test [unoptimized + debuginfo] target(s) in 0.96s
     Running target\debug\deps\test-8f8dfddd5ec24cfa.exe

running 6 tests
test kwindex_tests::test_is_empty ... ok
test kwindex_tests::test_count_matches ... ok
test kwindex_tests::test_count_matches2 ... ok
test kwindex_tests::test_is_not_empty ... ok
test kwindex_tests::test_len ... ok
test kwindex_tests::test_len2 ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests passed with no issues.

The tests are placed in tests/rand.rs file that uses std *assert_eq!()*, and *assert_ne!()* to test equality of the actual result and expected result of the *new()*, *extend_to_text()*, *count_matches()*, *len()* and *is_empty()* functions in that file.

Travis CI is running to do the automated testing. [![Build Status](https://travis-ci.com/ronniesong0809/kwindex.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=master)](https://travis-ci.com/ronniesong0809/kwindex)

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
