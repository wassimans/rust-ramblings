//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    // Split 'haystack' by 'delimiter'
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// In order to do something like: for str in strSplitInstance ...
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    // Find where the next delimiter appears in the remainder, chope off that part of the str
    // and return it, and set the remainder to what remains
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delimiter) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delimiter];
                *remainder = &remainder[(next_delimiter + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d e ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e", ""].into_iter()));
}
