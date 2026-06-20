# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`dice` is a Rust app and library for simulating dice rolling. It builds as both a
binary (an interactive terminal REPL) and a library that exposes the dice logic.

## Commands

```bash
cargo run            # launch the interactive dice-roller REPL
cargo build          # build binary + library
cargo test           # run all tests
cargo test favourableness_combine_test   # run a single test by name
cargo test dice::parser                  # run tests in one module
cargo clippy         # lint
cargo fmt            # format
```

`Cargo.lock` is gitignored, so this is treated as a library for versioning purposes.

## Architecture

The crate is split into two independent modules, wired together only in `main.rs`:

- **`dice/`** — the rolling engine, with no terminal/IO dependencies:
  - `parser.rs` parses dice expressions against the regex `^([+\-]|\d+)d(\d+)$`.
    The first group is either a sign (`+`/`-`) or a count; the second is the die
    size. A count produces N neutral dice; a `+`/`-` produces a single
    favoured/illfavoured die. Supported die sizes: 2, 4, 6, 8, 10, 12, 20, 100.
  - `rolls.rs` defines `DiceType`, `DiceRoll`, `RollResult`, and the
    `Favourableness` model. `roll_fav` is the entry point: favoured rolls twice
    and takes the max (advantage), illfavoured rolls twice and takes the min
    (disadvantage), neutral rolls once.
  - `error.rs` defines `DiceError`, the single error type returned across the crate.

- **`text_app/`** — a generic terminal REPL loop (`text_app_loop`), fully decoupled
  from dice. It owns the crossterm raw-mode input handling (line editing, history
  via Up/Down, Esc to clear, Ctrl-C / `quit`/`q`/`exit` to leave) and delegates each
  submitted line to an injected `app_logic: &dyn Fn(String) -> Result<(), DiceError>`
  closure. `main.rs::roll_dice_app` is the concrete closure that ties the REPL to the
  dice engine. Keep `text_app` free of dice-specific logic — new behaviour goes in the
  injected closure, not the loop.

### Favourableness

`Favourableness` is `Favoured` / `Illfavoured` / `Neutral(bool)`. The `bool` in
`Neutral` records whether the neutral state arose from *combining* an opposing
favoured + illfavoured pair (`true`), versus a plain neutral die (`false`); a
combined neutral is "sticky" and dominates further combination. Combination logic
lives in `Favourableness::combine` and the `Add`/`AddAssign` impls — the unit test
in `rolls.rs` is the authoritative spec for its truth table.

## Notes

- `main.rs` re-declares `mod dice;` / `mod text_app;` for the binary, while `lib.rs`
  exposes the same modules as `pub mod` for the library. Both build paths must stay
  in sync.
- `crossterm` raw mode is toggled around input reading; if you add early returns in
  `text_app_loop`, ensure `terminal::disable_raw_mode()` still runs or the terminal
  is left in a broken state.
