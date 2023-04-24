# Example manual HL7 v2 to FHIR transformation

This example demonstrates a simple transformation of HL7 v2 ADT A01 messages to FHIR resources.
It implements a basic HL7 v2 parser (CAVE: does not handle escapes) and some mappings to FHIR resources.

Use following command from fhirbolt project root to run the example:
```
cargo run -p example-hl7v2-transform-manual
```

## Limitations
* Uses only PID.3 as patient identifier (does not use the backwards compatibility fields PID.2 and PID.4)
* CodeSystem `http://terminology.hl7.org/CodeSystem/v3-MaritalStatus` is assumed for maritalStatus
* Skipped the insurance related segments (IN*)
* Skipped the German extension segments (Z*)
* Does not support subseconds and timezones