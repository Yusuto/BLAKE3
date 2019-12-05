#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod avx2;
mod platform;
mod portable;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod sse41;
#[cfg(test)]
mod test;

use arrayref::{array_mut_ref, array_ref};
use arrayvec::{ArrayString, ArrayVec};
use core::cmp;
use core::fmt;
use platform::{Platform, MAX_SIMD_DEGREE, MAX_SIMD_DEGREE_OR_2};

/// The number of bytes in the default output, 32.
pub const OUT_LEN: usize = 32;

/// The number of bytes in a key, 32.
pub const KEY_LEN: usize = 32;

// These are pub for tests and benchmarks. Callers don't need them.
#[doc(hidden)]
pub const BLOCK_LEN: usize = 64;
#[doc(hidden)]
pub const CHUNK_LEN: usize = 2048;

const IV: &[u32; 8] = &[
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

const IV_BYTES: &[u8; 32] = &[
    0x67, 0xe6, 0x09, 0x6a, 0x85, 0xae, 0x67, 0xbb, 0x72, 0xf3, 0x6e, 0x3c, 0x3a, 0xf5, 0x4f, 0xa5,
    0x7f, 0x52, 0x0e, 0x51, 0x8c, 0x68, 0x05, 0x9b, 0xab, 0xd9, 0x83, 0x1f, 0x19, 0xcd, 0xe0, 0x5b,
];

const MSG_SCHEDULE: [[usize; 16]; 7] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
];

const CHUNK_OFFSET_DELTAS: &[u64; 16] = &[
    CHUNK_LEN as u64 * 0,
    CHUNK_LEN as u64 * 1,
    CHUNK_LEN as u64 * 2,
    CHUNK_LEN as u64 * 3,
    CHUNK_LEN as u64 * 4,
    CHUNK_LEN as u64 * 5,
    CHUNK_LEN as u64 * 6,
    CHUNK_LEN as u64 * 7,
    CHUNK_LEN as u64 * 8,
    CHUNK_LEN as u64 * 9,
    CHUNK_LEN as u64 * 10,
    CHUNK_LEN as u64 * 11,
    CHUNK_LEN as u64 * 12,
    CHUNK_LEN as u64 * 13,
    CHUNK_LEN as u64 * 14,
    CHUNK_LEN as u64 * 15,
];

const PARENT_OFFSET_DELTAS: &[u64; 16] = &[0; 16];

// These are the internal flags that we use to domain separate root/non-root,
// chunk/parent, and chunk beginning/middle/end. These get set at the high end
// of the block flags word in the compression function, so their values start
// high and go down.
bitflags::bitflags! {
    struct Flags: u8 {
        const CHUNK_START = 1 << 0;
        const CHUNK_END = 1 << 1;
        const PARENT = 1 << 2;
        const ROOT = 1 << 3;
        const KEYED_HASH = 1 << 4;
        const DERIVE_KEY = 1 << 5;
    }
}

fn offset_low(offset: u64) -> u32 {
    offset as u32
}

fn offset_high(offset: u64) -> u32 {
    (offset >> 32) as u32
}

/// A BLAKE3 output of the default size, 32 bytes, which implements
/// constant-time equality.
#[derive(Clone, Copy, Hash)]
pub struct Hash([u8; OUT_LEN]);

impl Hash {
    /// The bytes of the `Hash`. Note that byte arrays don't provide
    /// constant-time equality, so if  you need to compare hashes, prefer the
    /// `Hash` type.
    pub fn as_bytes(&self) -> &[u8; OUT_LEN] {
        &self.0
    }

    /// The hexadecimal encoding of the `Hash`. The returned [`ArrayString`] is
    /// a fixed size and does not allocate memory on the heap. Note that
    /// [`ArrayString`] doesn't provide constant-time equality, so if you need
    /// to compare hashes, prefer the `Hash` type.
    ///
    /// [`ArrayString`]: https://docs.rs/arrayvec/0.5.1/arrayvec/struct.ArrayString.html
    pub fn to_hex(&self) -> ArrayString<[u8; 2 * OUT_LEN]> {
        let mut s = ArrayString::new();
        let table = b"0123456789abcdef";
        for &b in self.0.iter() {
            s.push(table[(b >> 4) as usize] as char);
            s.push(table[(b & 0xf) as usize] as char);
        }
        s
    }
}

