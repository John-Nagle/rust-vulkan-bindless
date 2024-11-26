//! # Bitalloc -- lockless allocator for bits in a large bitmap.

//!
//! John Nagle
//! Animats
//! November, 2024
//!
use std::sync::atomic::{AtomicUsize, Ordering};
use anyhow::{Error, anyhow};

/// Our bitmap is an array of these. u64 if necessary, u128 if atomics are available.
type WordType = AtomicUsize;
const WORDSIZE: usize = usize::BITS as usize;
const WORDALLONES: usize = usize::MAX;  // this had better be the all ones value

/// Bit allocator
pub struct BitAlloc {
    /// Search start position
    search_pos: AtomicUsize,
    /// The bitmap itself
    b: Vec<WordType>
}

impl BitAlloc {
    pub fn new(size: usize) -> Self {
        let word_count = (size + WORDSIZE - 1) / WORDSIZE;   // number of words
        let mut b: Vec<AtomicUsize> = Vec::new();
        b.resize_with(word_count, || AtomicUsize::new(0));
        Self {
            search_pos: AtomicUsize::new(0),
            b
        }
    }

    /// Get one bit. Not atomic
    pub fn get_bit(&self, ix: usize) -> bool {
        let (word, bit) = Self::word_bit(ix);
        if word < self.b.len() {
            (self.b[word].load(Ordering::SeqCst) >> bit) & 0x1 == 1    // bit as bool
        } else {
            false
        }
    }
    
    /// Clear one bit. It is a error to clear an un-set bit.
    pub fn clear_bit(&self, ix: usize) -> Result<(), Error> {
        let (word, bit) = Self::word_bit(ix);
        if word < self.b.len() {
            todo!();    // Has to be done atomically
        } else {
            Err(anyhow!("Bitalloc clear_bit index out of range."))
        }       
    }
    
    /// Allocate a bit, if any are available.
    pub fn alloc_bit(&self) -> Option<usize> {
        for word in self.search_pos.load(Ordering::SeqCst)..self.b.len() {
            let val = self.b[word].load(Ordering::SeqCst);
            // ***MORE***
        }
        todo!();    // UNIMPLEMENTED
    }
    
    /// Which word and bit for an index
    fn word_bit(index: usize) -> (usize, usize) {
        (index / WORDSIZE, index % WORDSIZE)
    }
}
