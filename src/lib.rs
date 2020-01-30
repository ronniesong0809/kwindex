#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a str> {
    word: Vec<&'a str>,
}

impl<'a str> KWIndex<'a str> {
    // Make a new empty target words list.
    pub fn new() -> Self {
        let word = Vec::new();
        Self { word }

    }

    // Parse the `target` text and add the sequence of
    // valid words contained in it to this `KWIndex`
    // index.
    // pub fn extend_from_text(mut self, target: &'a str) -> Self {

    // }

    // Count the number of occurrences of the given `keyword`
    // that are indexed by this `KWIndex`.
    // pub fn count_matches(&self, keyword: &str) -> usize {

    // }

    // Count the number of words that are indexed by this
    // `KWIndex`.
    pub fn len(&self) -> usize {
        self.len()
    }

    // Is this index empty?
    pub fn is_empty(&self) -> bool {
        self.len()==0
    }
}