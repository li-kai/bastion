---
name: write-tests
description:
    Testing best practices, TDD workflow, and test structure. Use when writing
    or reviewing tests.
---

# Writing Tests

## TDD Workflow

1. **Red**: Write a failing test that defines expected behavior
2. **Green**: Write minimal code to make it pass
3. **Refactor**: Clean up while keeping tests green

## Naming

Describe behavior, not implementation. A reader should understand what's tested
without reading the body.

```
// Bad
test_parser()
handles_edge_case()

// Good
rejects_negative_amounts()
parse_returns_error_when_input_is_empty()
```

## Parameterization

Prefer parameterized tests over duplication. Name cases for clarity:

**Rust (rstest):**

```rust
#[rstest]
#[case::empty("", 0)]
#[case::unicode("日本語", 3)]
fn char_count(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(input.chars().count(), expected);
}
```

**TypeScript (Vitest):**

```typescript
test.each([
    { name: "empty", input: "", expected: 0 },
    { name: "unicode", input: "日本語", expected: 3 },
])("char_count($name)", ({ input, expected }) => {
    expect(charCount(input)).toBe(expected)
})
```

## Structure

Keep tests flat. Avoid nested describe/context blocks.

## Best Practices

**One behavior per test.** A failing test should pinpoint the bug. If a test
asserts multiple unrelated behaviors, split it.

**Minimal setup.** Only set up what the test needs. Long setup suggests the code
under test has too many dependencies.

**Test boundaries, not internals.** Test public APIs.

**Test behavior, not implementation.** Tests should survive refactors. If
changing internals (without changing behavior) breaks tests, they're too
coupled.

**Descriptive failures.** When a test fails, the output should explain what went
wrong:

```rust
// Bad: "assertion failed: result.is_ok()"
assert!(result.is_ok());

// Good: "parse failed: InvalidToken at line 3"
let ast = parse(input).expect("parse failed");
```

```typescript
// Bad: shows generic matcher failure
expect(result.ok).toBe(true)

// Good: shows the actual error
expect(result.error).toBeUndefined()
```

## Expect Tests (Snapshots)

Use expect tests for outputs that are:

- **Large or complex** — ASTs, formatted output, serialized data structures
- **Tedious to write by hand** — error messages with context, multi-line strings

Avoid expect tests when:

- **Exact values matter for correctness** — hashes, IDs, precise calculations
- **Output is non-deterministic**

**Rust (`expect_test`):**

```rust
use expect_test::expect;

#[test]
fn formats_error() {
    let err = parse("invalid {").unwrap_err();
    expect![[r#"
        ParseError: unexpected end of input
          at line 1, column 9
    "#]].assert_eq(&err.to_string());
}
```

Update snapshots: `UPDATE_EXPECT=1 just test`

**TypeScript (Vitest):**

```typescript
import { expect, test } from "vitest"

test("formats error", () => {
    const err = parse("invalid {")
    expect(err.message).toMatchInlineSnapshot(`
    "ParseError: unexpected end of input
      at line 1, column 9"
  `)
})
```

Update snapshots: `pnpm test -u`

**Review discipline:** Treat snapshot updates like code changes. Diff them
carefully—automated updates can silently accept bugs.

---

## Language-Specific Guides

- [TypeScript Tests](../typescript-tests/SKILL.md)
- [Rust Tests](../rust-tests/SKILL.md)
