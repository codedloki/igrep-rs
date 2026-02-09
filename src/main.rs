use std::env;
use std::fs;

fn main() {


    //// Taking command line arguements
    ////
    
    let args :Vec<String> = env::args().collect();
//    dbg!(&args);

    if args.len() < 3 {

        let s_color = "\x1b[33;1m";
        println!("\n {s_color} Usage : cargo run  < file_path > < word > \n");
        return;
    }
    let file_path = &args[1];
    let pattern = &args[2];


    let start_color = "\x1b[32;1m";
    let reset_color = "\x1b[0m";
//    println!("Received Query : {pattern}");
//    println!("Received Arguements : {} ",args);

//  File Reading 

//    let file_path : String = "src/test.txt".to_string();
 //   println!(" Received File  {}",file_path);
 //   println!("\n");
  //  println!("========================================");
   // println!("Reading file contents :  ");
 //   println!("==========================================\n");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines(){
        if line.contains(pattern) {
            let highlighted = line.replace(pattern,&format!("{start_color}{pattern}{reset_color}"));
            println!("{highlighted}")

    }
    }
//    println!("\n {contents}")




}
