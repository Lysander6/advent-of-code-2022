# Advent of Code 2022

## Prerequisites

- [`asdf` v0.10.2-7e7a1fa](https://asdf-vm.com/)

## Setup

```sh
asdf install
```

## Adding package for a new challenge

1. Run `cookiecutter .scaffold/day`
2. Add `"day_NN"` to `members` array in [Cargo.toml](./Cargo.toml)
3. Run `cargo check` to trigger `Cargo.lock` update

## Documentation

To compile and display documentation run:

```sh
cargo doc --open
```

## Day 1

```sh
cargo run -p day_01 -- ./day_01/input.txt
```

## Day 2

```sh
cargo run -p day_02 -- ./day_02/input.txt
```

## Day 3

```sh
cargo run -p day_03 -- ./day_03/input.txt
```

## Day 4

```sh
cargo run -p day_04 -- ./day_04/input.txt
```

## Day 5

```sh
cargo run -p day_05 -- ./day_05/input.txt
```

## Day 6

```sh
cargo run -p day_06 -- ./day_06/input.txt
```

## Day 7

```sh
cargo run -p day_07 -- ./day_07/input.txt
```
