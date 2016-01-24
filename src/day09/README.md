This is a classic *traveling salesman problem*. While there are algorithms that
may improve on a brute force check of every possibility, the set of cities here
is small enough to not care.

I used the `permutohedron` crate to generate all possible routes (permutations
on the list of cities), and then summed up the distances for each. The
`get_total_route_distance` function uses `windows` to look at each leg of the
route list, then the `lookup` returns an `Option` in case there's no route
between the cities (never happens, the input is complete). Due to that choice,
the sum is done with a `fold_options` from `itertools` which is like a folding
version of using `collect` to check for errors. Finally `filter_map` is used to
call the summing function while ignoring `None` and strip away the `Option`
wrapper from `Some(usize)` before passing to `min` or `max`.

I implemented the `UnorderedPair` type with custom `PartialEq` and `Hash`
behavior to get a feel for implementing standard traits over custom types. The
much easier choice for just solving this problem would be to insert each pair
of cities into the RouteMap twice ((a,b) -> distance, and then (b,a) ->
distance).

---

### --- Day 9: All in a Single Night ---

Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided
him the distances between every pair of locations. He can start and end at any
two (different) locations he wants, but he must visit each location exactly
once. What is the **shortest distance** he can travel to achieve this?

For example, given the following distances:
```
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
```

The possible routes are therefore:
```
Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982
```

The shortest of these is `London -> Dublin -> Belfast = 605`, and so the answer
is `605` in this example.

What is the distance of the shortest route?

### --- Part Two ---

The next year, just to show off, Santa decides to take the route with the
**longest distance** instead.

He can still start and end at any two (different) locations he wants, and he
still must visit each location exactly once.

For example, given the distances above, the longest route would be `982` via 
(for example) `Dublin -> London -> Belfast`.

What is the distance of the longest route?
