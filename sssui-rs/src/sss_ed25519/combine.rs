use crate::point::Point256;

pub fn sss_combine_ed25519(split_points: Vec<Point256>, t: u32) -> Result<[u8; 32], String> {
    if split_points.len() != t as usize {
        return Err("Not enough keyshare points to combine".to_string());
    }

    // find lagrange coefficient

    Ok([0; 32])
}