impl From<[u8; OUT_LEN]> for Hash {
    fn from(bytes: [u8; OUT_LEN]) -> Self {
        Self(bytes)
    }
}

impl From<Hash> for [u8; OUT_LEN] {
    fn from(hash: Hash) -> Self {
        hash.0
    }
}

/// This implementation is constant-time.
impl PartialEq for Hash {
    fn eq(&self, other: &Hash) -> bool {
        constant_time_eq::constant_time_eq(&self.0[..], &other.0[..])
    }
}

/// This implementation is constant-time.
impl PartialEq<[u8; OUT_LEN]> for Hash {
    fn eq(&self, other: &[u8; OUT_LEN]) -> bool {
        constant_time_eq::constant_time_eq(&self.0[..], other)
    }
}

impl Eq for Hash {}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hash({})", self.to_hex())
    }
}

// Each chunk or parent node can produce either a 32-byte chaining value or, by
// setting the ROOT flag, any number of final output bytes. The Output struct
// captures the state just prior to choosing between those two possibilities.
struct Output {
    input_chaining_value: [u8; 32],
    block: [u8; 64],
    block_len: u8,
    offset: u64,
    flags: Flags,
    platform: Platform,
}

impl Output {
    fn chaining_value(&self) -> [u8; 32] {
        let out = self.platform.compress(
            &self.input_chaining_value,
            &self.block,
            self.block_len,
            self.offset,
            self.flags,
        );
        *array_ref!(out, 0, 32)
    }

    fn root_hash(&self) -> Hash {
        debug_assert_eq!(self.offset, 0);
        let out = self.platform.compress(
            &self.input_chaining_value,
            &self.block,
            self.block_len,
            0,
            self.flags | Flags::ROOT,
        );
        Hash(*array_ref!(out, 0, 32))
    }

    fn root_output_bytes(&self, out_slice: &mut [u8]) {
        debug_assert_eq!(self.offset, 0);
        let mut offset = 0;
        for out_block in out_slice.chunks_mut(2 * OUT_LEN) {
            let out_bytes = self.platform.compress(
                &self.input_chaining_value,
                &self.block,
                self.block_len,
                offset,
                self.flags | Flags::ROOT,
            );
            out_block.copy_from_slice(&out_bytes[..out_block.len()]);
            offset += 2 * OUT_LEN as u64;
        }
    }
}

#[derive(Clone)]
struct ChunkState {
    cv: [u8; 32],
    offset: u64,
    buf: [u8; BLOCK_LEN],
    buf_len: u8,
    blocks_compressed: u8,
    flags: Flags,
    platform: Platform,
}

impl ChunkState {
    fn new(key: &[u8; 32], offset: u64, flags: Flags, platform: Platform) -> Self {
        Self {
            cv: *key,
            offset,
            buf: [0; BLOCK_LEN],
            buf_len: 0,
            blocks_compressed: 0,
            flags,
            platform,
        }
    }

    fn reset(&mut self, key: &[u8; KEY_LEN], new_offset: u64) {
        debug_assert_eq!(new_offset % CHUNK_LEN as u64, 0);
        self.cv = *key;
        self.offset = new_offset;
        self.buf = [0; BLOCK_LEN];
        self.buf_len = 0;
        self.blocks_compressed = 0;
    }

    fn len(&self) -> usize {
        BLOCK_LEN * self.blocks_compressed as usize + self.buf_len as usize
    }

    fn fill_buf(&mut self, input: &mut &[u8]) {
        let want = BLOCK_LEN - self.buf_len as usize;
        let take = cmp::min(want, input.len());
        self.buf[self.buf_len as usize..][..take].copy_from_slice(&input[..take]);
        self.buf_len += take as u8;
        *input = &input[take..];
    }

    fn start_flag(&self) -> Flags {
        if self.blocks_compressed == 0 {
            Flags::CHUNK_START
        } else {
            Flags::empty()
        }
    }

