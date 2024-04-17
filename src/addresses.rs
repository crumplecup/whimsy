use crate::prelude::*;
use egui::{Align, Layout, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use galileo::layer::feature_layer::Feature;
use galileo_types::cartesian::{CartesianPoint2d, Point2d, Rect};
use galileo_types::geo::Projection;
use galileo_types::geometry::{CartesianGeometry2d, Geom, Geometry};
use polite::Polite;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tracing::info;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Address {
    // #[serde(rename(deserialize = "FULLADDRES"))]
    pub label: String,
    // #[serde(rename(deserialize = "Add_Number"))]
    pub number: i64,
    // #[serde(deserialize_with = "deserialize_mixed_pre_directional",
    //         rename(deserialize = "St_PreDir"))]
    pub directional: Option<StreetNamePreDirectional>,
    // #[serde(rename(deserialize = "St_Name"))]
    pub street_name: String,
    // #[serde(deserialize_with = "deserialize_mixed_post_type",
    //         rename(deserialize = "St_PosTyp"))]
    pub street_type: Option<StreetNamePostType>,
    // #[serde(rename(deserialize = "Subaddre_1"))]
    pub subaddress_id: Option<String>,
    // #[serde(deserialize_with = "deserialize_mixed_subaddress_type",
    //         rename(deserialize = "Subaddress"))]
    pub subaddress_type: Option<SubaddressType>,
    // #[serde(rename(deserialize = "Post_Code"))]
    pub zip: i64,
    // #[serde(rename(deserialize = "STATUS"))]
    pub status: AddressStatus,
    // #[serde(rename(deserialize = "wgs84_y"))]
    pub lat: f64,
    // #[serde(rename(deserialize = "wgs84_x"))]
    pub lon: f64,
    // #[serde(rename(deserialize = "espg3857_x"))]
    pub x: f64,
    // #[serde(rename(deserialize = "espg3857_y"))]
    pub y: f64,
}

// impl Default for Address {
//     fn default() -> Self {
//
//     }
// }

impl Address {
    pub fn column<T: fmt::Display>(&self, columns: &AddressColumns) -> String {
        match *columns {
            AddressColumns::Label => format!("{}", self.label),
            AddressColumns::Number => format!("{}", self.number),
            AddressColumns::Directional => {
                if let Some(prefix) = self.directional {
                    format!("{}", prefix)
                } else {
                    "".to_string()
                }
            },
            AddressColumns::StreetName => format!("{}", self.street_name),
            AddressColumns::StreetType => {
                if let Some(value) = &self.street_type {
                    format!("{}", value.abbreviate())
                } else {
                    "".to_string()
                }
            },
            AddressColumns::SubaddressType => {
                if let Some(subtype) = &self.subaddress_type {
                    format!("{}", subtype)
                } else {
                    "".to_string()
                }
            },
            AddressColumns::SubaddressId => {
                if let Some(value) = &self.subaddress_id {
                    format!("{}", value)
                } else {
                    "".to_string()
                }
            },
            AddressColumns::Zip => format!("{}", self.zip),
            AddressColumns::Status => format!("{}", self.status),
        }
    }

    pub fn columns(&self) -> Vec<String> {
        let mut values = Vec::new();
        for column in AddressColumns::iter() {
            values.push(self.column::<String>(&column));
        }
        values
    }
}

impl Tabular<Address> for Addresses {
    fn headers() -> Vec<String> {
        AddressColumns::names()
    }

    fn rows(&self) -> Vec<Address> {
        self.records.clone()
    }
}

impl Columnar for Address {
    fn names() -> Vec<String> {
        AddressColumns::names()
    }


    fn values(&self) -> Vec<String> {
        self.columns()
    }
}

impl Card for Address {
    fn contains(&self, fragment: &str, config: SearchConfig) -> bool {
        let mut label = self.label.to_owned();
        let mut status = self.status.to_string();
        let mut test = fragment.to_string();
        if !config.case_sensitive {
            label = label.to_lowercase();
            status = status.to_lowercase();
            test = test.to_lowercase();
        }
        if label.contains(&test) | status.contains(&test) {
            true
        } else {
            false
        }
    }

    fn show(&self, ui: &mut Ui) {
        ui.label(format!("Address: {}", self.label));
        ui.label(format!("Status: {}", self.status));
    }

}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address: {} \n Status: {}", self.label, self.status)
    }
}

