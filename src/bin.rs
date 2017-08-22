use test::Bencher;

fn main() {

    let brackets_str = (0..5000).map(|_| "[]").collect::<String>();
    brackets_paint(0, &brackets_str, 0, "");
}
