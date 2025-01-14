pub mod prelude {
    use std::{
        fmt::Debug,
        fs::File,
        io::{self, BufRead, Error},
        str::FromStr,
    };

    #[macro_export]
    macro_rules! debug_println {
        ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
    }

    pub fn read_from_file<V: FromStr>(filename: &str) -> Result<Vec<(V, V)>, Error>
    where V: FromStr<Err: Debug>,
    {
        //let res = Vec::<(u8, u8)>::new();
        let file = File::open(filename);

        match file {
            Ok(f) => Ok(parse_file(f)),
            Err(e) => Err(e),
        }
    }

    fn parse_file<V: FromStr>(file: File) -> Vec<(V, V)>
    where <V as FromStr>::Err: Debug,
    {
        //let res = Vec::<(u8, u8)>::new();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines();
        lines.next(); //skip first line

        //let graph_info = lines.next().unwrap().unwrap();
        let graph_info = match lines.next(){
            Some(v) => v.unwrap(),
            None => {
                println!("Empty file");
                std::process::exit(1);
            }
        };


        let graph_struct: Vec<_> = graph_info.split_whitespace().collect();
        let n_edges = match graph_struct[2].parse::<usize>() {
            Ok(v) => v,
            Err(_) => 0
        };

        let mut edges: Vec<(V, V)> = Vec::with_capacity(n_edges);

        for line in lines {
            let str = line.ok().unwrap();
            let parts: Vec<&str> = str.split_whitespace().collect();

            let edge = (
                parts[0].parse::<V>().unwrap(),
                parts[1].parse::<V>().unwrap(),
            );
            edges.push(edge);
        }

        edges
    }
}