use clap::Parser;
use savis::{App, Chronicle};
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Parser)]
struct Cli {
    /// The path to the JSON data file
    data: PathBuf,
    /// The time to wait between updates, in milliseconds
    #[clap(long, short, default_value = "500")]
    delay: usize,
}

fn main() {
    let args = Cli::parse();
    let file = fs::File::open(args.data).unwrap();
    let data: Chronicle = serde_json::from_reader(io::BufReader::new(file)).unwrap();
    nannou::run(App::new(data, args.delay), |builder| builder);

    // let chronicle = Chronicle {
    //     history: vec![
    //         State {
    //             temperature: 1.0,
    //             energy: 20.0,
    //             route: vec![
    //                 Node {
    //                     name: "Home".to_string(),
    //                 },
    //                 Node {
    //                     name: "Train Station".to_string(),
    //                 },
    //             ],
    //             used: true,
    //         },
    //         State {
    //             temperature: 0.8,
    //             energy: 30.0,
    //             route: vec![
    //                 Node {
    //                     name: "Home".to_string(),
    //                 },
    //                 Node {
    //                     name: "Train Station".to_string(),
    //                 },
    //                 Node {
    //                     name: "School".to_string(),
    //                 },
    //             ],
    //             used: false,
    //         },
    //     ],
    //     nodes: vec![
    //         Node {
    //             name: "Home".to_string(),
    //         },
    //         Node {
    //             name: "Train Station".to_string(),
    //         },
    //         Node {
    //             name: "School".to_string(),
    //         },
    //     ],
    //     edges: vec![
    //         (
    //             Node {
    //                 name: "Home".to_string(),
    //             },
    //             Node {
    //                 name: "Train Station".to_string(),
    //             },
    //         ),
    //         (
    //             Node {
    //                 name: "Train Station".to_string(),
    //             },
    //             Node {
    //                 name: "School".to_string(),
    //             },
    //         ),
    //     ],
    //     positions: vec![
    //         (
    //             Node {
    //                 name: "Home".to_string(),
    //             },
    //             (-0.5, -0.5),
    //         ),
    //         (
    //             Node {
    //                 name: "Train Station".to_string(),
    //             },
    //             (0.4, -0.2),
    //         ),
    //         (
    //             Node {
    //                 name: "School".to_string(),
    //             },
    //             (0.5, 0.5),
    //         ),
    //     ]
    //     .into_iter()
    //     .collect(),
    // };
    // nannou::run(App::new(chronicle, 500), |builder| builder);
}
