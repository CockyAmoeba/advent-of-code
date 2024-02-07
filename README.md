# advent-of-code
Rust based implementation of Advent of Code puzzles as a learning exercise for familiarising myself with Rust

I've pre-set some things to automate or reduce the friction of doing things by leveraging hte great rust ecosystem - for testing, benchmarking, and otherwise evaluating the performance of the Rust solutions.

This includes the command `just work` which is passed a particular day and part and is the equivalent workflow of running all of these in a row and stopping if one fails, then re-starting the flow after changes.

```
cargo check
cargo nextest run
clippy-tracing --action check
cargo clippy
cargo bench
```

Alternatively you can make use of the awesome tool [`bacon`][bacon] - is a background rust code checker.

It's designed for minimal interaction so that you can just let it run, alongside your editor, and be notified of warnings, errors, or test failures in your Rust code.

## Quick setup
``` shell
cargo install cargo-nextest cargo-generate cargo-watch aoc-cli bacon
```

## Just

Just is a command runner and provided a lightweight interface to predefined and dynamically configured commands for testing, templatised bootsrapping, benchmarking, downloading AoC inputs and puzzle markdown etc.

```shell
brew install just
```

## Prepare for a new day

```shell
just create <day_number>
```

## Divan

Criterion is the defacto benchmarking crate but I wanted to compare it against [Divan][divan]. Divan has a simpler API and provides neat approach to benchmarking generic functions and measuring allocations (not yet tried) - see [Divan over criterion][divan:compared-to-criterion].

## cargo-nextest

[cargo-nextest][cargo-nextest] is "a next-generation test runner for Rust projects". Basically that means it includes [an interesting execution model][cargo-nextest-execution-model] than can be great for projects with a _lot_ of tests. As it is essentially 2 ~4 x faster than standard testing [Nextest Benchmarks][nextest-bench]

Only drawback is that doesn't run doctests yet - basically that means you also run `cargo test --doc`.

```shell
cargo install cargo-nextest
```


[bacon]: https://crates.io/crates/bacon
[divan]: https://github.com/nvzqz/divan
[divan:compared-to-criterion]: https://nikolaivazquez.com/blog/divan/#compared-to-criterion
[cargo-nextest]: https://nexte.st/
[cargo-nextest-execution-model]: https://nexte.st/book/how-it-works.html
[nextest-bench]: https://nexte.st/book/benchmarks.html