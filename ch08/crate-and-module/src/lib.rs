pub mod plant_structures;

// src/lib.rs がライブラリのルートモジュールを構成する
// このライブラリを用いる他のクレートはこのルートモジュールのパブリックアイテムしかアクセスできない
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// link to [`spores`](crate::fern_sim:plant_structures::spores)
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
