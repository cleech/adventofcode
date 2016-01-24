I initially implemented this using regular expressions, but later changed my
mind. All of these cases can be easily checked using regular expressions with
back-references, but the Rust-native `regex` crate doesn't do backrefs. Instead
I switched to `pcre`, but the lack of pre-compiled expressions meant that for a
simple program like this a lot of time was spent compiling them at runtime
(also, I stupidly kept re-compiling them with each use rather than complicate
the structure of the code to use some sort of context structure)

In the end I think the solution without regexs worked out fine. Since I defined
these functions over Rust string, which are UTF-8, where needed they are
converted into `Vec<char>` in order to use functions like `window`.

Regular Expressions for reference:

- Part 1:
    - vowel count can use the regex `[aeiou]` and count the matches
    - repeated letters can be matched with `(.)\1`
    - sub-strings to avoid can be matched with `(ab|cd|pq|xy)`
- Part 2:
    - repeated pairs can be matched with `(..).*\1`
    - "sandwich" triples can be matched with `(.)[^\1]\1`

---

### --- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or
nice.

A **nice string** is one with all of the following properties:

- It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or
  `aeiouaeiouaeiou`.
- It contains at least one letter that appears twice in a row, like `xx`,
  `abcdde` (`dd`), or `aabbccdd` (`aa`, `bb`, `cc`, or `dd`).
- It does **not** contain the strings `ab`, `cd`, `pq`, or `xy`, even if they
  are part of one of the other requirements.

For example:

- `ugknbfddgicrmopn` is nice because it has at least three vowels
  (`u...i...o...`), a double letter (`...dd...`), and none of the disallowed
  substrings.
- `aaa` is nice because it has at least three vowels and a double letter, even
  though the letters used by different rules overlap.
- `jchzalrnumimnmhp` is naughty because it has no double letter.
- `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
- `dvszwmarrgswjxmb` is naughty because it contains only one vowel.

How many strings are nice?

### --- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of
determining whether a string is naughty or nice. None of the old rules apply,
as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

- It contains a pair of any two letters that appears at least twice in the
  string without overlapping, like `xyxy` (`xy`) or `aabcdefgaa` (`aa`), but not like
  `aaa` (`aa`, but it overlaps).
- It contains at least one letter which repeats with exactly one letter between
  them, like `xyx`, `abcdefeghi` (`efe`), or even `aaa`.

For example:

- `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`)
  and a letter that repeats with exactly one letter between them (`zxz`).
- `xxyxx` is nice because it has a pair that appears twice and a letter that
  repeats with one between, even though the letters used by each rule overlap.
- `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with
  a single letter between them.
- `ieodomkazucvgmuy` is naughty because it has a repeating letter with one
  between (`odo`), but no pair that appears twice.

How many strings are nice under these new rules?
