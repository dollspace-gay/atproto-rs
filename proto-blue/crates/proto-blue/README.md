# proto-blue

Full-stack AT Protocol SDK for Rust.

This is the top-level facade crate that re-exports all proto-blue modules. Add a single dependency to get the entire SDK:

```toml
[dependencies]
proto-blue = "0.1"
```

## Modules

| Module | Description |
|--------|-------------|
| `syntax` | Identifier types: DID, Handle, NSID, AT-URI, TID, RecordKey, Datetime |
| `crypto` | P-256/K-256 key pairs, signing, did:key, SHA-256 |
| `lex_data` | Core Lexicon data model: `LexValue`, `BlobRef`, `Cid` |
| `lex_cbor` | DAG-CBOR encoding/decoding |
| `lex_json` | JSON ↔ Lexicon conversion |
| `common` | DID document parsing, TID generation, retry helpers |
| `lexicon` | Lexicon schema registry and validation |
| `repo` | Merkle Search Tree, CAR files, block storage |
| `xrpc` | XRPC HTTP client |
| `ws` | Auto-reconnecting WebSocket client |
| `identity` | DID and handle resolution with caching |
| `api` | Agent, rich text, moderation, session management |
| `oauth` | OAuth 2.0: DPoP, PKCE, PAR, token management |

## Usage

```rust
use proto_blue::api::Agent;
use proto_blue::syntax::Did;
use proto_blue::oauth::OAuthClient;
```

## License

Licensed under MIT OR Apache-2.0.

Part of the [proto-blue](https://github.com/dollspace-gay/proto-blue) AT Protocol SDK for Rust.
