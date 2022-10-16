//! Deserialize and serialize FHIR resources to and from JSON.
//!
//! # Example
//! ```
//! # fn main() {
//! // The `Resource` type is an enum that contains all possible FHIR resources.
//! // If the resource type is known in advance, you could also use a concrete resource type
//! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
//! use fhirbolt::model::r4b::Resource as R4BResource;
//!
//! // The type of `s` is `&str`
//! let s = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
//!     <Observation xmlns=\"http://hl7.org/fhir\">
//!         <status value=\"final\"/>
//!         <code>
//!             <text value=\"some code\"/>
//!         </code>
//!         <valueString value=\"some value\"/>
//!     </Observation>";
//!
//! let r: R4BResource = fhirbolt::xml::from_str(s, None).unwrap();
//! println!("{:?}", r);
//! # }
//! ```

pub mod de;
//pub mod ser;

pub mod error;

mod number;
mod path;
mod read;

pub use de::{from_reader, from_slice, from_str, Deserializer};
//pub use ser::{to_string, Serializer};

pub use error::{Error, Result};