    // Try to avoid buffering as much as possible, by compressing directly from
    // the input slice when full blocks are available.
    fn update(&mut self, mut input: &[u8]) -> &mut Self {
        if self.buf_len > 0 {
            self.fill_buf(&mut input);
            if !input.is_empty() {
                debug_assert_eq!(self.buf_len as usize, BLOCK_LEN);
                let block_flags = self.flags | self.start_flag(); // borrowck
                let output = self.platform.compress(
                    &self.cv,
                    &self.buf,
                    BLOCK_LEN as u8,
                    self.offset,
                    block_flags,
                );
                self.cv = *array_ref!(output, 0, 32);
                self.buf_len = 0;
                self.buf = [0; BLOCK_LEN];
                self.blocks_compressed += 1;
            }
        }

        while input.len() > BLOCK_LEN {
            debug_assert_eq!(self.buf_len, 0);
            let block_flags = self.flags | self.start_flag(); // borrowck
            let output = self.platform.compress(
                &self.cv,
                array_ref!(input, 0, BLOCK_LEN),
                BLOCK_LEN as u8,
                self.offset,
                block_flags,
            );
            self.cv = *array_ref!(output, 0, 32);
            self.blocks_compressed += 1;
            input = &input[BLOCK_LEN..];
        }

        self.fill_buf(&mut input);
        debug_assert!(input.is_empty());
        debug_assert!(self.len() <= CHUNK_LEN);
        self
    }

    fn output(&self) -> Output {
        let block_flags = self.flags | self.start_flag() | Flags::CHUNK_END;
        Output {
            input_chaining_value: self.cv,
            block: self.buf,
            block_len: self.buf_len,
            offset: self.offset,
            flags: block_flags,
            platform: self.platform,
        }
    }
}

// Don't derive(Debug), because the state may be secret.
impl fmt::Debug for ChunkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ChunkState {{ len: {}, offset: {}, flags: {:?}, platform: {:?} }}",
            self.len(),
            self.offset,
            self.flags,
            self.platform
        )
    }
}

// IMPLEMENTATION NOTE
// ===================
// The recursive function compress_subtree_wide(), implemented below, is the
// basis of high-performance BLAKE3. We use it both for all-at-once hashing,
// and for the incremental input with Hasher (though we have to be careful with
// subtree boundaries in the incremental case). compress_subtree_wide() applies
// several optimizations at the same time:
// - Multi-threading with Rayon.
// - Parallel chunk hashing with SIMD.
// - Parallel parent hashing with SIMD. Note that while SIMD chunk hashing
//   maxes out at MAX_SIMD_DEGREE*CHUNK_LEN, parallel parent hashing continues
//   to benefit from larger inputs, because more levels of the tree benefit can
//   use full-width SIMD vectors for parent hashing. Without parallel parent
//   hashing, we lose about 10% of overall throughput on AVX2 and AVX-512.

// The largest power of two less than or equal to `n`, used for left_len()
// immediately below, and also directly in Hasher::update().
fn largest_power_of_two_leq(n: usize) -> usize {
    ((n / 2) + 1).next_power_of_two()
}

// Given some input larger than one chunk, return the number of bytes that
// should go in the left subtree. This is the largest power-of-2 number of
// chunks that leaves at least 1 byte for the right subtree.
fn left_len(content_len: usize) -> usize {
    debug_assert!(content_len > CHUNK_LEN);
    // Subtract 1 to reserve at least one byte for the right side.
    let full_chunks = (content_len - 1) / CHUNK_LEN;
    largest_power_of_two_leq(full_chunks) * CHUNK_LEN
}

// Recurse in parallel with rayon::join() if the "rayon" feature is active.
// Rayon uses a global thread pool and a work-stealing algorithm to hand the
// right side off to another thread, if idle threads are available. If the
// "rayon" feature is disabled, just make ordinary function calls for the left
// and the right.
fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
where
    A: FnOnce() -> RA + Send,
    B: FnOnce() -> RB + Send,
    RA: Send,
    RB: Send,
{
    #[cfg(feature = "rayon")]
    return rayon::join(oper_a, oper_b);
    #[cfg(not(feature = "rayon"))]
    return (oper_a(), oper_b());
}

