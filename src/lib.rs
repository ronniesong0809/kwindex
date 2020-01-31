#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a> {
    word: Vec<&'a str>,
}

impl<'a> KWIndex<'a> {
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
    pub fn count_matches(&self, keyword: &str) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut counter = 0;
        for i in &self.word {
            if i == & keyword {
                counter += 1;
            }
        }
        counter
    }

    // Count the number of words that are indexed by this
    // `KWIndex`.
    pub fn len(&self) -> usize {
        self.word.len()
    }

    // Is this index empty?
    pub fn is_empty(&self) -> bool {
        self.word.len()==0
    }
}