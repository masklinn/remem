- The rust version can just be `cargo run`'d.
- The C++ version can be built with `make remem`, it requires re2, a
  flake is available if useful.
- Both take a single regex through stdin (beware newlines and shell
  escaping / evaluation if using echo), compile it, then runs it
  against nonsense input to *try* and estimate capture memory
  overhead, the two values printed to stdout are the allocations
  (minus deallocations) for compilation, and the allocations (minus
  deallocations) for capture.
- `make` will build both and run them against the `regexen` sample via
  the `run` script (which requires Python), printing the values for
  each then the regex on stdout in an ascii table.

  Various variants of each gets run:

  - re2 utf8 mode
  - re2 latin1 mode, not entirely clear what it does but it impacts
    memory use so seemed fair
  - regex unicode + string
  - regex ascii + string -- this is the only mode which can fail to
    compile a regex, in which case the entries are `-`
  - regex ascii + bytes

  regex unicode + bytes is a valid mode for the binary, but it seems
  to have the same memory behaviour as unicode + string so was not
  included in the `run` script.

The Rust version uses a custom allocator to measure allocations, the
C++ version overrides `new` and `delete` and stashes the requested
size in the allocation to know how much allocations decrease it.

