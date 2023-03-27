# Fhirbolt - (Experimental) FHIR library for Rust (and maybe 🔜 JS)

Fhirbolt is an experimental suite of libraries for working with FHIR resources.
It currently provides serialization and deserialization of JSON and XML resources for the Rust programming language.

Bindings for other languages shall be added in the future.
Primary goal is adding JS bindings by compiling the core Rust crate to WASM.

More elaborate features like validation (including cardinality and slicing) or full FHIRPath evaluation might be added eventually.

Currenlty supported FHIR releases:
  * R4
  * R4B

## Language Support

Lang       | JSON  ser/de | XML ser/de | Package              |                 |
---------- | ------------ | ---------- | -------------------- | --------------- |
Rust       | ✓/✓          | ✓/✓        | [![Rust]][crates.io] | [Docs][docs.rs] |
Javascript | tbd./tbd.    | tbd./tbd.  |                      |                 |

[Rust]: https://img.shields.io/crates/v/fhirbolt.svg
[docs.rs]: https://docs.rs/fhirbolt
[crates.io]: https://crates.io/crates/fhirbolt
