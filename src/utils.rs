use galileo_types::cartesian::{CartesianPoint2d, Point2d, Rect};
use polite::Polite;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::path::Path;
use tracing::info;

/// Generic function to deserialize data types from a CSV file.  Called by methods to avoid code
/// duplication.
pub fn from_csv<T: DeserializeOwned + Clone, P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<T>, std::io::Error> {
    let mut records = Vec::new();
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut dropped = 0;
    for result in rdr.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => {
                info!("Dropping: {:#?}.", e.to_string());
                dropped += 1;
            }
        }
    }
    info!("{} records dropped.", dropped);

    Ok(records)
}

/// Generic function to serialize data types into a CSV file.  Called by methods to avoid code
/// duplication.
pub fn to_csv<T: Serialize + Clone, U: AsRef<std::path::Path>>(
    item: &mut Vec<T>,
    title: U,
) -> Result<(), std::io::Error> {
    let mut wtr = csv::Writer::from_path(title)?;
    for i in item.clone() {
        wtr.serialize(i)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn point_bounds(point: &Point2d, buffer: f64) -> Rect {
    let xmin = point.x() - buffer;
    let xmax = point.x() + buffer;
    let ymin = point.y() - buffer;
    let ymax = point.y() + buffer;
    Rect::new(xmin, ymin, xmax, ymax)
}

pub fn save<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> Polite<()> {
    info!("Serializing to binary.");
    let encode = bincode::serialize(data)?;
    info!("Writing to file.");
    std::fs::write(path, encode)?;
    Ok(())
}
