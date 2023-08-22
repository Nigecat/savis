use savis::{App, Chronicle, Node, State};

fn main() {
    let chronicle = Chronicle {
        history: vec![
            State {
                temperature: 1.0,
                energy: 20.0,
                route: vec![
                    Node {
                        name: "Home".to_string(),
                    },
                    Node {
                        name: "Train Station".to_string(),
                    },
                ],
                used: true,
            },
            State {
                temperature: 0.8,
                energy: 30.0,
                route: vec![
                    Node {
                        name: "Home".to_string(),
                    },
                    Node {
                        name: "Train Station".to_string(),
                    },
                    Node {
                        name: "School".to_string(),
                    },
                ],
                used: false,
            },
        ],
        nodes: vec![
            Node {
                name: "Home".to_string(),
            },
            Node {
                name: "Train Station".to_string(),
            },
            Node {
                name: "School".to_string(),
            },
        ],
        edges: vec![
            (
                Node {
                    name: "Home".to_string(),
                },
                Node {
                    name: "Train Station".to_string(),
                },
            ),
            (
                Node {
                    name: "Train Station".to_string(),
                },
                Node {
                    name: "School".to_string(),
                },
            ),
        ],
        positions: vec![
            (
                Node {
                    name: "Home".to_string(),
                },
                (-0.5, -0.5),
            ),
            (
                Node {
                    name: "Train Station".to_string(),
                },
                (0.4, -0.2),
            ),
            (
                Node {
                    name: "School".to_string(),
                },
                (0.5, 0.5),
            ),
        ]
        .into_iter()
        .collect(),
    };
    nannou::run(App::new(chronicle, 500), |builder| builder);
}
