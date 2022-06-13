use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raekna_common::expression::Literal;
use raekna_compute::evaluate;
use raekna_parser::parse;

fn parse_and_evaluate(input: &str, variables: &mut HashMap<String, Literal>) {
    let ast = parse(input);
    let ast = ast.unwrap();
    evaluate(ast, variables).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut variables = HashMap::new();
    c.bench_function("parse and evaluate", |b| {
        b.iter(|| {
            variables.clear();
            parse_and_evaluate(
                black_box("var_def: pow(25, 5 / 2.0) * (1e2 + 2.2)"),
                &mut variables,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