// Use SIMD parallelism to hash up to MAX_SIMD_DEGREE chunks at the same time
// on a single thread. Write out the chunk chaining values and return the
// number of chunks hashed. These chunks are never the root and never empty;
// those cases use a different codepath.
fn compress_chunks_parallel(
    input: &[u8],
    key: &[u8; KEY_LEN],
    offset: u64,
    flags: Flags,
    platform: Platform,
    out: &mut [u8],
) -> usize {
    debug_assert!(!input.is_empty(), "empty chunks below the root");
    debug_assert!(input.len() <= MAX_SIMD_DEGREE * CHUNK_LEN);
    debug_assert_eq!(offset % CHUNK_LEN as u64, 0, "invalid offset");

    let mut chunks_exact = input.chunks_exact(CHUNK_LEN);
    let mut chunks_array = ArrayVec::<[&[u8; CHUNK_LEN]; MAX_SIMD_DEGREE]>::new();
    for chunk in &mut chunks_exact {
        chunks_array.push(array_ref!(chunk, 0, CHUNK_LEN));
    }
    platform.hash_many(
        &chunks_array,
        key,
        offset,
        CHUNK_OFFSET_DELTAS,
        flags,
        Flags::CHUNK_START,
        Flags::CHUNK_END,
        out,
    );

    // Hash the remaining partial chunk, if there is one. Note that the empty
    // chunk (meaning the empty message) is a different codepath.
    let chunks_so_far = chunks_array.len();
    if !chunks_exact.remainder().is_empty() {
        let chunk_offset = offset + (chunks_so_far * CHUNK_LEN) as u64;
        let mut chunk_state = ChunkState::new(key, chunk_offset, flags, platform);
        chunk_state.update(chunks_exact.remainder());
        *array_mut_ref!(out, chunks_so_far * OUT_LEN, OUT_LEN) =
            chunk_state.output().chaining_value();
        chunks_so_far + 1
    } else {
        chunks_so_far
    }
}

// Use SIMD parallelism to hash up to MAX_SIMD_DEGREE parents at the same time
// on a single thread. Write out the parent chaining values and return the
// number of parents hashed. (If there's an odd input chaining value left over,
// return it as an additional output.) These parents are never the root and
// never empty; those cases use a different codepath.
fn compress_parents_parallel(
    child_chaining_values: &[u8],
    key: &[u8; KEY_LEN],
    flags: Flags,
    platform: Platform,
    out: &mut [u8],
) -> usize {
    debug_assert_eq!(child_chaining_values.len() % OUT_LEN, 0, "wacky hash bytes");
    let num_children = child_chaining_values.len() / OUT_LEN;
    debug_assert!(num_children >= 2, "not enough children");
    debug_assert!(num_children <= 2 * MAX_SIMD_DEGREE, "too many");

    let mut parents_exact = child_chaining_values.chunks_exact(BLOCK_LEN);
    // Use MAX_SIMD_DEGREE_OR_2 rather than MAX_SIMD_DEGREE here, because of
    // the requirements of compress_subtree_wide().
    let mut parents_array = ArrayVec::<[&[u8; BLOCK_LEN]; MAX_SIMD_DEGREE_OR_2]>::new();
    for parent in &mut parents_exact {
        parents_array.push(array_ref!(parent, 0, BLOCK_LEN));
    }
    platform.hash_many(
        &parents_array,
        key,
        0, // Parents always use offset 0.
        PARENT_OFFSET_DELTAS,
        flags | Flags::PARENT,
        Flags::empty(), // Parents have no start flags.
        Flags::empty(), // Parents have no end flags.
        out,
    );

    // If there's an odd child left over, it becomes an output.
    let parents_so_far = parents_array.len();
    if !parents_exact.remainder().is_empty() {
        out[parents_so_far * OUT_LEN..][..OUT_LEN].copy_from_slice(parents_exact.remainder());
        parents_so_far + 1
    } else {
        parents_so_far
    }
}

