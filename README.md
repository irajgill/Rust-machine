# Rust-machine

**A modular blockchain state machine runtime in Rust, inspired by Polkadot SDK.**

---

## Overview

Rust-machine is a lightweight, educational blockchain runtime, architected with *modular pallets*, strong type safety, and procedural macros for call routing. It demonstrates key blockchain concepts: balance management, ownership proofs, and generic block/extrinsic structures—mirroring patterns from projects like [Substrate](https://substrate.dev/).

---

## Features

- **System Pallet:** Manages core blockchain concepts like block numbers and nonces.
- **Balances Pallet:** Handles account balances with safe, checked transfer logic.
- **Proof of Existence Pallet:** Allows claiming and revoking ownership of arbitrary content.
- **Support Module:** Contains shared primitives like blocks and extrinsics.
- **Custom Procedural Macros:** Automatically generate call-routing, dispatch enums, and runtime glue code.

---

