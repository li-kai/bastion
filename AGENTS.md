# AGENTS.md

Rust monorepo in `crates/*`.

## Commands

Use `just` for all tasks.

`just build`, `just test`, `just check`, `just fix`, `just fmt`, `just check-all`

## Design

- Parse, don't validate
- Make illegal states unrepresentable
- Functional core, imperative shell
- Dependency injection over global state
- Separate configuration from logic
- Build for replaceability

Use lexical grouping/semantic prefixing. Prefer explicit, meaningful names and
structures that align with the problem domain.

## Testing

See [common testing practices](.claude/skills/write-tests/SKILL.md).

## Rust

- Use `thiserror` for error types.
- Small modules, minimal public APIs
- Newtype pattern — wrap primitives (`Latitude(f64)` not `f64`)

More on [writing Rust tests](.claude/skills/rust-tests/SKILL.md).
