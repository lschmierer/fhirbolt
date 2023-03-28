use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use zip::read::{ZipArchive, ZipFile};

use fhirbolt::FhirRelease;

const BASE_URL: &str = "http://hl7.org/fhir/{release}/{file}";

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
    pub fn next(&mut self) -> Option<ZipFile> {
        if self.index < self.archive.len() {
            let index = self.index;

            self.index += 1;

            Some(self.archive.by_index(index).unwrap())
        } else {
            None
        }
    }

    pub fn get(&mut self, name: &str) -> ZipFile {
        self.archive.by_name(name).unwrap()
    }
}

pub fn examples(fhir_release: FhirRelease, json_or_xml: JsonOrXml) -> ExamplesIterator {
    let zip_path = download_fhir_examples(fhir_release, json_or_xml);

    let zip_file = fs::File::open(zip_path).expect("Error opening zip file");
    let archive = ZipArchive::new(zip_file).unwrap();

    ExamplesIterator {
        archive: archive,
        index: if fhir_release == FhirRelease::R4B {
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
        .join(fhir_release.to_string().to_lowercase());
    let zip_path = folder_path.join(json_or_xml.as_str());

    if !zip_path.exists() {
        fs::create_dir_all(folder_path).unwrap();

        let bytes = reqwest::blocking::get(download_url)
            .and_then(|r| r.bytes())
            .expect(&format!("Error downloading {} examples", fhir_release));
        fs::write(&zip_path, bytes).expect(&format!("Error writing \"{:?}\"", zip_path));

        println!(" Download complete!");
    }

    zip_path
}

fn fhir_examples_url(fhir_release: FhirRelease, json_or_xml: JsonOrXml) -> String {
    let url = str::replace(
        BASE_URL,
        "{release}",
        &fhir_release.to_string().to_lowercase(),
    );
    str::replace(&url, "{file}", json_or_xml.as_str())
}
