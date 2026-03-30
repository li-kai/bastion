# bastion

Secure HTTP proxy for AI coding agents. Enforces egress policies and injects credentials for sandboxed agents — so agents never see secrets.

## How it works

```
┌──────────────────────────────┐
│  Sandbox (VM / devcontainer) │
│                              │
│  AI Agent                    │
│  - No credentials            │
│  - All traffic via proxy     │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│  Bastion Proxy               │
│                              │
│  - TLS interception           │
│  - SSRF protection           │
│  - Egress allowlist          │
│  - Credential injection      │
│  - Audit logging             │
└──────────────────────────────┘
```

Agents make normal HTTP/HTTPS requests. The proxy intercepts TLS, evaluates egress policy, injects credentials for authorized hosts, and blocks everything else.

## Key properties

- **Agents never see credentials** — no sealed secrets, no dummy tokens
- **Defense in depth** — allowlists, SSRF protection, credential scoping, audit logging
- **Library-first** — embeddable in orchestrators via Rust API
- **Hot-reload** — zero-downtime config updates

## Configuration

```toml
[proxy]
listen = "127.0.0.1:8080"

[egress]
default_policy = "block"
passthrough_hosts = ["crates.io", "registry.npmjs.org"]
blocked_hosts = ["pastebin.com"]

[credentials.openai]
preset = "openai"
env = "OPENAI_API_KEY"

[credentials.github]
preset = "github"
env = "GITHUB_TOKEN"
```
