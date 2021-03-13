use criterion::{criterion_group, criterion_main};

mod requests;
mod responses;

fn generate_headers<'a>(count: usize) -> stream_httparse::Headers<'a> {
    let mut result = stream_httparse::Headers::new();
    for i in 0..count {
        let key = format!("Key-{:03}", i);
        let value = format!("Value-{:03}", i);

        result.set(key, value);
    }
    result
}

fn generate_header_text(count: usize) -> String {
    let mut result = String::new();
    for i in 0..count {
        let key = format!("Key-{:03}", i);
        let value = format!("Value-{:03}", i);

        let line = format!("{}: {}\r\n", key, value);
        result.push_str(&line);
    }
    result
}

criterion_group!(
    benches,
    requests::parsing,
    requests::parse_finish,
    requests::serialize,
    responses::parsing,
    responses::parse_finish,
    responses::serialize
);
criterion_main!(benches);
