use crate::structs::Point3d;

//use std::collections::HashMap;
//use std::env;
//use std::fs;

///座標を位置ベクトルと捉えた場合の外積を計算するメソッド
pub fn veprod(v1: &Point3d, v2: &Point3d) -> Point3d {
    let x: f64 = v1.y * v2.z - v2.y * v1.z;
    let y: f64 = v1.z * v2.x - v2.z * v1.x;
    let z: f64 = v1.x * v2.y - v2.x * v1.y;
    return Point3d { x: x, y: y, z: z };
}

///座標を位置ベクトルと捉えた場合の内積を計算するメソッド
pub fn scprod(v1: &Point3d, v2: &Point3d) -> f64 {
    let ret: f64 = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    return ret;
}

///座標を位置ベクトルと捉えた場合の差を計算するメソッド
pub fn sub(p1: &Point3d, p2: &Point3d) -> Point3d {
    let x: f64 = p2.x - p1.x;
    let y: f64 = p2.y - p1.y;
    let z: f64 = p2.z - p1.z;
    return Point3d { x: x, y: y, z: z };
}
