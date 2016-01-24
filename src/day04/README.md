Creating partial hash collisions with MD5, using brute force. I made use of the
`rust-crypto` crate for the MD5 implementation, it's pure rust (so it doesn't
complicate the build requirements) and has an easy to use interface.

I later went back and did some performance improvements, somewhat based on
[this discussion](https://www.reddit.com/r/rust/comments/3w9z41/is_rusts_md5_slow_heck_no/),
avoiding memory allocations or String creation (which has an additional cost to
validate as UTF-8). It works over `u8` arrays, and so looks for `0`
bit-patterns at the front of the array instead of `'0'` chars in a string.

Day 4 still takes up about half of the total running time for all 25 days, but
a quick perf analysis show more than 70% of the time is in the core MD5
routines. Further significant improvements would likely require either
switching the MD5 implementation, or a multi-threaded search.

---

### --- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (very similar to bitcoins) to use as
gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at
least **five zeroes**. The input to the MD5 hash is some secret key (your
puzzle input, given below) followed by a number in decimal. To mine
AdventCoins, you must find Santa the lowest positive number (no leading zeroes:
`1`, `2`, `3`, ...) that produces such a hash.

For example:

- If your secret key is `abcdef`, the answer is `609043`, because the MD5 hash
  of `abcdef609043` starts with five zeroes (`000001dbbfa...`), and it is the
  lowest such number to do so.
- If your secret key is `pqrstuv`, the lowest number it combines with to make
  an MD5 hash starting with five zeroes is `1048970`; that is, the MD5 hash of
  `pqrstuv1048970` looks like `000006136ef....`

### --- Part Two ---

Now find one that starts with **six zeroes**.
