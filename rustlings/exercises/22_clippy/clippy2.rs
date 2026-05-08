fn main() {
    let mut res = 42;
    let option: Option<i32> = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
