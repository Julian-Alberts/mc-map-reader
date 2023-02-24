use mc_map_reader::LoadMcSave;

fn main() {
    let map = mc_map_reader::Loader
        .load("../server_world/region/r.-1.-1.mca")
        .unwrap();
    println!("{map:#?}")
}
