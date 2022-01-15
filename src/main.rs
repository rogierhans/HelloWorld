#![allow(non_snake_case)]
mod f;
mod qi;
mod suc;
mod rrf;
fn main() {
    let filename = "C:\\Users\\Rogier\\OneDrive - Universiteit Utrecht\\1UCTest\\GA10\\0.suc";
    let uc = suc::create_suc(filename);
    let mut rrf = rrf::RRF{
        UC : uc.clone(),
        Fs : Vec::new(),
        stop : Vec::new(),
    };
    let score = rrf.GetScore();
    println!("{}",uc.Objective);
    println!("{}",score);
    //quadartic.print();
    // suc.print();
}
