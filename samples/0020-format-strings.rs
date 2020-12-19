
//
// The first thing I want to know when learning a language is how to
// define variables and how to display values to the screen.  The
// necessary tools for exploring just about everything else.
//

fn main() {

    let i : i32 = 37;
    let j : i32 = 42;

    // the template consumes arguments in order
    println!("{} - {}", i, j);
    // you can specify the index
    println!("{} - {} - {0}", i, j);
    // better you can used named arguments
    println!("{bar} {foo}", foo=i, bar=j);
    // you can format numbers (need to find the docs for this)
    println!("{val:>06}", val=32);
}
