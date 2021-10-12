#[allow(dead_code)]
pub enum Units {
    Meter(Meter),
    Millimeter(Millimeter),
    Centimeter(Centimeter),
    Foot(Foot),
    Inch(Inch),
}

#[derive(Debug)]
pub struct Meter;
#[derive(Debug)]
pub struct Millimeter;
#[derive(Debug)]
pub struct Centimeter;
#[derive(Debug)]
pub struct Foot;
#[derive(Debug)]
pub struct Inch;
