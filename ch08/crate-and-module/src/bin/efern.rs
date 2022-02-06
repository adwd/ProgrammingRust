use fern_sim::{run_simulation, Fern};

// cargo build --bin efern --verbose
fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };
    run_simulation(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}
