---
name: write-comments
description:
    Comment style and hygiene. Use when writing, reviewing, or refining code
    comments and documentation.
---

# Writing Comments

## Principle

Comments should tell the reader something they can't get from the signature and
a quick scan of the body.

**Litmus test:** if you deleted the comment and a competent reader of the
signature alone would be surprised by some behavior, the comment is
load-bearing. If they'd just nod, it's noise.

## Contract over mechanism

Document _what_ a function promises (failure conditions, edge-case behavior,
`None` semantics), not _how_ it works internally.

```rust
// Good: tells you when it fails
/// Intersects numeric bounds and computes LCM of `multipleOf` values.
/// Returns `None` when bounds invert or no multiple exists in range.

// Bad: restates the body
/// Takes the max of the mins and the min of the maxs, then computes
/// the LCM of mul_of_a and mul_of_b using integer scaling.
```

## Keep the "why", drop the "what"

If a design choice isn't obvious, the comment earns its place. If the code is
clear, the comment is noise.

```rust
// Good: non-obvious rationale
// LCM overflow: keep the larger value (sound overapproximation --
// multiples of max(a,b) are a superset of multiples of lcm(a,b)).

// Bad: restates the match arms
// If both are Required, return Required. If one is Required, return
// Required. Otherwise return NotRequired.
```

## Doc comments (`///`) vs inner comments (`//`)

| Use   | For                                                          |
| ----- | ------------------------------------------------------------ |
| `///` | Contract: what callers need to know without reading body     |
| `//`  | Implementation: algorithm steps, formulas, non-obvious logic |

A 3-line match body doesn't need a doc comment restating its truth table. A
sound overapproximation strategy does need an inner `//` explaining why it's
safe.

## Style

- Lead with what the function does, not how
- State failure conditions and edge-case behavior
- Mention `None`/`Err` semantics when not obvious from the type
- Use plain English; avoid Unicode arrows and symbols in comments
- Keep doc comments concise; one or two lines is usually enough
- Use list form for case-by-case behavior when there are 3+ variants

```rust
/// Merges per-item constraints by variant:
/// - `Any` + x = x (identity)
/// - `Uniform` + `Uniform` = `and` schemas
/// - `Tuple` + `Tuple` = pairwise `and`, padded with `additional`
/// - `Uniform` + `Tuple` = distribute `Uniform` over each position
```

## When to omit

No comment is needed when:

- The function name and signature fully describe the behavior
- The body is a short, obvious match or delegation
- A doc comment would just restate the function name in sentence form
