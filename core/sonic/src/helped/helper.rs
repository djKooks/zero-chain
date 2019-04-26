//! Our protocol allows the verification of multiple proofs and even
//! of individual proofs to batch the pairing operations such that
//! only a smaller, fixed number of pairings must occur for an entire
//! batch of proofs. This is possible because G2 elements are fixed
//! in our protocol and never appear in proofs; everything can be
//! combined probabilistically.
//!
//! The helper protocol for computing aggregated signatures
//! of correct computation to ensure that an element `s` is equal to `s(z, y)` for
//! known polynomial.
//! The helper algorithm is run on a batch of proofs.
//!
//! This submodule contains the `Batch` abstraction for creating a
//! context for batch verification.

use pairing::{Engine, CurveAffine, CurveProjective};
use crate::srs::SRS;

pub struct Batch<E: Engine> {
    /// Context of openings of polynomial commitment
    alpha_x: Vec<(E::G1Affine, E::Fr)>,
    alpha_x_precomp: <E::G2Affine as CurveAffine>::Prepared,

    /// Context of openings of polynomial commitment
    alpha: Vec<(E::G1Affine, E::Fr)>,
    alpha_precomp: <E::G2Affine as CurveAffine>::Prepared,

    /// Context of polynomial commitment and randomness
    neg_h: Vec<(E::G1Affine, E::Fr)>,
    neg_h_precomp: <E::G2Affine as CurveAffine>::Prepared,

    neg_x_n_minus_d: Vec<(E::G1Affine, E::Fr)>,
    neg_x_n_minus_d_precomp: <E::G2Affine as CurveAffine>::Prepared,

    value: E::Fr,
    g: E::G1Affine,
}

impl<E: Engine> Batch<E> {
    pub fn new(srs: &SRS<E>, n: usize) -> Self {
        Batch {
            alpha_x: vec![],
            // Prepares `alpha * h^{x^{1}}` for pairing purposes.
            alpha_x_precomp: srs.h_pos_x_alpha[1].prepare(),

            alpha: vec![],
            // Prepares `alpha * h^{x^{0}}` for pairing purposes.
            alpha_precomp: srs.h_pos_x_alpha[0].prepare(),

            neg_h: vec![],
            // Prepares `-h^{x^0}` for pairing purposes.
            neg_h_precomp: {
                let mut tmp = srs.h_neg_x[0];
                tmp.negate();
                tmp.prepare()
            },

            neg_x_n_minus_d: vec![],
            // Prepares `-h^{x^{d-n}}` for pairing purposes.
            neg_x_n_minus_d_precomp: {
                let mut tmp = srs.h_neg_x[srs.d - n];
                tmp.negate();
                tmp.prepare()
            },

            value: E::Fr::zero(),
            g: srs.g_pos_x[0], // g^{x^0}
        }
    }

    pub fn add_comm(&mut self, comm: E::G1Affine, random: E::Fr) {
        self.neg_h.push((comm, random));
    }

    pub fn add_opening(&mut self, opening: E::G1Affine, mut random: E::Fr, point: E::Fr) {
        self.alpha_x.push((opening, random));


    }

    pub fn check_all(mut self) -> bool {
        unimplemented!();
    }
}


