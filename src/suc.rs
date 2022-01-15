use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
pub fn create_suc(filename: impl AsRef<Path>) -> SUC {
    let lines = lines_from_file(filename);
    SUC {
        Objective: lines[0].parse().unwrap(),
        TotalTime: lines[1].parse().unwrap(),
        pMax: lines[2].parse().unwrap(),
        pMin: lines[3].parse().unwrap(),
        RampUp: lines[4].parse().unwrap(),
        RampDown: lines[5].parse().unwrap(),
        SU: lines[6].parse().unwrap(),
        SD: lines[7].parse().unwrap(),
        MinDownTime: lines[8].parse().unwrap(),
        MinUpTime: lines[9].parse().unwrap(),
        StartCost: lines[10].parse().unwrap(),
        A: lines[11].parse().unwrap(),
        B: lines[12].parse().unwrap(),
        C: lines[13].parse().unwrap(),
        LagrangeMultipliers: lines[14].split('\t').map(|c| c.parse().unwrap()).collect(),
        BM: lines[15].split('\t').map(|c| c.parse().unwrap()).collect(),
        CM: lines[16].split('\t').map(|c| c.parse().unwrap()).collect(),
    }
}

impl SUC {
    // pub fn print(&self) {
    //     println!(
    //         "[{},{}] ({},{},{})",
    //         self.pMin, self.pMax, self.A, self.B, self.C
    //     );
    //     let mut line = String::new();
    //     for multipllier in self.BM.iter() {
    //         line.push_str(&multipllier.to_string())
    //     }
    //     println!("{}",line);
    // }
}
#[derive(Clone)]
pub struct SUC {
    pub Objective: f64,
    pub TotalTime: usize,
    pub pMax: f64,
    pub pMin: f64,
    pub RampUp: f64,
    pub RampDown: f64,
    pub SU: f64,
    pub SD: f64,
    pub MinDownTime: usize,
    pub MinUpTime: usize,
    pub StartCost: f64,
    pub A: f64,
    pub B: f64,
    pub C: f64,
    pub LagrangeMultipliers: Vec<f64>,
    pub BM: Vec<f64>,
    pub CM: Vec<f64>,
}
