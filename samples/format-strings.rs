

// notice how types are specified, var : type with return type as -> type
fn double(x: i32) -> i32 {
    return x * 2;
}

fn main() {
    // the template consumes arguments in order
    println!("{} - {}", double(3), double(7));
    // you can specify the index
    println!("{} - {} - {0}", double(3), double(7));
    // better you can used named arguments
    println!("{bar} {foo}", foo="FOO", bar="BAR");
    // you can format numbers (need to find the docs for this)
    println!("{val:>06}", val=32);
}
