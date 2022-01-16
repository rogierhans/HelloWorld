#![allow(non_snake_case)]
use std::time::{ Instant};
mod f;
mod qi;
mod rrf;
mod suc;
fn main() {
    let filename = "C:\\Users\\Rogier\\OneDrive - Universiteit Utrecht\\1UCTest\\GA10\\0.suc";
    let uc = suc::create_suc(filename);
    let mut list:Vec<f64> = Vec::new();
    let start = Instant::now();
    for _ in 0..10000 {
        let mut rrf = rrf::RRF {
            UC: uc.clone(),
            Fs: Vec::with_capacity(uc.TotalTime),
            stop: Vec::with_capacity(uc.TotalTime),
        };
        let score = rrf.GetScore();
        list.push(score);

    }
    let minValue = list.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("{}",minValue);
    
    //quadartic.print();
    // suc.print();
}
