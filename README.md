# Fhirbolt - (Experimental) FHIR library for Rust (and maybe 🔜 JS)

Fhirbolt is an experimental suite of libraries for working with FHIR resources.
It currently provides serialization and deserialization of JSON and XML resources for the Rust programming language.

Bindings for other languages shall be added in the future.
Primary goal is adding JS bindings by compiling the core Rust crate to WASM.

More elaborate features like validation (including cardinality and slicing) or full FHIRPath evaluation might be added eventually.

Currenlty supported FHIR releases: R4, R4B and R5

## Roadmap
- [x] JSON (de)serialization
- [x] XML (de)serialization
- [ ] FHIRPath
- [ ] JS/TS package (by compiling to WASM)

## Language Support

Lang       | JSON  ser/de | XML ser/de | Package              |                 |
---------- | ------------ | ---------- | -------------------- | --------------- |
Rust       | ✓/✓          | ✓/✓        | [![Rust]][crates.io] | [Docs][docs.rs] |
Javascript | tbd./tbd.    | tbd./tbd.  |                      |                 |

[Rust]: https://img.shields.io/crates/v/fhirbolt.svg
[docs.rs]: https://docs.rs/fhirbolt
[crates.io]: https://crates.io/crates/fhirbolt

## Rust

The Rust crate supports two working modes:
1. a generic element model
2. working with fully typed model structs.

The element model is always enabled as it is also used internally for deserializing model structs.
Model structs can be optionally enabled by specifying the desried FHIR release as Cargo feature.

You should only include the release that you really need, as this signigicantly increases build time.

```toml
[dependencies]
fhirbolt = { version = "0.4", features = ["r4b"] }
```

### Example

```rust
// The `Resource` type is an enum that contains all possible FHIR resources.
// If the resource type is known in advance, you could also use a concrete resource type
// (like e.g. `fhirbolt::model::r4b::resources::Observation`).
use fhirbolt::model::r4b::Resource;
use fhirbolt::serde::{DeserializationConfig, DeserializationMode};

// The type of `s` is `&str`
let s = r#"{
    "resourceType": "Observation",
    "status": "final",
    "code": {
        "text": "some code"
    },
    "valueString": "some value"
}"#;

let r: Resource = fhirbolt::json::from_str(s, None).unwrap();

match r {
    Resource::Observation(o) => println!("deserialized observation: {:?}", r),
    _ => (),
}

// Use Default::default() or constructing new resources by yourself
let o = Observation {
    status: "final".into(),
    code: Box::new(CodeableConcept {
        text: Some("some code".into()),
        ..Default::default()
    }),
    value: Some(ObservationValue::String("some value".into())),
    ..Default::default()
};
```

## Benchmarks

The JSON and XML benchmarks live in `benches/bench_serde.rs`. To run them with
the recommended stack size, execute:

```bash
RUST_MIN_STACK=33554432 cargo bench -p benches bench_serde
```

The output includes Criterion measurements for the fhirbolt, helios main, and
helios serde-context implementations. You can pipe that output into
`scripts/compare_json_benches.py` to quickly compare the JSON-only benchmarks
(element-based measurements are automatically ignored):

```bash
RUST_MIN_STACK=33554432 cargo bench -p benches bench_serde \
    | ./scripts/compare_json_benches.py
```

The script reports the median estimate for each implementation and the relative
speed-up or slowdown versus your chosen baseline (default: fhirbolt). Pass
`--baseline helios-main` (or `helios-serde-context`) to compare against a
different implementation.
