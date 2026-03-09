//! # proto-blue
//!
//! Full-stack AT Protocol SDK for Rust.
//!
//! This crate re-exports all proto-blue modules so you can access the entire
//! SDK from a single dependency:
//!
//! ```toml
//! [dependencies]
//! proto-blue = "0.1"
//! ```
//!
//! ```rust,no_run
//! use proto_blue::api::Agent;
//! use proto_blue::syntax::Did;
//! ```

/// AT Protocol identifier types: DID, Handle, NSID, AT-URI, TID, RecordKey, Datetime.
pub use proto_blue_syntax as syntax;

/// Cryptographic primitives: P-256/K-256 key pairs, signing, did:key, SHA-256.
pub use proto_blue_crypto as crypto;

/// Core Lexicon data model: `LexValue`, `BlobRef`, `Cid`.
pub use proto_blue_lex_data as lex_data;

/// DAG-CBOR encoding and decoding with deterministic map ordering.
pub use proto_blue_lex_cbor as lex_cbor;

/// JSON ↔ Lexicon value conversion with `$link`/`$bytes` encoding.
pub use proto_blue_lex_json as lex_json;

/// Shared utilities: DID document parsing, TID generation, retry helpers.
pub use proto_blue_common as common;

/// Lexicon schema types, registry, and validation engine.
pub use proto_blue_lexicon as lexicon;

/// Repository primitives: Merkle Search Tree, CAR files, block storage.
pub use proto_blue_repo as repo;

/// XRPC HTTP client for AT Protocol queries and procedures.
pub use proto_blue_xrpc as xrpc;

/// Auto-reconnecting WebSocket client for event streams.
pub use proto_blue_ws as ws;

/// DID and handle resolution with caching.
pub use proto_blue_identity as identity;

/// High-level API: Agent, rich text, moderation, session management.
pub use proto_blue_api as api;

/// OAuth 2.0 client: DPoP, PKCE, PAR, token management.
pub use proto_blue_oauth as oauth;
