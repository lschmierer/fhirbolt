use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fhirbolt::model::{self, FhirRelease};

use test_utils::examples::{examples, ExampleFile, JsonOrXml};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut examples = examples(FhirRelease::R4, JsonOrXml::Xml);

    let valuesets_bytes = examples.get("valuesets.xml").read_to_vec();
    let valuesets_bundle =
        fhirbolt::json::from_slice::<model::r4::resources::Bundle>(&valuesets_bytes, None).unwrap();

    c.bench_function("read valuesets", |b| {
        b.iter(|| {
            black_box(
                fhirbolt::xml::from_slice::<model::r4::resources::Bundle>(&valuesets_bytes, None)
                    .unwrap(),
            )
        })
    })
    .bench_function("write valuesets", |b| {
        b.iter(|| black_box(fhirbolt::xml::to_vec(&valuesets_bundle)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
);
criterion_main!(benches);
