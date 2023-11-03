mod stats;
mod bootstrap;
mod core;
mod id2int;
mod path_similarity;
mod node_list;
mod helpers;
mod sliding_window;

use clap::{Arg, App, AppSettings};
use crate::bootstrap::bootstrap_main::bootstrap_main;
use crate::core::core_main::core_main;
use crate::id2int::id2int_main::id2int_main;
use crate::node_list::node_list_main::nodelist_main;
use crate::path_similarity::ps_main::ps_main;
use crate::sliding_window::sliding_window_main::window_main;
use crate::stats::stats_main::stats_main;

fn main() {


    let matches = App::new("gfastats")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Sebastian V")
        .about("GFa stats")
        .setting(AppSettings::SubcommandRequiredElseHelp)

        // Subcommand for normal stats for one graph
        .subcommand(App::new("stats")
            .about("Create statists about the graph or its path")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true))
            .arg(Arg::new("bins")
                .long("bins")
                .about("Size of bins. Example: Format 10,20,30 -> (0-10, 11-20, 30+)[default: 1,50,100,1000]")
                .takes_value(true))
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .about("Report path statistics (default:off -> report graph stats)"))
            .arg(Arg::new("YAML")
                .short('y')
                .about("Report output in YAML format (default:off -> report in tsv)")))

        .subcommand(App::new("bootstrap")
            .about("Bootstrap approach")
            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Seperate by first entry in Pan-SN spec")
                .takes_value(true))

            .arg(Arg::new("meta input")
                .long("meta-input")
                .about("Take a specific meta file as input")
                .takes_value(true))
            .arg(Arg::new("Threads")
                .long("threads")
                .short('t')
                .takes_value(true)
                .about("Number of threads")
                .default_value("1"))

            .help_heading("Modifications")
            .arg(Arg::new("meta line")
                .long("meta-line")
                .about("Take a specific line of the meta file (only works when meta file is provided)")
                .takes_value(true))
            .arg(Arg::new("level")
                .long("level")
                .about("Only calculate a specific level")
                .takes_value(true))
            .arg(Arg::new("number")
                .long("number")
                .about("How many bootstraps do you want to run")
                .takes_value(true))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true))
            .arg(Arg::new("meta")
                .long("meta")
                .about("Report an additional meta file with all combinations")
                .takes_value(true)))






        // Subcommand for similarity level
        .subcommand(App::new("core")
            .about("Graph similarity statistics")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("statistics")
                .short('s')
                .long("stats")
                .about("similarity, depth, nd [default: similarity]")
                .default_value("similarity")
                .takes_value(true))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true)))


        // Subcommand for detailed similarity
        .subcommand(App::new("ps")
            .about("Detailed similarity information for each path")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Seperate by first entry in Pan-SN spec")
                .takes_value(true)))

        // Subcommand to convert string graph to numeric
        .subcommand(App::new("id2int")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .short('s')
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true))
            .about("Convert node identifier to numeric values (not sorted)")
            .arg(Arg::new("dict")
                .long("dict")
                .about("Write a dictionary for Old->New identifiers in this file.")
                .takes_value(true)
                .short('d')))

        .subcommand(App::new("window")
            .about("Sliding window along the samples")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Seperate by first entry in Pan-SN spec")
                .takes_value(true))
            .arg(Arg::new("size")
                .short('s')
                .long("window")
                .about("Window on sequence")
                .takes_value(true))
            .arg(Arg::new("step")
                .long("step")
                .about("Step size (If nothing is set, step size -> bin size")
                .takes_value(true))
            .arg(Arg::new("node")
                .short('n')
                .long("node")
                .about("Window on nodes nodes ([default: off] -> on sequence)"))
            .arg(Arg::new("metric")
                .short('m')
                .long("metric")
                .about("Which metric")
                .takes_value(true)))



        .subcommand(App::new("node-list")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .short('s')
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true))
            .about("Some information about each node")
            .arg(Arg::new("Features")
                .short('f')
                .long("feature")
                .takes_value(true)
                .about("Name the features you need. If nothing is used, report everything. Example -f Length, Core")))
        .get_matches();

    // Read the graph

    if let Some(ref matches) = matches.subcommand_matches("core"){
        core_main(matches);

    } else if let Some(ref matches) = matches.subcommand_matches("bootstrap") {
        bootstrap_main(&matches);
    } else if let Some(ref matches) = matches.subcommand_matches("stats"){
        stats_main(&matches);
    } else if let Some(ref matches) = matches.subcommand_matches("id2int"){
        id2int_main(&matches);
    } else if let Some(ref matches) = matches.subcommand_matches("ps"){
        ps_main(&matches);
    } else if let Some(ref matches) = matches.subcommand_matches("node-list"){
        nodelist_main(&matches);
    }  else if let Some(ref matches) = matches.subcommand_matches("window"){
        window_main(&matches);
    }

}
