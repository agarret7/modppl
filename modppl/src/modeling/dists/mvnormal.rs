use super::{normal, Distribution};
use crate::real::consts::PI;
use crate::Real;
use nalgebra::{DMatrix, DVector};
use rand::rngs::ThreadRng;

/// Multivariate Gaussian distribution type
pub struct MvNormal {}

/// Instantiation of the Multivariate Gaussian distribution
pub const mvnormal: MvNormal = MvNormal {};

impl Distribution<DVector<Real>, (DVector<Real>, DMatrix<Real>)> for MvNormal {
    fn logpdf(&self, x: &DVector<Real>, params: (DVector<Real>, DMatrix<Real>)) -> Real {
        let (mu, cov) = params;
        let k = mu.len() as Real;
        let cov_det = cov.determinant();
        let cov_inv = cov.clone().try_inverse().unwrap();
        let centered_x = x - mu;
        let mahalanobis_squared = (centered_x.transpose() * cov_inv * centered_x).trace();
        -(k * (2. * PI).ln() + cov_det.ln() + mahalanobis_squared) / 2.
    }

    fn random(&self, rng: &mut ThreadRng, params: (DVector<Real>, DMatrix<Real>)) -> DVector<Real> {
        let (mu, cov) = params;
        let transform: DMatrix<Real>;
        match cov.clone().cholesky() {
            Some(c) => {
                transform = c.l();
            }
            None => {
                let decomp = cov.clone().symmetric_eigen();
                transform = decomp.eigenvectors
                    * DMatrix::from_diagonal(&decomp.eigenvalues.map(|v| v.sqrt()));
            }
        }
        let samples = transform * &mu.map(|_| normal.random(rng, (0., 1.))) + mu;
        samples
    }
}
