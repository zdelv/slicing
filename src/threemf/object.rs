use crate::threemf::Mesh;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Object {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type", default)]
    pub otype: String,
    pub mesh: Mesh,
}
