use std::fmt::Display;

use wfc_v2::*;

const SIZE: u8 = 25;

fn main() {
    do_basic();
    do_freq();
}

fn test() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;

    use grid::*;
    let pat = grid!(
        [0, 0, a, 0, 0,],
        [0, 0, a, 0, 0,],
        [c, b, a, 0, 0,],
        [0, a, 0, 0, d,],
        [0, a, a, a, d,]
    );
    println!("{}", pat);
}

fn do_freq() {
    let mut pattern = Pattern::new();
    let a = pattern.add(Module::Advanced {
        module: Box::new(Freq::new(1).limit(1)),
    });
    let b = pattern.add(Module::Advanced {
        module: Box::new(Freq::new(2).frequency(2)),
    });
    let c = pattern.add(Module::Advanced {
        module: Box::new(Freq::new(3).limit(5).frequency(5)),
    });
    let d = pattern.add(Module::Advanced {
        module: Box::new(Freq::new(4)),
    });
    let always = pattern.add(Module::Basic { value: 0 });

    pattern.connect_all(&vec![a, b, c]);
    pattern.connect_all(&vec![d, b, c]);
    pattern.connect_each(&always, &vec![a, b, c, d]);

    do_wave(pattern);
}

fn do_basic() {
    let mut pattern = Pattern::new();
    let a = pattern.add(Module::Basic { value: 1 });
    let b = pattern.add(Module::Basic { value: 2 });
    let c = pattern.add(Module::Basic { value: 3 });
    let d = pattern.add(Module::Basic { value: 4 });

    pattern.connect_all(&vec![a, b, c]);
    pattern.connect_all(&vec![d, b, c]);

    do_wave(pattern);
}

fn do_wave<T: Clone + PartialEq + Display>(pattern: Pattern<T>) {
    let mut wave = Wave::new(pattern);

    let result = wave.collapse(SIZE, SIZE);
    let is_valid = wave.validate(&result);

    println!(
        "{}\n{}",
        (if is_valid { "VALID" } else { "INVALID" }),
        result
    );
}
