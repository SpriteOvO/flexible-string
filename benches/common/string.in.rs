extern crate test;

use test::{black_box, Bencher};

#[bench]
fn bench_clone(bencher: &mut Bencher) {
    let string = BenchStringType::from("hello world! hello rust! hello flexible-string!");
    bencher.iter(|| string.clone())
}

#[bench]
fn bench_from_str(bencher: &mut Bencher) {
    bencher.iter(|| {
        BenchStringType::from(black_box("hello world! hello rust! hello flexible-string!"))
    })
}

#[bench]
fn bench_push(bencher: &mut Bencher) {
    bencher.iter(|| {
        BenchStringType::new().push(black_box('r'));
    })
}

#[bench]
fn bench_push_str(bencher: &mut Bencher) {
    bencher.iter(|| {
        BenchStringType::new().push_str(black_box("hello"));
    })
}
