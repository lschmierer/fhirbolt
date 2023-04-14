use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use fhirbolt::{element::Element, FhirReleases};

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

    let mut group = c.benchmark_group("json");
    group.throughput(Throughput::Bytes(account_bytes_json.len() as u64));
    group.bench_function("read account struct", |b| {
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
    group.bench_function("read account enum", |b| {
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
    group.bench_function("read account element", |b| {
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
    group.bench_function("write account struct", |b| {
        b.iter(|| black_box(fhirbolt::json::to_vec(&account_struct)))
    });
    group.bench_function("write account enum", |b| {
        b.iter(|| black_box(fhirbolt::json::to_vec(&account_enum)))
    });
    group.bench_function("write account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::json::to_vec::<Element<{ FhirReleases::R4 }>>(&account_element).unwrap(),
            )
        })
    });
    group.finish();

    let mut group = c.benchmark_group("xml");
    group.throughput(Throughput::Bytes(account_bytes_xml.len() as u64));
    group.bench_function("read account struct", |b| {
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
    group.bench_function("read account enum", |b| {
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
    group.bench_function("read account element", |b| {
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
    group.bench_function("write account struct", |b| {
        b.iter(|| black_box(fhirbolt::xml::to_vec(&account_struct)))
    });
    group.bench_function("write account enum", |b| {
        b.iter(|| black_box(fhirbolt::xml::to_vec(&account_enum)))
    });
    group.bench_function("write account element", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::xml::to_vec::<Element<{ FhirReleases::R4 }>>(&account_element).unwrap(),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