// The wide helper function returns (writes out) an array of chaining values
// and returns the length of that array. The number of chaining values returned
// is the dyanmically detected SIMD degree, at most MAX_SIMD_DEGREE. Or fewer,
// if the input is shorter than that many chunks. The reason for maintaining a
// wide array of chaining values going back up the tree, is to allow the
// implementation to hash as many parents in parallel as possible.
//
// As a special case when the SIMD degree is 1, this function will still return
// at least 2 outputs. This guarantees that this function doesn't perform the
// root compression. (If it did, it would use the wrong flags, and also we
// wouldn't be able to implement exendable ouput.) Note that this function is
// not used when the whole input is only 1 chunk long; that's a different
// codepath.
fn compress_subtree_wide(
    input: &[u8],
    key: &[u8; KEY_LEN],
    offset: u64,
    flags: Flags,
    platform: Platform,
    out: &mut [u8],
) -> usize {
    // Note that the single chunk case does *not* bump the SIMD degree up to 2
    // when it is 1. This allows Rayon the option of multi-threading even the
    // 2-chunk case, which can help performance on smaller platforms.
    if input.len() <= platform.simd_degree() * CHUNK_LEN {
        return compress_chunks_parallel(input, key, offset, flags, platform, out);
    }

    // With more than simd_degree chunks, we need to recurse. Start by dividing
    // the input into left and right subtrees. (Note that this is only optimal
    // as long as the SIMD degree is a power of 2. If we ever get a SIMD degree
    // of 3 or something, we'll need a more complicated strategy.)
    debug_assert_eq!(platform.simd_degree().count_ones(), 1, "power of 2");
    let (left, right) = input.split_at(left_len(input.len()));
    let right_offset = offset + left.len() as u64;

    // Make space for the child outputs. Here we use MAX_SIMD_DEGREE_OR_2 to
    // account for the special case of returning 2 outputs when the SIMD degree
    // is 1.
    let mut cv_array = [0; 2 * MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    let degree = if left.len() == CHUNK_LEN {
        // The "simd_degree=1 and we're at the leaf nodes" case.
        debug_assert_eq!(platform.simd_degree(), 1);
        1
    } else {
        cmp::max(platform.simd_degree(), 2)
    };
    let (left_out, right_out) = cv_array.split_at_mut(degree * OUT_LEN);

    // Recurse! This uses multiple threads if the "rayon" feature is enabled.
    let (left_n, right_n) = join(
        || compress_subtree_wide(left, key, offset, flags, platform, left_out),
        || compress_subtree_wide(right, key, right_offset, flags, platform, right_out),
    );

    // The special case again. If simd_degree=1, then we'll have left_n=1 and
    // right_n=1. Rather than compressing them into a single output, return
    // them directly, to make sure we always have at least two outputs.
    debug_assert_eq!(left_n, degree);
    debug_assert!(right_n >= 1 && right_n <= left_n);
    if left_n == 1 {
        out[..2 * OUT_LEN].copy_from_slice(&cv_array[..2 * OUT_LEN]);
        return 2;
    }

    // Otherwise, do one layer of parent node compression.
    let num_children = left_n + right_n;
    compress_parents_parallel(
        &cv_array[..num_children * OUT_LEN],
        key,
        flags,
        platform,
        out,
    )
}

// Hash a subtree with compress_subtree_wide(), and then condense the resulting
// list of chaining values down to a single parent node. Don't compress that
// last parent node, however. Instead, return its message bytes (the
// concatenated chaining values of its children). This is necessary when the
// first call to update() supplies a complete subtree, because the topmost
// parent node of that subtree could end up being the root.
//
// As with compress_subtree_wide(), this function is not used on inputs of 1
// chunk or less. That's a different codepath.
fn compress_subtree_to_parent_node(
    input: &[u8],
    key: &[u8; KEY_LEN],
    offset: u64,
    flags: Flags,
    platform: Platform,
) -> [u8; BLOCK_LEN] {
    debug_assert!(input.len() > CHUNK_LEN);
    let mut cv_array = [0; 2 * MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    let mut num_cvs = compress_subtree_wide(input, &key, offset, flags, platform, &mut cv_array);
    debug_assert!(num_cvs >= 2);

    // If MAX_SIMD_DEGREE is greater than 2 and there's enough input,
    // compress_subtree_wide() returns more than 2 chaining values. Condense
    // them into 2 by forming parent nodes repeatedly.
    let mut out_array = [0; MAX_SIMD_DEGREE_OR_2 * OUT_LEN / 2];
    while num_cvs > 2 {
        let cv_slice = &cv_array[..num_cvs * OUT_LEN];
        num_cvs = compress_parents_parallel(cv_slice, key, flags, platform, &mut out_array);
        cv_array[..num_cvs * OUT_LEN].copy_from_slice(&out_array[..num_cvs * OUT_LEN]);
    }
    *array_ref!(cv_array, 0, 2 * OUT_LEN)
}

// Hash a complete input all at once. Unlike compress_subtree_wide() and
// compress_subtree_to_parent_node(), this function handles the 1 chunk case.
fn hash_all_at_once(input: &[u8], key: &[u8; KEY_LEN], flags: Flags) -> Output {
    let platform = Platform::detect();

    // If the whole subtree is one chunk, hash it directly with a ChunkState.
    if input.len() <= CHUNK_LEN {
        return ChunkState::new(key, 0, flags, platform)
            .update(input)
            .output();
    }

    // Otherwise construct an Output object from the parent node returned by
    // compress_subtree_to_parent_node().
    Output {
        input_chaining_value: *key,
        block: compress_subtree_to_parent_node(input, key, 0, flags, platform),
        block_len: BLOCK_LEN as u8,
        offset: 0,
        flags: flags | Flags::PARENT,
        platform,
    }
}

/// The default hash function.
pub fn hash(input: &[u8]) -> Hash {
    hash_all_at_once(input, IV_BYTES, Flags::empty()).root_hash()
}

/// The keyed hash function.
pub fn hash_keyed(key: &[u8; KEY_LEN], input: &[u8]) -> Hash {
    hash_all_at_once(input, key, Flags::KEYED_HASH).root_hash()
}

/// The key derivation function.
pub fn derive_key(key: &[u8; KEY_LEN], context: &[u8]) -> Hash {
    hash_all_at_once(context, key, Flags::DERIVE_KEY).root_hash()
}

fn parent_node_output(
    left_child: &[u8; 32],
    right_child: &[u8; 32],
    key: &[u8; KEY_LEN],
    flags: Flags,
    platform: Platform,
) -> Output {
    let mut block = [0; BLOCK_LEN];
    block[..32].copy_from_slice(left_child);
    block[32..].copy_from_slice(right_child);
    Output {
        input_chaining_value: *key,
        block,
        block_len: BLOCK_LEN as u8,
        offset: 0,
        flags: flags | Flags::PARENT,
        platform,
    }
}

/// An incremental hash state that can accept any number of writes, with
/// support for extendable output.
#[derive(Clone)]
pub struct Hasher {
    key: [u8; KEY_LEN],
    chunk_state: ChunkState,
    // 2^53 * 2048 = 2^64
    cv_stack: ArrayVec<[[u8; OUT_LEN]; 53]>,
}

impl Hasher {
    fn new_internal(key: &[u8; 32], flags: Flags) -> Self {
        Self {
            key: *key,
            chunk_state: ChunkState::new(key, 0, flags, Platform::detect()),
            cv_stack: ArrayVec::new(),
        }
    }

    /// Construct a new `Hasher` for the regular hash function.
    pub fn new() -> Self {
        Self::new_internal(IV_BYTES, Flags::empty())
    }

    /// Construct a new `Hasher` for the keyed hash function.
    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        Self::new_internal(key, Flags::KEYED_HASH)
    }

    /// Construct a new `Hasher` for the key derivation function.
    ///
    /// Note that the input in this case is intended to be an
    /// application-specific context string. Most callers should hardcode such
    /// strings and prefer the [`derive_key`] function.
    ///
    /// [`derive_key`]: fn.derive_key.html
    pub fn new_derive_key(key: &[u8; KEY_LEN]) -> Self {
        Self::new_internal(key, Flags::DERIVE_KEY)
    }

    /// The total number of input bytes so far.
    pub fn count(&self) -> u64 {
        self.chunk_state.offset + self.chunk_state.len() as u64
    }

    // See comment in push_cv.
    fn merge_cv_stack(&mut self, total_len: u64) {
        let post_merge_stack_len = total_len.count_ones() as usize;
        while self.cv_stack.len() > post_merge_stack_len {
            let right_child = self.cv_stack.pop().unwrap();
            let left_child = self.cv_stack.pop().unwrap();
            let parent_cv = parent_node_output(
                &left_child,
                &right_child,
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            )
            .chaining_value();
            self.cv_stack.push(parent_cv);
        }
    }

    fn push_cv(&mut self, new_cv: &[u8; 32], offset: u64) {
        // In reference_impl.rs, we merge the new CV with existing CVs from the
        // stack before pushing it. We can do that because we know more input
        // is coming, so we know none of the merges are root.
        //
        // This setting is different. We want to feed as much input as possible
        // to compress_subtree_wide(), without setting aside anything in the
        // chunk_state. If the user gives us 64 KiB, we want to parallelize
        // over all 64 KiB at once as a single subtree, rather than hashing 32
        // KiB followed by 16 KiB followed by...etc.
        //
        // But we have to worry about the possibility that no more input comes
        // in the future. That 64 KiB might be bring the total to e.g. 128 KiB.
        // We shouldn't merge that whole 128 KiB tree yet, because if no more
        // input comes in the future, then we'll have merged the root node. We
        // need that node for extendable output, not to mention setting the
        // ROOT flag properly.
        //
        // To deal with this, we merge the CV stack lazily. We do a merge of
        // what's in there *just* before adding a new CV, and we don't do any
        // merging with the new CV itself.
        //
        // We still use the "count the 1 bits" algorithm, adjusted slightly for
        // this setting, using the offset (the start of the new CV's bytes)
        // rather than the final total (the end of the new CV's bytes). That
        // algorithm is explained in detail in the spec.
        self.merge_cv_stack(offset);
        self.cv_stack.push(*new_cv);
    }

    /// Add input bytes to the hash state. You can call this any number of
    /// times.
    pub fn update(&mut self, mut input: &[u8]) -> &mut Self {
        // If we have some partial chunk bytes in the internal chunk_state, we
        // need to finish that chunk first.
        if self.chunk_state.len() > 0 {
            let want = CHUNK_LEN - self.chunk_state.len();
            let take = cmp::min(want, input.len());
            self.chunk_state.update(&input[..take]);
            input = &input[take..];
            if !input.is_empty() {
                // We've filled the current chunk, and there's more input
                // coming, so we know it's not the root and we can finalize it.
                // Then we'll proceed to hashing whole chunks below.
                debug_assert_eq!(self.chunk_state.len(), CHUNK_LEN);
                let chunk_cv = self.chunk_state.output().chaining_value();
                self.push_cv(&chunk_cv, self.chunk_state.offset);
                let new_offset = self.chunk_state.offset + CHUNK_LEN as u64;
                self.chunk_state.reset(&self.key, new_offset);
            } else {
                return self;
            }
        }

        // Now the chunk_state is clear, and we have more input. If there's
        // more than a single chunk (so, definitely not the root chunk), hash
        // the largest whole subtree we can, with the full benefits of SIMD and
        // multi-threading parallelism. Two restrictions:
        // - The subtree has to be a power-of-2 number of chunks. Only subtrees
        //   along the right edge can be incomplete, and we don't know where
        //   the right edge is going to be until we get to finalize().
        // - The subtree must evenly divide the total number of chunks up until
        //   this point (if total is not 0). If the current incomplete subtree
        //   is only waiting for 1 more chunk, we can't hash a subtree of 4
        //   chunks. We have to complete the current subtree first.
        // Because we might need to break up the input to form powers of 2, or
        // to evenly divide what we already have, this part runs in a loop.
        while input.len() > CHUNK_LEN {
            debug_assert_eq!(self.chunk_state.len(), 0, "no partial chunk data");
            debug_assert_eq!(CHUNK_LEN.count_ones(), 1, "power of 2 chunk len");
            debug_assert_eq!(self.chunk_state.offset % CHUNK_LEN as u64, 0);
            let mut subtree_len = largest_power_of_two_leq(input.len());
            // Shrink the subtree_len until it evenly divides the count so far.
            // We know it's a power of 2, so we can use a bitmask rather than
            // the more expensive modulus operation. Note that if the caller
            // consistently passes power-of-2 inputs of the same size (as is
            // hopefully typical), we'll always skip over this loop.
            while (subtree_len - 1) as u64 & self.chunk_state.offset != 0 {
                subtree_len /= 2;
            }
            // The shrunken subtree_len might now be 1 chunk long. If so, hash
            // that one chunk by itself. Otherwise, compress the subtree into a
            // pair of CVs.
            if subtree_len <= CHUNK_LEN {
                debug_assert_eq!(subtree_len, CHUNK_LEN);
                self.push_cv(
                    &ChunkState::new(
                        &self.key,
                        self.chunk_state.offset,
                        self.chunk_state.flags,
                        self.chunk_state.platform,
                    )
                    .update(&input[..subtree_len])
                    .output()
                    .chaining_value(),
                    self.chunk_state.offset,
                );
            } else {
                // This is the high-performance happy path, though getting here
                // depends on the caller giving us a long enough input.
                let cv_pair = compress_subtree_to_parent_node(
                    &input[..subtree_len],
                    &self.key,
                    self.chunk_state.offset,
                    self.chunk_state.flags,
                    self.chunk_state.platform,
                );
                let left_cv = array_ref!(cv_pair, 0, 32);
                let right_cv = array_ref!(cv_pair, 32, 32);
                // Push the two CVs we received into the CV stack in order. Because
                // the stack merges lazily, this guarantees we aren't merging the
                // root.
                self.push_cv(left_cv, self.chunk_state.offset);
                self.push_cv(right_cv, self.chunk_state.offset + (subtree_len as u64 / 2));
            }
            self.chunk_state.offset += subtree_len as u64;
            input = &input[subtree_len..];
        }

        // What remains is 1 chunk or less. Add it to the chunk state.
        debug_assert!(input.len() <= CHUNK_LEN);
        if !input.is_empty() {
            self.chunk_state.update(input);
            // Having added some input to the chunk_state, we know what's in
            // the CV stack won't become the root node, and we can do an extra
            // merge. This simplifies finalize().
            self.merge_cv_stack(self.chunk_state.offset);
        }

        self
    }

    fn final_output(&self) -> Output {
        // If the current chunk is the only chunk, that makes it the root node
        // also. Convert it directly into an Output. Otherwise, we need to
        // merge subtrees below.
        if self.cv_stack.is_empty() {
            debug_assert_eq!(self.chunk_state.offset, 0);
            return self.chunk_state.output();
        }

        // If there are any bytes in the ChunkState, finalize that chunk and
        // merge its CV with everything in the CV stack. In that case, the work
        // we did at the end of update() above guarantees that the stack
        // doesn't contain any unmerged subtrees that need to be merged first.
        // (This is important, because if there were two chunk hashes sitting
        // on top of the stack, they would need to merge with each other, and
        // merging a new chunk hash into them would be incorrect.)
        //
        // If there are no bytes in the ChunkState, we'll merge what's already
        // in the stack. In this case it's fine if there are unmerged chunks on
        // top, because we'll merge them with each other. Note that the case of
        // the empty chunk is taken care of above.
        let mut output: Output;
        let mut num_cvs_remaining = self.cv_stack.len();
        if self.chunk_state.len() > 0 {
            debug_assert_eq!(
                self.cv_stack.len(),
                self.chunk_state.offset.count_ones() as usize,
                "cv stack does not need a merge"
            );
            output = self.chunk_state.output();
        } else {
            debug_assert!(self.cv_stack.len() >= 2);
            output = parent_node_output(
                &self.cv_stack[num_cvs_remaining - 2],
                &self.cv_stack[num_cvs_remaining - 1],
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            num_cvs_remaining -= 2;
        }
        while num_cvs_remaining > 0 {
            output = parent_node_output(
                &self.cv_stack[num_cvs_remaining - 1],
                &output.chaining_value(),
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            num_cvs_remaining -= 1;
        }
        output
    }

    /// Finalize the hash state and return the [`Hash`](struct.Hash.html) of
    /// the input.
    ///
    /// This method is idempotent. Calling it twice will give the same result.
    /// You can also add more input and finalize again.
    pub fn finalize(&self) -> Hash {
        self.final_output().root_hash()
    }

    /// Finalize the hash state and write any number of extended output bytes.
    ///
    /// This method is idempotent. Calling it twice will give the same result.
    /// You can also add more input and finalize again.
    pub fn finalize_xof(&self, output: &mut [u8]) {
        self.final_output().root_output_bytes(output);
    }
}

// Don't derive(Debug), because the state may be secret.
impl fmt::Debug for Hasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hasher {{ count: {}, flags: {:?}, platform: {:?} }}",
            self.count(),
            self.chunk_state.flags,
            self.chunk_state.platform
        )
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Hasher {
    /// This is equivalent to [`update`](#method.update).
    fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
        self.update(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}