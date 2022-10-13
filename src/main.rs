use wfc_v2::*;

fn main() {
    const SIZE: u8 = 25;

    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;

    let mut pattern = Pattern::new();
    let a = pattern.add(Module::Basic { value: a });
    let b = pattern.add(Module::Basic { value: b });
    let c = pattern.add(Module::Basic { value: c });
    let d = pattern.add(Module::Basic { value: d });

    pattern.connect_all(&vec![a, b, c]);
    pattern.connect_all(&vec![d, b, c]);

    let wave = Wave::new(pattern);

    let result = wave.collapse(SIZE, SIZE);
    let is_valid = wave.validate(&result);

    println!(
        "{}\n{}",
        (if is_valid { "VALID" } else { "INVALID" }),
        result
    );
}
