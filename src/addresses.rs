use crate::prelude::{save, Columnar, Filtration, Tabular};
use address::prelude::{
    Address, AddressStatus, MatchRecord, MatchRecords, MatchStatus, SpatialAddress,
    SpatialAddresses,
};
use galileo::galileo_types::cartesian::{CartesianPoint2d, CartesianPoint3d, Point2d};
use galileo::galileo_types::geo::impls::GeoPoint2d;
use galileo::galileo_types::geo::{GeoPoint, NewGeoPoint}; //, Projection};
use galileo::galileo_types::geometry::Geom;
use galileo::galileo_types::geometry_type::{
    AmbiguousSpace, GeoSpace2d, GeometryType, PointGeometryType,
};
use galileo::galileo_types::impls::{Contour, Polygon};
use galileo::layer::feature_layer::symbol::Symbol;
use galileo::layer::feature_layer::Feature;
use galileo::render::point_paint::PointPaint;
use galileo::render::render_bundle::RenderPrimitive;
use galileo::Color;
use num_traits::AsPrimitive;
use polite::Polite;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

#[derive(
    Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumIter, Serialize, Deserialize,
)]
pub enum AddressColumns {
    #[default]
    Label,
    Number,
    Directional,
    StreetName,
    StreetType,
    SubaddressType,
    SubaddressId,
    Zip,
    Status,
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

// We convert the "column index" from the table view to this enum to take advantage of pattern
// matching over an index.  Commits a *faux pas* if the number does not match an index in the address columns.
impl TryFrom<usize> for AddressColumns {
    type Error = polite::FauxPas;
    fn try_from(index: usize) -> Result<Self, Self::Error> {
        // iterate through address columns
        let columns = Self::iter()
            // index the iterator
            .enumerate()
            // match the column indices
            .filter(|(i, _)| *i == index)
            // grab the corresponding address column enum
            .map(|(_, v)| v)
            // will only return at most one success
            .take(1)
            // but we have to collect it into a vector, cuz ?
            .collect::<Vec<AddressColumns>>();
        // Given index does not map to a valid column.
        if columns.is_empty() {
            // Return a *faux pas*.
            Err(polite::FauxPas::Unknown)
        // Valid column found.
        } else {
            // Return success value.
            Ok(columns[0].clone())
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AddressPoint {
    pub address: SpatialAddress,
    pub id: Uuid,
    pub point: Point2d,
    pub geo_point: GeoPoint2d,
}

impl AddressPoint {
    pub fn geo_point(&self) -> geo::geometry::Point {
        let x = CartesianPoint2d::x(self);
        let y = CartesianPoint2d::y(self);
        geo::geometry::Point::new(x, y)
    }

    pub fn column<T: fmt::Display>(&self, columns: &AddressColumns) -> String {
        match *columns {
            AddressColumns::Label => self.address.label(),
            AddressColumns::Number => format!("{}", self.address.number()),
            AddressColumns::Directional => {
                if let Some(prefix) = self.address.directional() {
                    format!("{}", prefix)
                } else {
                    "".to_string()
                }
            }
            AddressColumns::StreetName => self.address.street_name().to_string(),
            AddressColumns::StreetType => {
                if let Some(value) = &self.address.street_type() {
                    value.abbreviate()
                } else {
                    "".to_string()
                }
            }
            AddressColumns::SubaddressType => {
                if let Some(subtype) = &self.address.subaddress_type() {
                    format!("{}", subtype)
                } else {
                    "".to_string()
                }
            }
            AddressColumns::SubaddressId => {
                if let Some(value) = &self.address.subaddress_id() {
                    value.to_string()
                } else {
                    "".to_string()
                }
            }
            AddressColumns::Zip => format!("{}", self.address.zip()),
            AddressColumns::Status => format!("{}", self.address.status()),
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

impl Columnar for AddressPoint {
    fn names() -> Vec<String> {
        AddressColumns::names()
    }

    fn values(&self) -> Vec<String> {
        self.columns()
    }

    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl From<&SpatialAddress> for AddressPoint {
    fn from(address: &SpatialAddress) -> Self {
        let point = Point2d::new(CartesianPoint2d::x(address), CartesianPoint2d::y(address));
        let geo_point = GeoPoint2d::latlon(
            galileo_types::geo::GeoPoint::lat(address),
            galileo_types::geo::GeoPoint::lon(address),
        );
        let id = Uuid::new_v4();
        let address = address.clone();
        Self {
            address,
            id,
            point,
            geo_point,
        }
    }
}

impl GeoPoint for AddressPoint {
    type Num = f64;

    fn lat(&self) -> Self::Num {
        self.address.latitude
    }

    fn lon(&self) -> Self::Num {
        self.address.longitude
    }
}

impl CartesianPoint2d for AddressPoint {
    type Num = f64;

    fn x(&self) -> Self::Num {
        CartesianPoint2d::x(&self.address)
    }

    fn y(&self) -> Self::Num {
        CartesianPoint2d::y(&self.address)
    }
}

impl CartesianPoint3d for AddressPoint {
    type Num = f64;

    fn x(&self) -> Self::Num {
        CartesianPoint2d::x(self)
    }

    fn y(&self) -> Self::Num {
        CartesianPoint2d::y(self)
    }

    fn z(&self) -> Self::Num {
        match self.address.floor() {
            Some(x) => *x as f64 * 5.0,
            None => 5.0,
        }
    }
}

impl GeometryType for AddressPoint {
    type Type = PointGeometryType;
    type Space = AmbiguousSpace;
}

impl Feature for AddressPoint {
    type Geom = GeoPoint2d;

    fn geometry(&self) -> &Self::Geom {
        &self.geo_point
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AddressPoints {
    pub records: Vec<AddressPoint>,
}

impl AddressPoints {
    pub fn sort_by_col(&mut self, column_index: usize, reverse: bool) {
        // Parse the index to an address column.
        if let Ok(column) = AddressColumns::try_from(column_index) {
            // Match against the column type and sort.
            match column {
                AddressColumns::Label => {
                    if reverse {
                        self.records
                            .sort_by(|a, b| b.address.label().cmp(&a.address.label()));
                    } else {
                        self.records
                            .sort_by(|a, b| a.address.label().cmp(&b.address.label()));
                    }
                }
                AddressColumns::Number => {
                    if reverse {
                        self.records
                            .sort_by(|a, b| b.address.number().cmp(&a.address.number()));
                    } else {
                        self.records
                            .sort_by(|a, b| a.address.number().cmp(&b.address.number()));
                    }
                }
                AddressColumns::Directional => {
                    if reverse {
                        self.records
                            .sort_by(|a, b| b.address.directional().cmp(&a.address.directional()));
                    } else {
                        self.records
                            .sort_by(|a, b| a.address.directional().cmp(&b.address.directional()));
                    }
                }
                AddressColumns::StreetName => {
                    if reverse {
                        self.records
                            .sort_by(|a, b| b.address.street_name().cmp(&a.address.street_name()));
                    } else {
                        self.records
                            .sort_by(|a, b| a.address.street_name().cmp(&b.address.street_name()));
                    }
                }
                AddressColumns::StreetType => {
                    if reverse {
                        self.records
                            .sort_by(|a, b| b.address.street_type().cmp(&a.address.street_type()));
                    } else {
                        self.records
                            .sort_by(|a, b| a.address.street_type().cmp(&b.address.street_type()));
                    }
                }
                AddressColumns::SubaddressType => {
                    if reverse {
                        self.records.sort_by(|a, b| {
                            b.address
                                .subaddress_type()
                                .cmp(&a.address.subaddress_type())
                        });
                    } else {
                        self.records.sort_by(|a, b| {
                            a.address
                                .subaddress_type()
                                .cmp(&b.address.subaddress_type())
                        });
                    }
                }
                AddressColumns::SubaddressId => {
                    if reverse {
                        self.records.sort_by(|a, b| {
                            b.address.subaddress_id().cmp(&a.address.subaddress_id())
                        });
                    } else {
                        self.records.sort_by(|a, b| {
                            a.address.subaddress_id().cmp(&b.address.subaddress_id())
                        });
                    }
                }
                AddressColumns::Zip => {
                    if reverse {
                        self.records
                            .sort_by(|a, b| b.address.zip().cmp(&a.address.zip()));
                    } else {
                        self.records
                            .sort_by(|a, b| a.address.zip().cmp(&b.address.zip()));
                    }
                }
                AddressColumns::Status => {
                    if reverse {
                        self.records
                            .sort_by(|a, b| b.address.status().cmp(&a.address.status()));
                    } else {
                        self.records
                            .sort_by(|a, b| a.address.status().cmp(&b.address.status()));
                    }
                }
            }
        }
    }
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Polite<()> {
        tracing::info!("Serializing to binary.");
        save(self, path)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
        tracing::info!("Deserializing from binary.");
        let vec: Vec<u8> = std::fs::read(path)?;
        let addresses: AddressPoints = bincode::deserialize(&vec[..])?;
        Ok(addresses)
    }
}

impl Tabular<AddressPoint> for AddressPoints {
    fn headers() -> Vec<String> {
        AddressColumns::names()
    }

    fn rows(&self) -> Vec<AddressPoint> {
        self.records.clone()
    }

    fn sort_by_col(&mut self, column_index: usize, reverse: bool) {
        self.sort_by_col(column_index, reverse);
    }
}

impl Filtration<AddressPoints, String> for AddressPoints {
    fn filter(self, filter: &String) -> Self {
        self
    }
}

impl From<&SpatialAddresses> for AddressPoints {
    fn from(addresses: &SpatialAddresses) -> Self {
        let records = addresses
            .records
            .iter()
            .map(AddressPoint::from)
            .collect::<Vec<AddressPoint>>();
        Self { records }
    }
}

pub struct AddressSymbol {}

impl Symbol<AddressPoint> for AddressSymbol {
    fn render<'a, N, P>(
        &self,
        feature: &AddressPoint,
        geometry: &'a Geom<P>,
        _min_resolution: f64,
    ) -> Vec<RenderPrimitive<'a, N, P, Contour<P>, Polygon<P>>>
    where
        N: AsPrimitive<f32>,
        P: CartesianPoint3d<Num = N> + Clone,
    {
        let size = 7.0 as f32;
        let mut primitives = Vec::new();
        let Geom::Point(point) = geometry else {
            return primitives;
        };
        let color = match &feature.address.status() {
            AddressStatus::Current => Color::BLUE,
            AddressStatus::Other => Color::from_hex("#dbc200"),
            AddressStatus::Pending => Color::from_hex("#db00d4"),
            AddressStatus::Temporary => Color::from_hex("#db6e00"),
            AddressStatus::Retired => Color::from_hex("#ad0000"),
            AddressStatus::Virtual => Color::from_hex("#32a852"),
        };
        primitives.push(RenderPrimitive::new_point_ref(
            point,
            PointPaint::circle(color, size),
        ));
        primitives
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPoint {
    record: MatchRecord,
    geo_point: GeoPoint2d,
}

impl GeoPoint for MatchPoint {
    type Num = f64;

    fn lat(&self) -> Self::Num {
        galileo_types::geo::GeoPoint::lat(&self.record)
    }

    fn lon(&self) -> Self::Num {
        galileo_types::geo::GeoPoint::lon(&self.record)
    }
}

impl GeometryType for MatchPoint {
    type Type = PointGeometryType;
    type Space = GeoSpace2d;
}

impl Feature for MatchPoint {
    type Geom = GeoPoint2d;

    fn geometry(&self) -> &Self::Geom {
        &self.geo_point
    }
}

impl From<&MatchRecord> for MatchPoint {
    fn from(record: &MatchRecord) -> Self {
        let geo_point = GeoPoint2d::latlon(
            galileo_types::geo::GeoPoint::lat(record),
            galileo_types::geo::GeoPoint::lon(record),
        );
        let record = record.clone();
        Self { record, geo_point }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MatchPoints {
    pub records: Vec<MatchPoint>,
}

impl From<&MatchRecords> for MatchPoints {
    fn from(records: &MatchRecords) -> Self {
        let records = records
            .records
            .iter()
            .map(|r| r.into())
            .collect::<Vec<MatchPoint>>();
        Self { records }
    }
}

pub struct MatchSymbol {}

impl Symbol<MatchPoint> for MatchSymbol {
    fn render<'a, N, P>(
        &self,
        feature: &MatchPoint,
        geometry: &'a Geom<P>,
        _min_resolution: f64,
    ) -> Vec<RenderPrimitive<'a, N, P, Contour<P>, Polygon<P>>>
    where
        N: AsPrimitive<f32>,
        P: CartesianPoint3d<Num = N> + Clone,
    {
        let size = 7.0 as f32;
        let mut primitives = Vec::new();
        let Geom::Point(point) = geometry else {
            return primitives;
        };
        let color = match &feature.record.match_status {
            MatchStatus::Matching => Color::BLUE,
            MatchStatus::Divergent => Color::from_hex("#dbc200"),
            MatchStatus::Missing => Color::from_hex("#ad0000"),
        };
        primitives.push(RenderPrimitive::new_point_ref(
            point,
            PointPaint::circle(color, size),
        ));
        primitives
    }
}

// use crate::prelude::*;
// use egui::{Align, Layout, Sense, Ui};
// use egui_extras::{Column, TableBuilder};
// use galileo::layer::feature_layer::Feature;
// use galileo_types::cartesian::{CartesianPoint2d, Point2d, Rect};
// use galileo_types::geo::Projection;
// use galileo_types::geometry::{CartesianGeometry2d, Geom, Geometry};
// use polite::Polite;
// use serde::{Deserialize, Serialize};
// use std::fmt;
// use std::path::Path;
// use strum::IntoEnumIterator;
// use strum_macros::EnumIter;
// use tracing::info;
//
// #[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
// pub struct Address {
//     // #[serde(rename(deserialize = "FULLADDRES"))]
//     pub label: String,
//     // #[serde(rename(deserialize = "Add_Number"))]
//     pub number: i64,
//     // #[serde(deserialize_with = "deserialize_mixed_pre_directional",
//     //         rename(deserialize = "St_PreDir"))]
//     pub directional: Option<StreetNamePreDirectional>,
//     // #[serde(rename(deserialize = "St_Name"))]
//     pub street_name: String,
//     // #[serde(deserialize_with = "deserialize_mixed_post_type",
//     //         rename(deserialize = "St_PosTyp"))]
//     pub street_type: Option<StreetNamePostType>,
//     // #[serde(rename(deserialize = "Subaddre_1"))]
//     pub subaddress_id: Option<String>,
//     // #[serde(deserialize_with = "deserialize_mixed_subaddress_type",
//     //         rename(deserialize = "Subaddress"))]
//     pub subaddress_type: Option<SubaddressType>,
//     // #[serde(rename(deserialize = "Post_Code"))]
//     pub zip: i64,
//     // #[serde(rename(deserialize = "STATUS"))]
//     pub status: AddressStatus,
//     // #[serde(rename(deserialize = "wgs84_y"))]
//     pub lat: f64,
//     // #[serde(rename(deserialize = "wgs84_x"))]
//     pub lon: f64,
//     // #[serde(rename(deserialize = "espg3857_x"))]
//     pub x: f64,
//     // #[serde(rename(deserialize = "espg3857_y"))]
//     pub y: f64,
// }
//
// // impl Default for Address {
// //     fn default() -> Self {
// //
// //     }
// // }
//
// impl Address {
//     pub fn column<T: fmt::Display>(&self, columns: &AddressColumns) -> String {
//         match *columns {
//             AddressColumns::Label => format!("{}", self.label),
//             AddressColumns::Number => format!("{}", self.number),
//             AddressColumns::Directional => {
//                 if let Some(prefix) = self.directional {
//                     format!("{}", prefix)
//                 } else {
//                     "".to_string()
//                 }
//             }
//             AddressColumns::StreetName => format!("{}", self.street_name),
//             AddressColumns::StreetType => {
//                 if let Some(value) = &self.street_type {
//                     format!("{}", value.abbreviate())
//                 } else {
//                     "".to_string()
//                 }
//             }
//             AddressColumns::SubaddressType => {
//                 if let Some(subtype) = &self.subaddress_type {
//                     format!("{}", subtype)
//                 } else {
//                     "".to_string()
//                 }
//             }
//             AddressColumns::SubaddressId => {
//                 if let Some(value) = &self.subaddress_id {
//                     format!("{}", value)
//                 } else {
//                     "".to_string()
//                 }
//             }
//             AddressColumns::Zip => format!("{}", self.zip),
//             AddressColumns::Status => format!("{}", self.status),
//         }
//     }
//
//     pub fn columns(&self) -> Vec<String> {
//         let mut values = Vec::new();
//         for column in AddressColumns::iter() {
//             values.push(self.column::<String>(&column));
//         }
//         values
//     }
// }
//
// impl Tabular<Address> for Addresses {
//     fn headers() -> Vec<String> {
//         AddressColumns::names()
//     }
//
//     fn rows(&self) -> Vec<Address> {
//         self.records.clone()
//     }
// }
//
// impl Columnar for Address {
//     fn names() -> Vec<String> {
//         AddressColumns::names()
//     }
//
//     fn values(&self) -> Vec<String> {
//         self.columns()
//     }
// }
//
// impl Card for Address {
//     fn contains(&self, fragment: &str, config: SearchConfig) -> bool {
//         let mut label = self.label.to_owned();
//         let mut status = self.status.to_string();
//         let mut test = fragment.to_string();
//         if !config.case_sensitive {
//             label = label.to_lowercase();
//             status = status.to_lowercase();
//             test = test.to_lowercase();
//         }
//         if label.contains(&test) | status.contains(&test) {
//             true
//         } else {
//             false
//         }
//     }
//
//     fn show(&self, ui: &mut Ui) {
//         ui.label(format!("Address: {}", self.label));
//         ui.label(format!("Status: {}", self.status));
//     }
// }
//
// impl fmt::Display for Address {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Address: {} \n Status: {}", self.label, self.status)
//     }
// }
//
// pub trait RowView<T: fmt::Display> {
//     fn row(&self) -> T;
// }
//
// #[derive(Debug, PartialEq, EnumIter)]
// pub enum AddressColumns {
//     Label,
//     Number,
//     Directional,
//     StreetName,
//     StreetType,
//     SubaddressType,
//     SubaddressId,
//     Zip,
//     Status,
// }
//
// impl AddressColumns {
//     pub fn names() -> Vec<String> {
//         let mut values = Vec::new();
//         for column in AddressColumns::iter() {
//             values.push(format!("{column}"));
//         }
//         values
//     }
// }
//
// impl fmt::Display for AddressColumns {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             Self::Label => write!(f, "Label"),
//             Self::Number => write!(f, "Number"),
//             Self::Directional => write!(f, "Directional Prefix"),
//             Self::StreetName => write!(f, "Street Name"),
//             Self::StreetType => write!(f, "Street Type"),
//             Self::SubaddressType => write!(f, "Subaddress Type"),
//             Self::SubaddressId => write!(f, "Subaddress ID"),
//             Self::Zip => write!(f, "Zip"),
//             Self::Status => write!(f, "Status"),
//         }
//     }
// }
//
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Addresses {
//     pub records: Vec<Address>,
// }
//
// impl Addresses {
//     pub fn from_csv<P: AsRef<Path>>(path: P) -> Polite<Self> {
//         let records = from_csv(path)?;
//         Ok(Addresses { records })
//     }
//
//     pub fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Polite<()> {
//         to_csv(&mut self.records, path)?;
//         Ok(())
//     }
//
//     pub fn save<P: AsRef<Path>>(&self, path: P) -> Polite<()> {
//         save(self, path)
//     }
//
//     pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
//         info!("Deserializing from binary.");
//         let vec: Vec<u8> = std::fs::read(path)?;
//         let values: Addresses = bincode::deserialize(&vec[..])?;
//         Ok(values)
//     }
// }
//
// impl Filtration<Addresses, String> for Addresses {
//     fn filter(self, filter: &String) -> Self {
//         self
//     }
// }
//
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct AddressPoint {
//     pub address: Address,
//     pub geometry: Point2d,
//     pub bounds: Rect,
//     pub selected: bool,
// }
//
// impl From<Address> for AddressPoint {
//     fn from(address: Address) -> Self {
//         let geometry = Point2d::new(address.x, address.y);
//         let bounds = point_bounds(&geometry, 0.05);
//         let selected = false;
//         Self {
//             address,
//             geometry,
//             bounds,
//             selected,
//         }
//     }
// }
//
// impl Geometry for AddressPoint {
//     type Point = Point2d;
//
//     fn project<P: Projection<InPoint = Self::Point> + ?Sized>(
//         &self,
//         projection: &P,
//     ) -> Option<Geom<P::OutPoint>> {
//         self.geometry.project(projection)
//     }
// }
//
// impl CartesianGeometry2d<Point2d> for AddressPoint {
//     fn is_point_inside<Other: CartesianPoint2d<Num = f64>>(
//         &self,
//         point: &Other,
//         tolerance: f64,
//     ) -> bool {
//         if !self.bounds.contains(point) {
//             return false;
//         }
//
//         self.geometry.is_point_inside(point, tolerance)
//     }
//
//     fn bounding_rectangle(&self) -> Option<Rect> {
//         Some(self.bounds)
//     }
// }
//
// impl Feature for AddressPoint {
//     type Geom = Self;
//
//     fn geometry(&self) -> &Self::Geom {
//         self
//     }
// }
//
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct AddressPoints {
//     pub records: Vec<AddressPoint>,
// }
//
// impl From<Addresses> for AddressPoints {
//     fn from(addresses: Addresses) -> Self {
//         let records = addresses
//             .records
//             .iter()
//             .map(|v| AddressPoint::from(v.clone()))
//             .collect::<Vec<AddressPoint>>();
//         Self { records }
//     }
// }
//
// impl AddressPoints {
//     pub fn save<P: AsRef<Path>>(&self, path: P) -> Polite<()> {
//         save(self, path)
//     }
//
//     pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
//         info!("Deserializing from binary.");
//         let vec: Vec<u8> = std::fs::read(path)?;
//         let addresses: AddressPoints = bincode::deserialize(&vec[..])?;
//         Ok(addresses)
//     }
// }
