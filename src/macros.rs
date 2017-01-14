macro_rules! bitwise {
    ($a:expr, $op:tt, $b:expr) => (
        {
            let result = $a $op $b;
            println!("{:>8b} = {:?}", $a, $a);
            println!("{:>8b} = {:?}", $b, $b);
            println!("-------- = {:?}", stringify!($op));
            println!("{:>8b} = {:?}", result, result);
        }
    );
}

macro_rules! register_eq {
    ($chip:tt, $vx:expr, $value:expr) => (assert_eq!($chip.register.get($vx), $value);)
}
