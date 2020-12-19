// nothing to interesting here else is opitonal
// the expression between the if and the opening bracket must evaluate to a bool

fn main() {
    let mut a: i32 = 7;
    let b: i32;

    if a > 2 {
        b = 47;
    } else {
        b = 7;

    }

    // else is optional
    if (a == 7) && true {
     a = 1;
    }

    println!("a is {} and b is {}", a, b);
}
