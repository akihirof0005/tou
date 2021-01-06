///座標を代表する
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

///分子中の原子を記述する、シンボル、添字、座標、結合先のAtomの添字の配列を格納する
pub struct Atom {
    pub index: u32,
    pub coordinate: Point3d,
    pub symbol: String,
    pub links: Vec<i32>,
}
