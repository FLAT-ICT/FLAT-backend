use std::{error::Error, fs::File};

use crate::{model::db_util, repository::DeserializableSpot};

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let file_path = "espresso-beacons/src/spots.csv";
    let file = File::open(file_path)?;
    println!("{:?}", file);
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: DeserializableSpot = result?;
        db_util::insert_spot(record.to_insertable());
    }
    Ok(())
}
