use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use fhirbolt::{element::Element, FhirRelease};

use test_utils::examples::{examples, ExampleFile, JsonOrXml};

pub fn bench(c: &mut Criterion) {
    let mut examples_json = examples(FhirRelease::R4, JsonOrXml::Json);
    let mut examples_xml = examples(FhirRelease::R4, JsonOrXml::Xml);

    let account_bytes_json = examples_json
        .get("account-example-with-guarantor.json")
        .read_to_vec();
    let account_bytes_xml = examples_xml
        .get("account-example-with-guarantor(ewg).xml")
        .read_to_vec();

    let account_struct: fhirbolt::model::r4::resources::Account =
        fhirbolt::model::json::from_slice(&account_bytes_json, None).unwrap();
    let account_element: Element<{ FhirRelease::R4 }> =
        fhirbolt::element::json::from_slice(&account_bytes_json).unwrap();

    let mut group = c.benchmark_group("json");
    group.throughput(Throughput::Bytes(account_bytes_json.len() as u64));
    group.bench_function("read account struct", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::model::json::from_slice::<fhirbolt::model::r4::resources::Account>(
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
                fhirbolt::element::json::from_slice::<{ FhirRelease::R4 }>(&account_bytes_json)
                    .unwrap(),
            )
        })
    });
    group.bench_function("write account struct", |b| {
        b.iter(|| black_box(fhirbolt::model::json::to_vec(&account_struct)))
    });
    group.bench_function("write account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::element::json::to_vec::<{ FhirRelease::R4 }>(&account_element).unwrap(),
            )
        })
    });
    group.finish();

    let mut group = c.benchmark_group("xml");
    group.throughput(Throughput::Bytes(account_bytes_xml.len() as u64));
    group.bench_function("read account struct", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::model::xml::from_slice::<fhirbolt::model::r4::resources::Account>(
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
                fhirbolt::element::xml::from_slice::<{ FhirRelease::R4 }>(&account_bytes_xml)
                    .unwrap(),
            )
        })
    });
    group.bench_function("write account struct", |b| {
        b.iter(|| black_box(fhirbolt::model::xml::to_vec(&account_struct)))
    });
    group.bench_function("write account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::element::xml::to_vec::<{ FhirRelease::R4 }>(&account_element).unwrap(),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
