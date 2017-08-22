extern crate test;

use brackets::brackets_paint;
use test::Bencher;

#[bench]
fn bench_many_brackets(b: &mut Bencher) {
    let brackets_str = (0..5000).map(|_| "[]").collect::<String>();
    b.iter(|| {
        // use `test::black_box` to prevent compiler optimizations from disregarding
        // unused values
        test::black_box(brackets_paint(0, &brackets_str, 0, ""));
    });
}
