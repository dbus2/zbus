use serde::{Deserialize, Serialize};
#[cfg(feature = "serde_bytes")]
use serde_bytes::ByteBuf;
use std::{collections::HashMap, vec};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use zvariant::{serialized::Context, to_bytes_for_signature, Type, Value, LE};

macro_rules! benchmark {
    ($c:ident, $data:ident, $data_type:ty, $func_prefix:literal) => {
        let signature = <$data_type>::signature();

        let ser_function_name = format!("{}_ser", $func_prefix);
        let de_function_name = format!("{}_de", $func_prefix);

        // Let's try with DBus format first
        let ctxt = Context::new_dbus(LE, 0);
        let mut group = $c.benchmark_group("dbus");
        group.measurement_time(std::time::Duration::from_secs(30));
        group.bench_function(&ser_function_name, |b| {
            b.iter(|| {
                let encoded = to_bytes_for_signature(
                    black_box(ctxt),
                    black_box(&signature),
                    black_box(&$data),
                )
                .unwrap();
                black_box(encoded);
            })
        });

        let encoded = to_bytes_for_signature(ctxt, &signature, &$data).unwrap();
        group.bench_function(&de_function_name, |b| {
            b.iter(|| {
                let (s, _): ($data_type, _) = encoded
                    .deserialize_for_signature(black_box(&signature))
                    .unwrap();
                black_box(s);
            })
        });
        group.finish();

        // Now GVariant.
        #[cfg(feature = "gvariant")]
        {
            let ctxt = Context::new_gvariant(LE, 0);
            let mut group = $c.benchmark_group("gvariant");
            group.measurement_time(std::time::Duration::from_secs(30));

            group.bench_function(&ser_function_name, |b| {
                b.iter(|| {
                    let encoded = to_bytes_for_signature(
                        black_box(ctxt),
                        black_box(&signature),
                        black_box(&$data),
                    )
                    .unwrap();
                    black_box(encoded);
                })
            });

            let encoded = to_bytes_for_signature(ctxt, &signature, &$data).unwrap();
            group.bench_function(&de_function_name, |b| {
                b.iter(|| {
                    let (s, _): ($data_type, _) = encoded
                        .deserialize_for_signature(black_box(&signature))
                        .unwrap();
                    black_box(s);
                })
            });
            group.finish();
        }
    };
}

#[cfg(feature = "serde_bytes")]
fn byte_array(c: &mut Criterion) {
    let ay = ByteBuf::from(vec![77u8; 100_000]);

    benchmark!(c, ay, ByteBuf, "byte_array");
}

fn fixed_size_array(c: &mut Criterion) {
    let ay = vec![77u8; 100_000];

    benchmark!(c, ay, Vec<u8>, "fixed_size_array");
}

fn big_array(c: &mut Criterion) {
    #[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
    struct BigArrayField<'f> {
        int2: u64,
        string2: &'f str,
    }

    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct BigArrayStruct<'s> {
        string1: &'s str,
        int1: u64,
        field: BigArrayField<'s>,
        dict: HashMap<&'s str, Value<'s>>,
        int_array: Vec<u64>,
        string_array: Vec<&'s str>,
    }

    let mut dict = HashMap::new();
    let int_array = vec![0u64; 1024 * 10];
    let mut strings = Vec::new();
    let mut string_array: Vec<&str> = Vec::new();
    for idx in 0..1024 * 10 {
        strings.push(format!(
            "{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}"
        ));
    }
    for s in &strings {
        string_array.push(s.as_str());
        dict.insert(s.as_str(), Value::from(s.as_str()));
    }

    let data = BigArrayStruct {
        string1: "Testtest",
        int1: 0xFFFFFFFFFFFFFFFFu64,
        field: BigArrayField {
            string2: "TesttestTestest",
            int2: 0xFFFFFFFFFFFFFFFFu64,
        },
        int_array,
        string_array,
        dict,
    };

    benchmark!(c, data, BigArrayStruct<'_>, "big_array");
}

#[cfg(feature = "serde_bytes")]
criterion_group!(benches, big_array, byte_array, fixed_size_array);
#[cfg(not(feature = "serde_bytes"))]
criterion_group!(benches, big_array, fixed_size_array);
criterion_main!(benches);
