---
name: rust-tests
description:
    Rust testing patterns with rstest, parameterization, fixtures, and async
    tests. Use when writing or reviewing Rust tests.
paths: "**/*.rs"
---

# Rust Tests

See [common testing practices](../write-tests/SKILL.md) for general
philosophy.

Run tests with `just test`.

## Assertions

| Use                    | When                         | Example                                  |
| ---------------------- | ---------------------------- | ---------------------------------------- |
| `expect("msg")`        | Setup that must succeed      | `server.start().expect("server starts")` |
| `assert!`/`assert_eq!` | The thing you're testing     | `assert!(err.is_not_found())`            |
| Helper functions       | Reusable setup, context      | `read_fixture(path)`                     |
| Custom macros          | Standard output isn't enough | Multi-line diffs                         |

**expect** = "if this fails, the test is broken", **assert** = "this is what I'm
testing"

```rust
// Setup - expect
let server = TestServer::spawn().expect("server starts");

// Test - assert (expect_err is still setup: we expect an error to examine)
let err = result.expect_err("should fail");
assert!(err.api_error().expect("Api error").is_not_found());
```

Prefer helper methods over match blocks. Reusable helpers with
`#[track_caller]`:

```rust
#[track_caller]
fn read_fixture(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}
```

## Organization

Colocate test structs with their tests:

```rust
#[derive(Debug, PartialEq)]
struct RequiredOnly {
    pub name: String,
}

#[rstest]
#[case(RequiredOnly { name: "foo".into() }, json!({"name": "foo"}))]
fn required_roundtrip(#[case] val: RequiredOnly, #[case] json: Value) {
    // ...
}

#[derive(Debug, PartialEq)]
struct WithOptional {
    pub name: String,
    pub tag: Option<String>,
}

#[rstest]
// ...
```

Skip comment headers if types are near their tests. Otherwise, use `//`
comments.

## rstest

**Single parameter:** use `#[values]` **Multiple parameters:** use `#[case]`

```rust
// Single parameter
#[rstest]
fn roundtrip(#[values("hello", "world", "")] input: &str) {
    assert_eq!(parse(input).to_string(), input);
}

// Multiple parameters
#[rstest]
#[case("", 0)]
#[case::unicode("日本語", 3)]  // Add names when not obvious
fn char_count(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(input.chars().count(), expected);
}

// Matrix: cartesian product
#[rstest]
fn parse(#[values("0", "42")] input: &str, #[values(10, 16)] radix: u32) {
    assert!(i32::from_str_radix(input, radix).is_ok());
}
```

Fixtures can depend on other fixtures:

```rust
#[fixture]
fn parser(config: Config) -> Parser {
    Parser::new(config)
}
```

### Fixtures

Setup/teardown via fixtures that return a guard:

```rust
#[fixture]
fn temp_dir() -> TempDir {
    TempDir::new().unwrap()  // cleaned up on drop
}
```

### Async

```rust
#[fixture]
async fn db_pool() -> Pool {
    Pool::connect("postgres://test").await.unwrap()
}

#[rstest]
#[tokio::test]
async fn queries_users(#[future] db_pool: Pool) {
    let pool = db_pool.await;
    assert!(pool.query("SELECT 1").await.is_ok());
}
```

### Once Fixtures

Shared state initialized once across all tests (must be `Sync`):

```rust
#[fixture]
#[once]
fn global_config() -> Config {
    Config::load_expensive()  // called once, returns &Config
}
```

### File-Based Tests

```rust
#[rstest]
fn parses_fixture(#[files("fixtures/*.json")] path: PathBuf) {
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(parse(&content).is_ok());
}
```

## Serde

Don't test derived serde. `#[derive(Serialize, Deserialize)]` is proven—testing
it just verifies serde works.

Test only:

- Custom `Serialize`/`Deserialize` impls
- `#[serde(flatten)]` (known edge cases with field conflicts)
- Complex validation logic in deserializers
- Roundtrip for types with lossy or conditional serialization
