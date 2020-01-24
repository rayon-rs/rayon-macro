use rayon_macro::parallel;

#[test]
fn print10() {
    parallel!(for i in 0..10 {
        println!("{}", i);
    })
}

#[test]
fn print10_odd() {
    parallel!('outer: for i in 0..20 {
        for _ in 0..10 {
            continue;
        }
        'inner: for _ in 0..10 {
            if i % 4 == 0 {
                continue 'outer;
            }
            continue 'inner;
        }
        'outer: for _ in 0..10 {
            continue 'outer;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    })
}
