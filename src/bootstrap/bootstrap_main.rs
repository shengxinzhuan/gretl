use std::borrow::BorrowMut;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use clap::{ArgMatches};
use gfa_reader::{Gfa, GraphWrapper, NCGfa, NCPath};
use crate::bootstrap::helper::random_numbers;
use crate::bootstrap::meta::{combinations_maker, combinations_maker_wrapper, one_iteration, reduce_meta};
use crate::bootstrap::reader::read_meta;
use crate::bootstrap::writer::{write_meta, write_output};
use crate::helpers::helper::calculate_similarity;
use crate::stats::helper::get_filename;


/// Main function for bootstrapping
pub fn bootstrap_main(matches: &ArgMatches){

    let mut sep = " ";
    if matches.is_present("Pan-SN"){
        sep = matches.value_of("Pan-SN").unwrap();
    }


    // Read the graph
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), false);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();



    wrapper.from_gfa(&graph.paths, sep);
    let output = matches.value_of("output").unwrap();

    // Get the amount of iterations
    let mut amount = 10;
    if matches.is_present("number"){
        amount = matches.value_of("number").unwrap().parse().unwrap();
    }

    // Limit the amount of iterations (maximum 500)
    amount = min(amount, 500);

    // Combination: {number of genomes, number of iteration, combination (HashSet)}
    let mut combinations: Vec<(usize, usize, HashSet<usize>)> = Vec::new();
    if matches.is_present("meta input"){
        combinations = read_meta(matches.value_of("meta input").unwrap());
    } else {
        combinations = combinations_maker_wrapper(&wrapper.genomes.len(), &amount);
    }

    // Which line should be read
    let mut line = -1;
    if matches.is_present("meta line"){
        line = matches.value_of("meta line").unwrap().parse().unwrap();
    }

    // Which core should be used
    let mut core = -1;
    if matches.is_present("level"){
        core = matches.value_of("level").unwrap().parse().unwrap();
    }

    eprintln!("Running bootstrap");
    let amount_path = wrapper.genomes.len();

    // The which "geomes" have been used in this run
    let mut metas = Vec::new();

    // How much sequence, nodes have been used
    let mut total = Vec::new();

    // Removes lines and unused similarity level from the meta data (file)
    reduce_meta(& mut combinations, line, core);

    // We use the similarity measure
    let similarity = calculate_similarity(&&wrapper, &graph);



    // Iterate over all combinations - calculate the core and the sequence
    for (number_genomes, iterations, combination) in combinations.iter(){
        let combi: Vec<usize> = combination.iter().cloned().collect();
        let result_one_iteration = one_iteration(&wrapper, &graph, &combi, "similarity", &similarity);

        // Add results
        total.push((*number_genomes, *iterations, result_one_iteration));
        metas.push((*number_genomes, *iterations, combination.clone()));
    }

    // Write the meta data if wanted
    if matches.is_present("meta"){
        let metas_output = matches.value_of("meta").unwrap();
        write_meta(metas, metas_output);
    }

    // Write the main output
    write_output(total, output);

}

