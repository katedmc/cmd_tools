use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
/*
functionality to build out:
1. create blank file w/ expected name
2. add text from one file to final file
3. combine text from two files into final file
4. add command options
*/
fn main() {
    //let args: Vec<String> = env::args_os();

    let args: Vec<String> = env::args().collect();
    let mut input_files: Vec<String> = vec![String::new(); args.len()];
    let mut output_file = String::from("");
    let mut x = 0;
    let mut index = 0;
    let mut out_flag: bool = false;
    let mut input = String::new();
    let mut tmp;


    /*Next Steps 
    Confirm inputs are valid file names
    print out text of files
    concatenate to a new file
    */

    for arg in args {
        if arg == "->" {
            out_flag = true;
            continue;
        }
        if arg.contains("cat_rs.exe") == true {
            continue;
        }
        if arg.contains(".txt") == false{
            println!("Please enter valid parameters");
            return;
        }
        if out_flag == false {
            input_files[x] = arg;
            x = x + 1;
        }
        else {
            output_file = arg;
            break;
        }
    }

    while index < x {
        tmp = input_files[index].clone();
        let out = open_file(tmp).expect("Input file does not exist");
        input.push_str(&out);
        index = index + 1;
    }

    if out_flag == true && x == 0 {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
    }

    let _ = save_text(output_file, input).expect("Failed to concatenate"); 

}

fn open_file(in_f: String) -> std::io::Result<String> {
    let file = File::open(in_f)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{}", contents);

    Ok(contents)
}

fn save_text(out_f: String, f_text: String) -> std::io::Result<()> {
    let mut file = File::create(out_f)?;
    let _ = file.write_all(f_text.as_bytes());
    
    Ok(())
}

