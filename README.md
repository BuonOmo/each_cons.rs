# `each_cons.rs`

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/BuonOmo/each_cons.rs/CI)](https://github.com/BuonOmo/each_cons.rs/actions?query=workflow%3A%22CI%22)
[![Crates.io](https://img.shields.io/crates/v/each_cons)](https://crates.io/crates/each_cons)
[![License](https://img.shields.io/crates/l/each_cons)](https://github.com/BuonOmo/each_cons.rs/blob/main/LICENSE)
[![docs.rs](https://img.shields.io/badge/docs-available-informational)](https://docs.rs/each_cons)


![](https://buildstats.info/github/chart/BuonOmo/each_cons.rs?branch=main)

A port of ruby's [`Enumerable#each_cons`](https://rubydoc.info/stdlib/core/Enumerable:each_cons).


Add it to your dependencies:

```toml
[dependencies]
each_cons = 0.2.0
```

And use it:

```rust
use each_cons::ConsIterator;

let v = vec!["foo", "bar", "baz"];
for cons in v.iter().each_cons(2) {
    println!("{}", cons.iter().fold(
        "".to_string(),
        |acc, curr| format!("{} {}", acc, curr))
    );
}
// foo bar
// bar baz
```
