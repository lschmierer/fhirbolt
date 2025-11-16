use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use reqwest::blocking::Client;
use zip::read::{ZipArchive, ZipFile};

use fhirbolt::{FhirRelease, FhirReleases};

const BASE_URL: &str = "http://hl7.org/fhir/{release}/{file}";
const DOWNLOAD_USER_AGENT: &str =
    "Mozilla/5.0 (compatible; fhirbolt-tests/0.1; +https://github.com/FHIR/fhirbolt)";

fn fhir_release_name(fhir_release: FhirRelease) -> &'static str {
    match fhir_release {
        FhirReleases::R4 => "r4",
        FhirReleases::R4B => "r4b",
        FhirReleases::R5 => "r5",
        _ => panic!("unknown FHIR release"),
    }
}

#[derive(Clone, Copy)]
pub enum JsonOrXml {
    Json,
    Xml,
}

impl JsonOrXml {
    fn as_str(&self) -> &'static str {
        match self {
            JsonOrXml::Json => "examples-json.zip",
            JsonOrXml::Xml => "examples.zip",
        }
    }
}

pub trait ExampleFile {
    fn read_to_vec(self) -> Vec<u8>;
}

impl ExampleFile for ZipFile<'_> {
    fn read_to_vec(mut self) -> Vec<u8> {
        let mut vec = Vec::new();
        self.read_to_end(&mut vec).unwrap();
        vec
    }
}

pub struct ExamplesIterator {
    archive: ZipArchive<File>,
    index: usize,
}

impl ExamplesIterator {
    pub fn next_example(&mut self) -> Option<ZipFile<'_>> {
        if self.index < self.archive.len() {
            let index = self.index;

            self.index += 1;

            Some(self.archive.by_index(index).unwrap())
        } else {
            None
        }
    }

    pub fn get(&mut self, name: &str) -> ZipFile<'_> {
        self.archive.by_name(name).unwrap()
    }
}

pub fn examples(fhir_release: FhirRelease, json_or_xml: JsonOrXml) -> ExamplesIterator {
    let zip_path = download_fhir_examples(fhir_release, json_or_xml);

    let zip_file = fs::File::open(zip_path).expect("Error opening zip file");
    let archive = ZipArchive::new(zip_file).unwrap();

    ExamplesIterator {
        archive,
        index: if fhir_release == FhirReleases::R4B {
            1
        } else {
            0
        },
    }
}

fn download_fhir_examples(fhir_release: FhirRelease, json_or_xml: JsonOrXml) -> PathBuf {
    let download_url = fhir_examples_url(fhir_release, json_or_xml);
    let folder_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("target")
        .join("tmp")
        .join("fhirbolt")
        .join(fhir_release_name(fhir_release));
    let zip_path = folder_path.join(json_or_xml.as_str());

    if zip_path.exists() && !is_valid_zip(&zip_path) {
        fs::remove_file(&zip_path)
            .unwrap_or_else(|err| panic!("Error removing \"{:?}\": {}", zip_path, err));
    }

    if !zip_path.exists() {
        fs::create_dir_all(folder_path).unwrap();

        download_examples_zip(&download_url, &zip_path, fhir_release, json_or_xml.as_str());

        println!(" Download complete!");
    }

    zip_path
}

fn fhir_examples_url(fhir_release: FhirRelease, json_or_xml: JsonOrXml) -> String {
    let url = str::replace(BASE_URL, "{release}", fhir_release_name(fhir_release));
    str::replace(&url, "{file}", json_or_xml.as_str())
}

fn download_examples_zip(
    download_url: &str,
    zip_path: &Path,
    fhir_release: FhirRelease,
    file_name: &str,
) {
    let client = Client::builder()
        .user_agent(DOWNLOAD_USER_AGENT)
        .build()
        .unwrap();

    let response = client
        .get(download_url)
        .send()
        .unwrap_or_else(|err| {
            panic!(
                "Error downloading {} {}: {}",
                fhir_release_name(fhir_release),
                file_name,
                err
            )
        })
        .error_for_status()
        .unwrap_or_else(|err| {
            let status = err
                .status()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".to_string());
            panic!(
                "Error downloading {} {}: HTTP {}",
                fhir_release_name(fhir_release),
                file_name,
                status
            );
        });

    let bytes = response.bytes().unwrap_or_else(|err| {
        panic!(
            "Error downloading {} {}: {}",
            fhir_release_name(fhir_release),
            file_name,
            err
        )
    });
    fs::write(zip_path, bytes)
        .unwrap_or_else(|err| panic!("Error writing \"{:?}\": {}", zip_path, err));

    if !is_valid_zip(zip_path) {
        panic!(
            "Downloaded {} {} but {:?} is not a valid ZIP archive",
            fhir_release_name(fhir_release),
            file_name,
            zip_path
        );
    }
}

fn is_valid_zip(path: &Path) -> bool {
    File::open(path)
        .ok()
        .and_then(|file| ZipArchive::new(file).ok())
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Hits the public HL7 download endpoints"]
    fn downloads_r4_json_examples() {
        let mut iterator = examples(FhirReleases::R4, JsonOrXml::Json);
        assert!(iterator.next_example().is_some());
    }
}
