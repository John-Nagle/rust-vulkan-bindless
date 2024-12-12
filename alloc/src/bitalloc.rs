//! # Bitalloc -- lockless allocator for bits in a large bitmap.
//!
//! John Nagle
//! Animats
//! November, 2024
//!
#![forbid(unsafe_code)]
use anyhow::{anyhow, Error};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Our bitmap is an array of these. u64 today, u128 when atomics that large are widely available.
type WordType = u64;
type AtomicWordType = AtomicU64;
const WORDSIZE: usize = WordType::BITS as usize;
const WORDALLONES: WordType = !0; // this had better be the all ones value

/// Bit allocator
pub struct BitAlloc {
    /// Search start position
    search_pos: AtomicUsize,
    /// The bitmap itself
    b: Vec<AtomicWordType>,
    /// Statistics - requests
    alloc_count: AtomicU64,
    /// Statistics - word earches
    search_count: AtomicU64,
}

impl BitAlloc {
    pub fn new(size: usize) -> Self {
        assert!(WORDALLONES == WordType::MAX); // check on constant
        let word_count = (size + WORDSIZE - 1) / WORDSIZE; // number of words
        let mut b: Vec<AtomicWordType> = Vec::new();
        b.resize_with(word_count, || AtomicWordType::new(0));
        Self {
            search_pos: AtomicUsize::new(0),
            b,
            alloc_count: Default::default(),
            search_count: Default::default(),
        }
    }

    /// Length, in bits
    pub fn len(&self) -> usize {
        self.b.len() * WORDSIZE
    }

    /// Get one bit. Not atomic
    pub fn get_bit(&self, ix: usize) -> bool {
        let (word, bit) = Self::word_bit(ix);
        if word < self.b.len() {
            (self.b[word].load(Ordering::SeqCst) >> bit) & 0x1 == 1 // bit as bool
        } else {
            false
        }
    }

    /// Clear one bit. It is a error to clear an un-set bit.
    pub fn clear_bit(&self, ix: usize) -> Result<(), Error> {
        let (word, bit) = Self::word_bit(ix);
        if word < self.b.len() {
            //  Retry loop for atomic CAS
            loop {
                let val = self.b[word].load(Ordering::SeqCst); // get word
                let newval = val & !(1 << bit); // clear bit
                let swap_result =
                    self.b[word].compare_exchange(val, newval, Ordering::SeqCst, Ordering::Relaxed);
                if swap_result.is_ok() {
                    break;
                }
                log::warn!("Race condition in clear_bit, retrying");
            }
            //  Updated successfuly. Update start position for next search if this is the new min
            let _ = self.search_pos.fetch_min(word, Ordering::Relaxed);
            Ok(())
        } else {
            Err(anyhow!("Bitalloc clear_bit index out of range."))
        }
    }

    /// Allocate a bit, if any are available.
    pub fn alloc_bit(&self) -> Option<usize> {
        let _ = self.alloc_count.fetch_add(1, Ordering::Relaxed); // tally words searched
        let start_pos = self.search_pos.load(Ordering::SeqCst);
        for word in start_pos..self.b.len() {
            //  Retry loop for atomic CAS
            let _ = self.search_count.fetch_add(1, Ordering::Relaxed); // tally words searched
            loop {
                let val = self.b[word].load(Ordering::SeqCst); // get word
                if val == WORDALLONES {
                    // if all ones, done with inner loop, continue outer loo[
                    break;
                }
                //  There may be an open slot in this word.
                //  But we have to test that with an atomic operation.
                let bit = (!val).trailing_zeros(); // find first zero bit.
                let newval = val | (1 << bit); // new value for bitmap word

                //  Now try to insert that into the map with a compare and swap.
                //  If that fails, we have to try again.
                let swap_result =
                    self.b[word].compare_exchange(val, newval, Ordering::SeqCst, Ordering::Relaxed);
                if let Ok(_) = swap_result {
                    //  Update search start position to try from here next time.
                    let pos_result = self.search_pos.compare_exchange(
                        start_pos,
                        word,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    );
                    if let Err(_) = pos_result {
                        //  This is just an unsucessful optimization. Next search may be slightly slower.
                        log::info!("Race condition in alloc_bit pos update, harmless");
                    }
                    //  Return position of bit just set.
                    return Some(word as usize * usize::BITS as usize + bit as usize);
                }
                //  Compare and swap failed. Some other thread updated this value.
                log::warn!("Race condition in alloc_bit, retrying"); // should be very rare

                //  Have to try again
            }
        }
        None // bitmap is full
    }

    /// Which word and bit for an index
    fn word_bit(index: usize) -> (usize, usize) {
        (index / WORDSIZE, index % WORDSIZE)
    }
}

impl Drop for BitAlloc {
    fn drop(&mut self) {
        //  Performance statistics
        log::info!(
            "BitAlloc stats: {} allocations, {} words searched.",
            self.alloc_count.load(Ordering::Relaxed),
            self.search_count.load(Ordering::Relaxed)
        );
    }
}

#[test]
/// Basic test. Does this work at all?
fn test_bitalloc_basics() {
    use simplelog::{Config, LevelFilter, SimpleLogger};
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default()); // log to standard output
    /// Build up a list of bits
    fn bit_list(item: &BitAlloc) -> Vec<usize> {
        (0..item.len())
            .filter_map(|n| if item.get_bit(n) { Some(n) } else { None })
            .collect()
    }
    //  Try some basic operations
    let bit_alloc = BitAlloc::new(100000);
    let v0 = bit_alloc.alloc_bit().unwrap();
    assert_eq!(v0, 0);
    let v1 = bit_alloc.alloc_bit().unwrap();
    assert_eq!(v1, 1);
    let v2 = bit_alloc.alloc_bit().unwrap();
    assert_eq!(v2, 2);
    assert_eq!(bit_list(&bit_alloc), [0, 1, 2]);
    bit_alloc.clear_bit(v1).unwrap();
    assert_eq!(bit_list(&bit_alloc), [0, 2]);
    // Allocate 500 multiple words and see if that works.
    for _ in 0..500 {
        let _ = bit_alloc.alloc_bit().unwrap();
    }
    //  Construct expected result.
    let mut vlist: Vec<usize> = [0, 1, 2].to_vec();
    let vlist1: Vec<usize> = (3..502).map(|n: usize| n).collect();
    vlist.extend(vlist1);
    assert_eq!(bit_list(&bit_alloc), vlist);
    //  Do many allocs for performance check.
    for _ in 0..99000 {
        let _ = bit_alloc.alloc_bit().unwrap();
    }
    let efficiency_ratio = bit_alloc.search_count.load(Ordering::Relaxed) as f64
        / bit_alloc.alloc_count.load(Ordering::Relaxed) as f64;
    assert!(efficiency_ratio < 1.1); // for this case, it should be small.
}
