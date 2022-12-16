use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use fhirbolt::model::{self, FhirRelease};

use test_utils::examples::{examples, ExampleFile, JsonOrXml};

pub fn bench(c: &mut Criterion) {
    let mut examples_json = examples(FhirRelease::R4, JsonOrXml::Json);

    let account_bytes_json = examples_json
        .get("account-example-with-guarantor.json")
        .read_to_vec();
    let account_json =
        fhirbolt::json::from_slice::<model::r4::resources::Account>(&account_bytes_json, None)
            .unwrap();

    let mut examples_xml = examples(FhirRelease::R4, JsonOrXml::Xml);

    let account_bytes_xml = examples_xml
        .get("account-example-with-guarantor(ewg).xml")
        .read_to_vec();
    let account_xml =
        fhirbolt::xml::from_slice::<model::r4::resources::Account>(&account_bytes_xml, None)
            .unwrap();

    let mut group = c.benchmark_group("json");
    group.throughput(Throughput::Bytes(account_bytes_json.len() as u64));
    group.bench_function("read account", |b| {
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
    group.bench_function("write account", |b| {
        b.iter(|| black_box(fhirbolt::json::to_vec(&account_json)))
    });
    group.finish();

    let mut group = c.benchmark_group("xml");
    group.throughput(Throughput::Bytes(account_bytes_xml.len() as u64));
    group.bench_function("read account", |b| {
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
    group.bench_function("write account", |b| {
        b.iter(|| black_box(fhirbolt::xml::to_vec(&account_xml)))
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
