# Fhirbolt



Fhirbolt is a suite of libraries for working with FHIR resources.
It currently provides (de)serialization from and to JSON and XML for the Rust programming language.

Bindings for other languages are added in the future (using the Rust implementation as core).

More elaborate features like validation (including cardinality and slicing) or full FHIRPath evaluation might be added in the future.

Currenlty supported FHIR releases:
  * R4
  * R4B

## Language Bindings

Lang       | JSON  ser/de | XML ser/de | JSON<->XML<br>Conversion | Download
---------- | ------------ | ---------- | ------------------------ | --------
Rust       | ✓/✓          | tbd./✓     | tbd.                     | [![Rust]][crates.io]
Javascript | tbd./tbd.    | tbd./tbd.  | tbd.                     | 

[Rust]: https://img.shields.io/crates/v/fhirbolt.svg
[crates.io]: https://crates.io/crates/fhirbolt