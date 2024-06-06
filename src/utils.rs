use galileo_types::cartesian::{CartesianPoint2d, Point2d, Rect};
use indicatif::{ProgressBar, ProgressStyle};
use polite::Polite;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::{fs, io, path, time};
use tracing::info;

/// Generic function to deserialize data types from a CSV file.  Called by methods to avoid code
/// duplication.
pub fn from_csv<T: DeserializeOwned + Clone, P: AsRef<path::Path>>(
    path: P,
) -> Result<Vec<T>, io::Error> {
    let mut records = Vec::new();
    let file = fs::File::open(path)?;
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
pub fn to_csv<T: Serialize + Clone, U: AsRef<path::Path>>(
    item: &mut Vec<T>,
    title: U,
) -> Result<(), io::Error> {
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

pub fn save<T: Serialize, P: AsRef<path::Path>>(data: &T, path: P) -> Polite<()> {
    info!("Serializing to binary.");
    let encode = bincode::serialize(data)?;
    info!("Writing to file.");
    fs::write(path, encode)?;
    Ok(())
}

/// The `load_bin` function loads the contents of a file at location `path` into a `Vec<u8>`.
/// May error reading the file, for example if the location is invalid, or when deserializing
/// the binary if the format is invalid.
pub fn load_bin<P: AsRef<path::Path>>(path: P) -> Polite<Vec<u8>> {
    info!("Loading from binary.");
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(time::Duration::from_millis(120));
    bar.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    bar.set_message("Loading...");
    let vec = fs::read(path)?;
    bar.finish_with_message("Loaded!");
    Ok(vec)
}
