use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use serde::{de::DeserializeSeed, Serialize};

use fhirbolt::model::{self, FhirRelease};

use test_utils::examples::{examples, ExampleFile, JsonOrXml};

pub fn bench(c: &mut Criterion) {
    let mut examples_json = examples(FhirRelease::R4, JsonOrXml::Json);

    let account_bytes_json = examples_json
        .get("account-example-with-guarantor.json")
        .read_to_vec();
    let account_json_struct =
        fhirbolt::json::from_slice::<model::r4::resources::Account>(&account_bytes_json, None)
            .unwrap();
    let account_json_element = fhirbolt::model::DeserializationContext::new(FhirRelease::R4, true)
        .deserialize(&mut serde_json::Deserializer::from_slice(
            &account_bytes_json,
        ))
        .unwrap();

    let mut examples_xml = examples(FhirRelease::R4, JsonOrXml::Xml);

    let account_bytes_xml = examples_xml
        .get("account-example-with-guarantor(ewg).xml")
        .read_to_vec();
    let account_xml_struct =
        fhirbolt::xml::from_slice::<model::r4::resources::Account>(&account_bytes_xml, None)
            .unwrap();
    let account_xml_element = fhirbolt::model::DeserializationContext::new(FhirRelease::R4, false)
        .deserialize(
            &mut fhirbolt::xml::de::Deserializer::from_slice::<fhirbolt::model::r4::Resource>(
                &account_bytes_xml,
            )
            .unwrap(),
        )
        .unwrap();

    let mut group = c.benchmark_group("json");
    group.throughput(Throughput::Bytes(account_bytes_json.len() as u64));
    group.bench_function("read account struct", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::json::from_slice::<model::r4::resources::Account>(
                    &account_bytes_json,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("read account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::model::DeserializationContext::new(FhirRelease::R4, true)
                    .deserialize(&mut serde_json::Deserializer::from_slice(
                        &account_bytes_json,
                    ))
                    .unwrap(),
            )
        })
    });
    group.bench_function("write account struct", |b| {
        b.iter(|| black_box(fhirbolt::json::to_vec(&account_json_struct)))
    });
    group.bench_function("write account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::model::SerializationContext::new(
                    &account_json_element,
                    FhirRelease::R4,
                    true,
                )
                .serialize(serde_json::value::Serializer)
                .unwrap(),
            )
        })
    });
    group.finish();

    let mut group = c.benchmark_group("xml");
    group.throughput(Throughput::Bytes(account_bytes_xml.len() as u64));
    group.bench_function("read account struct", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::xml::from_slice::<model::r4::resources::Account>(
                    &account_bytes_xml,
                    None,
                )
                .unwrap(),
            )
        })
    });
    group.bench_function("read account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::model::DeserializationContext::new(FhirRelease::R4, false)
                    .deserialize(
                        &mut fhirbolt::xml::de::Deserializer::from_slice::<
                            fhirbolt::model::r4::Resource,
                        >(&account_bytes_xml)
                        .unwrap(),
                    )
                    .unwrap(),
            )
        })
    });
    group.bench_function("write account struct", |b| {
        b.iter(|| black_box(fhirbolt::xml::to_vec(&account_xml_struct)))
    });
    group.bench_function("write account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::model::SerializationContext::new(
                    &account_xml_element,
                    FhirRelease::R4,
                    true,
                )
                .serialize(serde_json::value::Serializer)
                .unwrap(),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
