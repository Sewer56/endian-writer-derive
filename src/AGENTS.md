# endian-writer-derive

Procedural Macros for endian-writer crate. Auto-derives EndianWritableAt, EndianReadableAt, and HasSize traits for structs.

# Project Structure

- `endian-writer-derive/` - Main proc-macro crate
  - `src/` - Proc-macro source code
  - `tests/` - Macro expansion tests

# Code Guidelines

- Optimize for performance; use zero-cost abstractions, avoid allocations.
- Keep modules under 500 lines (excluding tests); split if larger.
- Place `use` inside functions only for `#[cfg]` conditional compilation.

# Documentation Standards

- Document public items with `///`
- Add examples in docs where helpful
- Use `//!` for module-level docs
- Focus comments on "why" not "what"
- Use [`TypeName`] rustdoc links, not backticks.

# Post-Change Verification

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-features -- -D warnings
cargo doc --workspace --all-features
cargo fmt --all
cargo publish --dry-run -p endian-writer-derive
```

All must pass before submitting.
