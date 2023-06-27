use rand::rngs::ThreadRng;
use super::{Distribution,normal};
use std::f64::consts::PI;
use nalgebra::{DVector,DMatrix};


pub struct MvNormal { }
pub const mvnormal: MvNormal = MvNormal { };

impl Distribution<DVector<f64>,(&DVector<f64>,&DMatrix<f64>)> for MvNormal {
    fn logpdf(&self, x: &DVector<f64>, params: (&DVector<f64>,&DMatrix<f64>)) -> f64 {
        let (mu, cov) = params;
        let k = mu.len() as f64;
        let cov_det = cov.determinant();
        let cov_inv = cov.clone().try_inverse().unwrap();
        let centered_x = x - mu;
        let mahalanobis_squared = (centered_x.transpose() * cov_inv * centered_x).trace();
        -(k*(2.*PI).ln() + cov_det.ln() + mahalanobis_squared)/2.
    }

    fn random(&self, rng: &mut ThreadRng, params: (&DVector<f64>,&DMatrix<f64>)) -> DVector<f64> {
        let (mu, cov) = params;
        let transform: DMatrix<f64>;
        match cov.clone().cholesky() {
            Some(c) => {
                transform = c.l();
            },
            None => {
                let decomp = cov.clone().symmetric_eigen();
                transform = decomp.eigenvectors * DMatrix::from_diagonal(&decomp.eigenvalues.map(|v| v.sqrt()));
            }
        }
        let samples = transform * &mu.map(|m| normal.random(rng, (0.,1.))) + mu;
        samples
    }
}