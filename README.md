# Rust-machine

**A modular blockchain state machine runtime in Rust, inspired by Polkadot SDK.**

---

## Overview

Rust-machine is a lightweight, blockchain runtime, architected with *modular pallets*, strong type safety, and procedural macros for call routing. It demonstrates key blockchain concepts: balance management, ownership proofs, and generic block/extrinsic structures—mirroring patterns from projects like [Substrate](https://substrate.dev/).

---

## Features

- **System Pallet:** Manages core blockchain concepts like block numbers and nonces.
- **Balances Pallet:** Handles account balances with safe, checked transfer logic.
- **Proof of Existence Pallet:** Allows claiming and revoking ownership of arbitrary content.
- **Support Module:** Contains shared primitives like blocks and extrinsics.
- **Custom Procedural Macros:** Automatically generate call-routing, dispatch enums, and runtime glue code.

---

### Build & Run

`git clone https://github.com/irajgill/Rust-machine.git`
`cd Rust-machine`
`cargo build`


The main runtime logic and pallet tests can be run with:

`cargo test`


---

## Key Concepts & Architecture

- **Configuration Trait Pattern:** Each pallet abstracts its dependencies via a trait, enabling composability and flexible type usage.
- **Generic Pallet Design:** Pallets are generic over their config, reusable with different type and runtime setups.
- **Unified Call Dispatch:** A custom procedural macro system generates enums and dispatch traits for all "callable" extrinsics/actions of each pallet.
- **Thorough Error Handling:** Uses Rust's `Result` types, clear error enums, and checked arithmetic.

---

## Macros

### Call Macro

The `#[macros::call]` macro:
- Parses function signatures in each pallet
- Generates a `Call` enum aggregating all dispatchable calls
- Implements the `Dispatch` trait for automatic routing

### Runtime Macro

The `#[macros::runtime]` macro:
- Generates runtime construction
- Aggregates all pallets
- Implements block execution and extrinsic dispatch

---

## Testing

Run all tests:
`cargo run`


