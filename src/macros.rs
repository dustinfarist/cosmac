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