pub trait RowView<T: fmt::Display> {
    fn row(&self) -> T;
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum AddressColumns {
    Label,
    Number,
    Directional,
    StreetName,
    StreetType,
    SubaddressType,
    SubaddressId,
    Zip,
    Status
}

impl AddressColumns {
    pub fn names() -> Vec<String> {
        let mut values = Vec::new();
        for column in AddressColumns::iter() {
            values.push(format!("{column}"));
        }
        values
    }

}

impl fmt::Display for AddressColumns {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Label => write!(f, "Label"),
            Self::Number => write!(f, "Number"),
            Self::Directional => write!(f, "Directional Prefix"),
            Self::StreetName => write!(f, "Street Name"),
            Self::StreetType => write!(f, "Street Type"),
            Self::SubaddressType => write!(f, "Subaddress Type"),
            Self::SubaddressId => write!(f, "Subaddress ID"),
            Self::Zip => write!(f, "Zip"),
            Self::Status => write!(f, "Status"),
        }
    }
}






#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Addresses {
    pub records: Vec<Address>,
}

impl Addresses {
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Polite<Self> {
        let records = from_csv(path)?;
        Ok(Addresses { records })
    }

    pub fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Polite<()> {
        to_csv(&mut self.records, path)?;
        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path:P) -> Polite<()> {
        save(self, path)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
        info!("Deserializing from binary.");
        let vec: Vec<u8> = std::fs::read(path)?;
        let values: Addresses = bincode::deserialize(&vec[..])?;
        Ok(values)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressPoint {
    pub address: Address,
    pub geometry: Point2d,
    pub bounds: Rect,
    pub selected: bool,
}

impl From<Address> for AddressPoint {
    fn from(address: Address) -> Self {
        let geometry = Point2d::new(address.x, address.y);
        let bounds = point_bounds(&geometry, 0.05);
        let selected = false;
        Self {
            address,
            geometry,
            bounds,
            selected,
        }
    }
}

impl Geometry for AddressPoint {
    type Point = Point2d;

    fn project<P: Projection<InPoint = Self::Point> + ?Sized>(
        &self,
        projection: &P,
    ) -> Option<Geom<P::OutPoint>> {
        self.geometry.project(projection)
    }
}

impl CartesianGeometry2d<Point2d> for AddressPoint {
    fn is_point_inside<Other: CartesianPoint2d<Num = f64>>(
        &self,
        point: &Other,
        tolerance: f64,
    ) -> bool {
        if !self.bounds.contains(point) {
            return false;
        }

        self.geometry.is_point_inside(point, tolerance)
    }

    fn bounding_rectangle(&self) -> Option<Rect> {
        Some(self.bounds)
    }
}

impl Feature for AddressPoint {
    type Geom = Self;

    fn geometry(&self) -> &Self::Geom {
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressPoints {
    pub records: Vec<AddressPoint>,
}

impl From<Addresses> for AddressPoints {
    fn from(addresses: Addresses) -> Self {
        let records = addresses
            .records
            .iter()
            .map(|v| AddressPoint::from(v.clone()))
            .collect::<Vec<AddressPoint>>();
        Self { records }
    }
}

impl AddressPoints {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Polite<()> {
        save(self, path)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
        info!("Deserializing from binary.");
        let vec: Vec<u8> = std::fs::read(path)?;
        let addresses: AddressPoints = bincode::deserialize(&vec[..])?;
        Ok(addresses)
    }
}
