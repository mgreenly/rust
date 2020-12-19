
//
// The next thing I want to know is what built in types I have to
// work with.
//


fn main() {

    // signed integer values 8 to 128 bits wide.
    let ia : i8   = 42;
    let ib : i16  = 43;
    let ic : i32  = 44;
    let id : i64  = 45;
    let ie : i128 = 46;

    println!("{}", ia);
    println!("{}", ib);
    println!("{}", ic);
    println!("{}", id);
    println!("{}", ie);

    // // unsigned integer values 8 to 128 bits wide.
    let ua : u8   = 42;
    let ub : u16  = 43;
    let uc : u32  = 44;
    let ud : u64  = 45;
    let ue : u128 = 46;

    println!("{}", ua);
    println!("{}", ub);
    println!("{}", uc);
    println!("{}", ud);
    println!("{}", ue);

    // // floating point values
    let f : f32 = 3.142;
    let g : f64 = 0.577;

    println!("{}", f);
    println!("{}", g);

    // // boolean values
    let h : bool = true;
    let i : bool = false;

    println!("{}", h);
    println!("{}", i);


    // single characters and string
    let j : char = 'c';
    let k = "foo baz";

    println!("{}", j);
    println!("{}", k);

    // tuples
    let tuple = ("hello", 5, 'C');
    let (l, m, n) = tuple;

    println!("{}, {}, {}", l, m, n);

    let o : [char; 3] = ['a', 'b', 'c'];
    println!("{}, {}, {}", o[0], o[1], o[2]);

    // unit is a value but we can't print it
    let _p : () = ();

}
