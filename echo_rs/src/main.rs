use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut x = 0;
    
    let arg_len = args.len() - 1;

    //dbg!(args);

    loop {
        print!("{}",&args[x].to_string());
        if x+1 > arg_len {
            break;
        }
        x = x + 1;
        print!(" ");
    }
    
}
