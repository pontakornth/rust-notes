# Controlling tests

## Threads
Tests are run in parallel by default. If you need it, just specify it.

```sh
cargo test -- --test-threads=1
```

## Output
`println` won't output if we don't specify it.

```sh
cargo test -- --show-output
```

## Run by function name
You can run test by function name.

```sh
cargo test test-name
```

`test-name` also matches `test-name2` and `test-name-whatever` as well.


## Ignore some tests

Use `#[ignore]` to ignore a test. Use `--ignored` to run only ignored tests and `--include-ignored` to run all tests.


## Organization
Please see the book.
