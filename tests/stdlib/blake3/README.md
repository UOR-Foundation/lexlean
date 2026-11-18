# BLAKE3 oracle vectors

The official BLAKE3 test vectors, used by `SL-16` (SPEC.md §17.22) as an
oracle for the `LexLeanStd.Blake3` module. `test_vectors.json` is copied
verbatim from `test_vectors/test_vectors.json` of the BLAKE3 reference
repository,

    https://github.com/BLAKE3-team/BLAKE3/blob/ae3e8e6b3a5ae3190ca5d62820789b17886a0038/test_vectors/test_vectors.json

the last commit to change the file, and its SHA-256 is
`dcb91ea8accc77e6d6e632af7cdc1a99a9f3ae78cf648da595c7d064db32f624`. The
reference repository licenses it under CC0 1.0 or Apache 2.0.

Each case is an input length from 0 to 102400 bytes and the outputs of the
three modes; the input is the repeating sequence 0, 1, ..., 250. The module
implements the hash mode only, so the case reads the first 32 bytes of each
`hash` and ignores `keyed_hash` and `derive_key`. The vectors of 0 and 1 byte,
one block each, are stated in the module itself as theorems, and the
case checks every vector; vectors longer than 8193 bytes run under rustc only.

These vectors are an oracle, never an authority: a disagreement fails the
case and is filed as a defect; it is never resolved by adopting the vector's
digest.
