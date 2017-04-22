#!rust run

use std::env;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    match (args.get(1), args.get(2), args.get(3)) {
        (Some(x), Some(y), Some(z)) => println!("{x}時の{y}は{z}", x = x, y = y, z = z),
        (_, _, _) => println!("specify 3 words"),
    }
}
