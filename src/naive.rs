use getopts::Options;
use std::env;

mod io_util;
use io_util::read_from_file;

use rustworkx_core::{connectivity::{connected_components, number_connected_components}, petgraph::graphmap::UnGraphMap};


// ~20 ms / 50k edges

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    type V = u32;

    //get cli args
    let args: Vec<String> = std::env::args().collect();

    //get opts
    let mut opts = Options::new();
    opts.optopt(
        "f",
        "file",
        "provide the file containg the graph output file name",
        "FILEPATH",
    );
    opts.optflag("h", "help", "print help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    //handle -h/--help
    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", args[0]);
        print!("{}", opts.usage(&brief));

        return;
    }

    let filename = matches.opt_str("f");
    if filename.is_none() {
        println!("Please provide a filename");
        return;
    }

    let edges_result = read_from_file::<V>(filename.unwrap().as_str());
    if edges_result.is_err() {
        println!("Error reading edges from file: {:?}", edges_result.err());
        return;
    }

    let edges = edges_result.unwrap_or_default();
    let graph: UnGraphMap<V, ()> = UnGraphMap::from_edges(&edges);


    let now = std::time::Instant::now();


    let components = connected_components(&graph);
    let num_conn_comp = number_connected_components(&graph); 

    println!("{:?}", now.elapsed().as_millis());
    debug_println!("duration: {:?}", now.elapsed());

    assert_eq!(components.len(), num_conn_comp);

    debug_println!("num_conn_comp: {:?}", num_conn_comp);
}
