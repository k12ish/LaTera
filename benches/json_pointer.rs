#![feature(test)]
extern crate latera;
extern crate test;

#[bench]
fn bench_get_json_pointer(b: &mut test::Bencher) {
    b.iter(|| latera::get_json_pointer("foo.bar.baz"))
}

#[bench]
fn bench_get_json_pointer_with_map(b: &mut test::Bencher) {
    b.iter(|| latera::get_json_pointer("foo[\"http://example.com/\"].bar.baz"))
}
