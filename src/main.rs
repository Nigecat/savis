use savis::{App, Chronicle, Node, State};

fn main() {
    let chronicle = Chronicle {
        history: vec![State {
            temperature: 1.0,
            energy: 20.0,
            route: vec![
                Node {
                    name: "Home".to_string(),
                },
                Node {
                    name: "School".to_string(),
                },
            ],
            used: false,
        }],
        nodes: vec![
            Node {
                name: "Home".to_string(),
            },
            Node {
                name: "School".to_string(),
            },
        ],
        edges: vec![(
            Node {
                name: "Home".to_string(),
            },
            Node {
                name: "School".to_string(),
            },
        )],
    };
    nannou::run(App::new(chronicle, 500), |builder| builder);
}
