use std::{fs, path::PathBuf};

use fhirbolt::model::r5::{
    resources::{Bundle, BundleEntry},
    types::{Code, Uri},
    Resource,
};

use map::map_patient;

use crate::map::{map_conditions, map_encounter};

mod hl7v2;
mod map;

const DATA_FOLDER: &str = "examples/hl7v2-transform-manual/data";
const OUT_FOLDER: &str = "examples/hl7v2-transform-manual/out";

const BASE_URL: &str = "https://example.org/fhir";

fn main() {
    println!("reading data dir...");

    let paths = fs::read_dir(DATA_FOLDER).expect("Error: reading opening dir!");

    let mut all_resources = vec![];

    let mut patient_id = -1;

    for dir_entry_result in paths {
        patient_id += 1;

        let dir_entry = dir_entry_result.expect("Error: reading data dir!");
        let path = dir_entry.path();

        println!("parsing `{}`...", path.display());

        let message_str = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Error reading message `{}`", path.display()));

        let message = hl7v2::parse(&message_str)
            .map_err(|err| format!("Error: {}", err))
            .unwrap();

        println!("generating FHIR resources...");

        let patient_id = format!("patient{}", patient_id);
        let encounter_id = format!("{}encounter0", patient_id);

        let patient = map_patient(&message, &patient_id);
        let encounter = map_encounter(&message, &encounter_id, &patient_id);
        let conditions = map_conditions(&message, &patient_id, &encounter_id);

        all_resources.push(Resource::Patient(Box::new(patient)));
        all_resources.push(Resource::Encounter(Box::new(encounter)));
        all_resources.extend(
            conditions
                .into_iter()
                .map(Box::new)
                .map(Resource::Condition),
        )
    }

    let bundle = build_bundle(all_resources);

    println!("converting to JSON...");
    let json_bundle =
        fhirbolt::json::to_string_pretty(&bundle, None).expect("Error: serializing to JSON");

    println!("saving `bundle.json`...");

    fs::create_dir_all(OUT_FOLDER).expect("Error: creating output direcotry");
    fs::write(PathBuf::from(OUT_FOLDER).join("bundle.json"), json_bundle)
        .expect("Error: writing `bundle.json`");

    println!("Transformation \x1B[32msuccesful\x1B[39m! 🚀")
}

/// Build a FHIR bundle from a list of resources.
fn build_bundle(resources: Vec<Resource>) -> Bundle {
    Bundle {
        r#type: Code {
            value: Some("collection".into()),
            ..Default::default()
        },
        entry: resources
            .into_iter()
            .map(|r| BundleEntry {
                full_url: Some(Uri {
                    value: Some(format!(
                        "{}/{}",
                        BASE_URL,
                        r.id()
                            .and_then(|i| i.value.as_ref())
                            .expect("Error: resource is missing an id")
                    )),
                    ..Default::default()
                }),
                resource: Some(r),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }
}
