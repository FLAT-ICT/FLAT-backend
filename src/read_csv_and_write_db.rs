use crate::{model::db_util::insert_spots_from_csv, repository::DeserializableSpot};
use std::{error::Error, fs::File};

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let file_path = "espresso-beacons/src/spots.csv";
    let file = File::open(file_path)?;
    println!("{:?}", file);
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: DeserializableSpot = result?;
        if let Err(e) = insert_spots_from_csv(record.to_insertable()) {
            println!("{}", e)
        }
    }
    Ok(())
}

#[cfg(test)]
mod beacon_tests {

    use crate::model::db_util::establish_connection;
    use crate::schema::spots::dsl::*;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    #[tokio::test]
    async fn test_get_spot() {
        let minor_id = 43303;
        let major_id = 0;
        let conn = establish_connection();
        let result = spots
            .filter(major.eq(&major_id))
            .filter(minor.eq(&minor_id))
            .select(name_ja)
            .first::<String>(&conn)
            .unwrap();
        assert_eq!(result, "127教員室".to_string());
    }
}
