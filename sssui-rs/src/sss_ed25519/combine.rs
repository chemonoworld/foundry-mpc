use crate::{point::Point256, sss_ed25519::interpolate_ed25519};

pub fn sss_combine_ed25519(split_points: Vec<Point256>, t: u32) -> Result<[u8; 32], String> {
    if split_points.len() != t as usize {
        return Err("Not enough keyshare points to combine".to_string());
    }

    // find lagrange coefficient
    let lagrange_coefficient = interpolate_ed25519(&split_points);
    if lagrange_coefficient.is_err() {
        return Err(lagrange_coefficient.err().unwrap());
    }
    let lagrange_coefficient = lagrange_coefficient.unwrap();

    Ok(lagrange_coefficient)
}
