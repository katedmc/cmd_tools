use std::env;
use std::fs;
/*
ls -> list directory contents
"ls [option] [file/directory]"
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut x = 0;
    let mut abs;

    /*Overview of algorithm
    parse options
    parse directory
        -parse absolute if given relative
    'open' directory location
    get details based on options
    */

    //Put parse options here

    for arg in args {
        if x == 0 {
            abs = arg.clone().split('\\');
            x = x + 1;
            continue;
        }
        //If statement for options when implemented

        
        
    }


    
}

fn print_dir(f_path: String) {
    let paths = fs::read_dir(f_path).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}