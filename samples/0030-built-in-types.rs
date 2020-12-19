



fn main() {
    let a : i8 = 42;
    println!("{}", a);
    let b : i16 = 43;
    println!("{}", b);
    let c : i32 = 44;
    println!("{}", c);
    let d : i64 = 45;
    println!("{}", d);
    let e : i128 = 46;
    println!("{}", e);

    let f : u8 = 47;
    println!("{}", f);
    let g : u128 = 49;
    println!("{}", g);


    let h : bool = true;
    println!("{}", h);
    let i : bool = false;
    println!("{}", i);

    let j : f32 = 42.0;
    println!("{:.4}", j);
    let k : f64 = 43.0;
    println!("{:.2}", k);

    let l : char = 'c';
    println!("{}", l);

    let m : (i32, bool) = (0, false);
    let (m1, m2) = m;
    println!("{}, {}", m1, m2);
}
