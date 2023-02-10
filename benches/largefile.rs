use criterion::{criterion_group, criterion_main, Criterion};
use ffsr::lexer::Lexer;
use ffsr::reader::Reader;

const SOURCE: &str = include_str!("everything.sld");

fn parse_large_file() {
    let reader = Reader::from(Lexer::from(SOURCE));

    for datum in reader.iter() {
        match datum {
            Ok(datum) => {
                println!("{}: {}", datum.type_string(), datum);
                //println!("{}: {:#?}", datum.type_string(), datum);
            }
            Err(e) => {
                e.print(SOURCE);
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("large_file", |b| b.iter(parse_large_file));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
