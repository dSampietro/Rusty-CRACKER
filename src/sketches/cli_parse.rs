use std::env;

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "file", "provide the file containg the graph output file name", "FILEPATH");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
}