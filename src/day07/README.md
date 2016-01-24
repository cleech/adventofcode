The main trick to this is to memoize the evaluation of the wires, otherwise the
structure of the circuit causes the same gates/wires to be re-evaluated over
and over. I structured the circuit as a `HashMap` of gates keyed by the output
wire, with a second `HashMap` to hold already determined values as a cache.

There are a couple of, let's say underspecified forms, that show up in the
inputs. My original solution had some ugly hacks to deal with them, as I ran
into them after getting a bunch of code written. At first I assumed that fixed
values would only come from "pattern generator" gates of the form `123 -> x`,
but there are AND gates with fixed inputs like `1 AND a -> b`. Also, there are
rather pointless "pass through" gates of the form `a -> b`.

I got a lot of good cleanup ideas to reduce repeated code for each gate type
from <https://github.com/BartMassey/advent-of-code-2015/tree/master/7/>. I
decided not to copy Bart's rewriting of the original circuit `HashMap` to
remove the second "cache" `HashMap`, although I like it a lot.

New Rustisms for today; After cleanups, I made use of **slice patterns** for
string parsing in place of regular expression matching. Those are currently
feature gated, and so require a nightly build toolchain.

The **function pointers** used in the Gate values (type `fn(u16) -> u16`) are
different from the type parameter used previously when accepting closures as a
function argument (`Fn`). This is because I didn't need to close over any
environment, but also things get more complicated if the Gate type needs to
**own** a closure (which it would). You can't directly embed a closure (unless
it's always the same closure) because they each are a unique type (the size can
differ with the captured environment). If needed, the closures would have to be
referenced indirectly through a **boxed** trait object type like `Box<Fn(u16)
-> u16>`. Oh, and all those `std::ops` trait methods (`Not::not`,
`BitAnd::bitand`, `BitOr::bitor`, `Shl::shl`, `Shr::shr`) do resolve to a
function pointer by picking the correct implementation for the inferred type.


### --- Day 7: Some Assembly Required ---

This year, Santa brought little Bobby Tables a set of wires and bitwise logic
gates! Unfortunately, little Bobby is a little under the recommended age range,
and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a 16-bit
signal (a number from `0` to `65535`). A signal is provided to each wire by a
gate, another wire, or some specific value. Each wire can only get a signal
from one source, but can provide its signal to multiple destinations. A gate
provides no signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together:
`x AND y -> z` means to connect wires `x` and `y` to an AND gate, and then
connect its output to wire `z`.

For example:

- `123 -> x` means that the signal `123` is provided to wire `x`.
- `x AND y -> z` means that the bitwise AND of wire `x` and wire `y` is
  provided to wire `z`.
- `p LSHIFT 2 -> q` means that the value from wire `p` is left-shifted by `2`
  and then provided to wire `q`.
- `NOT e -> f` means that the bitwise complement of the value from wire `e` is
  provided to wire `f`.

Other possible gates include `OR` (bitwise OR) and `RSHIFT` (right-shift). If,
for some reason, you'd like to **emulate** the circuit instead, almost all
programming languages (for example, C, JavaScript, or Python) provide operators
for these gates.

For example, here is a simple circuit:
```
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
```

After it is run, these are the signals on the wires:
```
d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456
```

In little Bobby's kit's instructions booklet (provided as your puzzle input),
what signal is ultimately provided to **wire `a`**?

### --- Part Two ---

Now, take the signal you got on wire `a`, override wire `b` to that signal, and
reset the other wires (including wire `a`). What new signal is ultimately
provided to wire `a`?
