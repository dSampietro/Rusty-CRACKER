use std::{fmt::Debug, fs::File, io::{self, BufRead, Error}, str::FromStr};

pub fn read_from_file<V: FromStr>(filename: &str) -> Result<Vec<(V, V)>, Error>
    where <V as FromStr>::Err: Debug
{
    //let res = Vec::<(u8, u8)>::new();
    let file = File::open(filename);

    match file {
        Ok(f) => Ok(parse_file(f)),
        Err(e) => Err(e)
    }
}

fn parse_file<V: FromStr>(file: File) -> Vec<(V, V)>
    where <V as FromStr>::Err: Debug
{
    //let res = Vec::<(u8, u8)>::new();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    lines.next();   //skip first line
    
    let graph_info = lines.next().unwrap().unwrap();

    let graph_struct: Vec<_> = graph_info.split_whitespace().collect();
    let n_edges = graph_struct[2].parse::<usize>().unwrap();
    println!("{:?}", n_edges);

    let mut edges: Vec<(V, V)> = Vec::with_capacity(n_edges);


    for line in lines{
        let str = line.ok().unwrap();
        let parts: Vec<&str> = str.split_whitespace().collect();

        let edge = (parts[0].parse::<V>().unwrap(), parts[1].parse::<V>().unwrap());
        edges.push(edge);
    }

    return edges;
}
