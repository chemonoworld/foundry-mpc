use std::collections::BTreeSet;

use frost_core::{Ciphersuite, Error, Field, Group, Identifier, Scalar};

use crate::point::Point256;

pub fn compute_lagrange_coefficient<C: Ciphersuite>(
    x_set: &BTreeSet<Identifier<C>>,
    x: Option<Identifier<C>>,
    x_i: Identifier<C>,
) -> Result<Scalar<C>, Error<C>> {
    if x_set.is_empty() {
        return Err(Error::IncorrectNumberOfIdentifiers);
    }
    let mut num = <<C::Group as Group>::Field>::one();
    let mut den = <<C::Group as Group>::Field>::one();

    let mut x_i_found = false;

    for x_j in x_set.iter() {
        if x_i == *x_j {
            x_i_found = true;
            continue;
        }

        if let Some(x) = x {
            num = num * (x.to_scalar() - x_j.to_scalar());
            den = den * (x_i.to_scalar() - x_j.to_scalar());
        } else {
            // Both signs inverted just to avoid requiring Neg (-*xj)
            num = num * x_j.to_scalar();
            den = den * (x_j.to_scalar() - x_i.to_scalar());
        }
    }
    if !x_i_found {
        return Err(Error::UnknownIdentifier);
    }

    Ok(
        num * <<C::Group as Group>::Field>::invert(&den)
            .map_err(|_| Error::DuplicatedIdentifier)?,
    )
}

pub fn interpolate_ed25519<C: Ciphersuite>(keyshares: &Vec<Point256>) -> Result<[u8; 32], String> {
    let x_vec = keyshares.iter().map(|k| k.x).collect::<Vec<_>>();
    unimplemented!()
    // 1. x_vec -> identifiers
    // 2. call compute_lagrange_coefficient -> Vec<Scalar>(coeffs)
    // 3. sum(coeffs * keyshares.y) -> Scalar
    // 4. Scalar -> [u8; 32]
    // 5. Ok([u8; 32])
}
