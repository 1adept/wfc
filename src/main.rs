use wfc_v2::*;

fn main() {
    const SIZE: u8 = 10;

    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;

    let mut pattern = Pattern::new();
    let a = pattern.add(a);
    let b = pattern.add(b);
    let c = pattern.add(c);
    let d = pattern.add(d);

    pattern.connect(&a, &b);
    pattern.connect(&a, &c);
    pattern.connect(&d, &b);
    pattern.connect(&d, &c);

    let wave = BasicWave::new(pattern);
    let result = wave.collapse(SIZE, SIZE);
    let is_valid = wave.validate(&result);

    println!(
        "{}\n{}",
        (if is_valid { "VALID" } else { "INVALID" }),
        result
    );
}
