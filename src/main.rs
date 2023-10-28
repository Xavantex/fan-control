
use config::*;

fn main() {


    let conf1 = config1();


}







fn config1() -> Config {
    Config {
        custom_temps: vec![CustomTemp {
            name: "max".into(),
            kind: CustomTempType::Max,
            input: vec!["temp1".into(), "temp2".into()],
        }],
        graphs: vec![Graph {
            name: "graph1".into(),
            coords: vec![
                Coord {
                    temp: 10,
                    percent: 10,
                },
                Coord {
                    temp: 50,
                    percent: 30,
                },
                Coord {
                    temp: 90,
                    percent: 100,
                },
            ],
            input: "max".into(),
            output: vec!["control1".into()],
        }],
        flats: vec![
            Flat {
                name: "flat1".into(),
                value: 50,
                output: vec!["control2".into()],
            },
            Flat {
                name: "flat2".into(),
                value: 100,
                output: vec![],
            },
        ],
        linears: vec![Linear {
            name: "graph1".into(),
            min_temp: 10,
            min_speed: 10,
            max_temp: 70,
            max_speed: 100,
            input: "temp3".into(),
            output: vec!["control3".into()],
        }],
        targets: vec![Target {
            name: "graph1".into(),

            idle_temp: 40,
            idle_speed: 10,
            load_temp: 70,
            load_speed: 100,
            input: "temp3".into(),
            output: vec!["control4".into()],
        }],
    }
}
