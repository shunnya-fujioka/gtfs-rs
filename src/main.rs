mod gtfs;

use anyhow::Result;
use dotenvy::dotenv;
use std::{env, fs::File, path::Path};

use gtfs::agency::Agency;

fn main() -> Result<()> {
    dotenv().expect(".envファイルが見つかりません。");

    let gtfs_dir_path = env::var("GTFS_DIR_PATH").expect("GTFS_DIR_PATHを設定してください");
    let agency_path = gtfs_dir_path + "/agency.txt";
    let file_path = Path::new(&agency_path);
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let record: Agency = result?;
        println!("{:?}", record);
    }

    Ok(())
}
