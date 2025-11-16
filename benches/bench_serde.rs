use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use fhirbolt::{element::Element, serde::SerializationConfig, FhirReleases};
use helios_context_fhir::r4::{Account as HeliosContextAccount, Resource as HeliosContextResource};
use helios_context_serde::json as helios_context_json;
use helios_fhir::r4::{Account as HeliosAccount, Resource as HeliosResource};

use test_utils::examples::{examples, ExampleFile, JsonOrXml};

pub fn bench(c: &mut Criterion) {
    let mut examples_json = examples(FhirReleases::R4, JsonOrXml::Json);
    let mut examples_xml = examples(FhirReleases::R4, JsonOrXml::Xml);

    let account_bytes_json = examples_json
        .get("account-example-with-guarantor.json")
        .read_to_vec();
    let account_bytes_xml = examples_xml
        .get("account-example-with-guarantor(ewg).xml")
        .read_to_vec();

    let account_struct: fhirbolt::model::r4::resources::Account =
        fhirbolt::json::from_slice(&account_bytes_json, None).unwrap();
    let account_enum: fhirbolt::model::r4::Resource =
        fhirbolt::json::from_slice(&account_bytes_json, None).unwrap();
    let account_element: Element<{ FhirReleases::R4 }> =
        fhirbolt::json::from_slice(&account_bytes_json, None).unwrap();
    let helios_account_struct: HeliosAccount = serde_json::from_slice(&account_bytes_json).unwrap();
    let helios_account_enum: HeliosResource = serde_json::from_slice(&account_bytes_json).unwrap();
    let helios_context_account_struct: HeliosContextAccount =
        helios_context_json::from_slice(&account_bytes_json).unwrap();
    let helios_context_account_enum: HeliosContextResource =
        helios_context_json::from_slice(&account_bytes_json).unwrap();

    let mut group = c.benchmark_group("json");
    group.throughput(Throughput::Bytes(account_bytes_json.len() as u64));
    group.bench_function("read account struct (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::json::from_slice::<fhirbolt::model::r4::resources::Account>(
                    &account_bytes_json,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("read account enum (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::json::from_slice::<fhirbolt::model::r4::Resource>(
                    &account_bytes_json,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("read account element (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::json::from_slice::<Element<{ FhirReleases::R4 }>>(
                    &account_bytes_json,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("write account struct (fhirbolt)", |b| {
        b.iter(|| {
            black_box(fhirbolt::json::to_vec(
                &account_struct,
                Some(SerializationConfig { always_sort: false }),
            ))
        })
    });
    group.bench_function("write account enum (fhirbolt)", |b| {
        b.iter(|| {
            black_box(fhirbolt::json::to_vec(
                &account_enum,
                Some(SerializationConfig { always_sort: false }),
            ))
        })
    });
    group.bench_function("write account element (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::json::to_vec::<Element<{ FhirReleases::R4 }>>(
                    &account_element,
                    Some(SerializationConfig { always_sort: false }),
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("read account struct (helios main)", |b| {
        b.iter(|| black_box(serde_json::from_slice::<HeliosAccount>(&account_bytes_json).unwrap()))
    });
    group.bench_function("read account enum (helios main)", |b| {
        b.iter(|| black_box(serde_json::from_slice::<HeliosResource>(&account_bytes_json).unwrap()))
    });
    group.bench_function("write account struct (helios main)", |b| {
        b.iter(|| black_box(serde_json::to_vec(&helios_account_struct)))
    });
    group.bench_function("write account enum (helios main)", |b| {
        b.iter(|| black_box(serde_json::to_vec(&helios_account_enum)))
    });
    group.bench_function("read account struct (helios serde-context)", |b| {
        b.iter(|| {
            black_box(
                helios_context_json::from_slice::<HeliosContextAccount>(&account_bytes_json)
                    .unwrap(),
            )
        })
    });
    group.bench_function("read account enum (helios serde-context)", |b| {
        b.iter(|| {
            black_box(
                helios_context_json::from_slice::<HeliosContextResource>(&account_bytes_json)
                    .unwrap(),
            )
        })
    });
    group.bench_function("write account struct (helios serde-context)", |b| {
        b.iter(|| black_box(helios_context_json::to_vec(&helios_context_account_struct)))
    });
    group.bench_function("write account enum (helios serde-context)", |b| {
        b.iter(|| black_box(helios_context_json::to_vec(&helios_context_account_enum)))
    });
    group.finish();

    let mut group = c.benchmark_group("xml");
    group.throughput(Throughput::Bytes(account_bytes_xml.len() as u64));
    group.bench_function("read account struct (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::xml::from_slice::<fhirbolt::model::r4::resources::Account>(
                    &account_bytes_xml,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("read account enum (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::xml::from_slice::<fhirbolt::model::r4::Resource>(
                    &account_bytes_xml,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("read account element (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::xml::from_slice::<Element<{ FhirReleases::R4 }>>(
                    &account_bytes_xml,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("write account struct (fhirbolt)", |b| {
        b.iter(|| {
            black_box(fhirbolt::xml::to_vec(
                &account_struct,
                Some(SerializationConfig { always_sort: false }),
            ))
        })
    });
    group.bench_function("write account enum (fhirbolt)", |b| {
        b.iter(|| {
            black_box(fhirbolt::xml::to_vec(
                &account_enum,
                Some(SerializationConfig { always_sort: false }),
            ))
        })
    });
    group.bench_function("write account element (fhirbolt)", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::xml::to_vec::<Element<{ FhirReleases::R4 }>>(
                    &account_element,
                    Some(SerializationConfig { always_sort: false }),
                )
                .unwrap(),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
