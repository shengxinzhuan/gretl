use clap::ArgMatches;
use gfa_reader::Gfa;
use crate::node_list::wrapper::wrapper;

pub fn nodelist_main(matches: &ArgMatches, graph: &Gfa, output: &str) {
    eprintln!("Running node-list analysis.");
    // Size, depth, similarity, degree in, degree out, degree_total, inversion amount,
    // path related?
    let splits = vec!["Core", "Length", "Depth", "Core", "ND_in", "ND_out", "ND_total"];
    let mut splits2 = Vec::new();
    if matches.is_present("Features"){
        splits2 = matches.value_of("Features").unwrap().split(",").collect();
    }
    let mut final1 = Vec::new();
    for x in splits2.iter(){
        if splits.contains(x) {
            final1.push(*x);
        }
    }

    if final1.len() == 0{
        final1 = splits.clone();
    }
    let data = wrapper(&graph, output, final1);


}