use sra::sky::SkyDome;

fn main() {
    let dome = SkyDome::new();

    for patch in dome.patches() {
        print!("{},", patch.azimuth_deg());
    }
}
