use crate::deconvolution::Deconvolution;
use extendr_api::prelude::*;
use metabodecon::alignment;

pub(crate) struct Aligner {
    inner: alignment::Aligner,
}

#[extendr]
impl Aligner {
    pub(crate) fn new(max_distance: f64, min_similarity: f64) -> Self {
        Self {
            inner: alignment::Aligner::new(max_distance, min_similarity),
        }
    }

    pub(crate) fn align_deconvolutions(&self, deconvolutions: List) -> List {
        let deconvolutions = match Deconvolution::recover_list(&deconvolutions) {
            Ok(deconvolutions) => deconvolutions,
            Err(error) => throw_r_error(format!("{}", error)),
        };
        let alignment = self.inner.align_deconvolutions(&deconvolutions);
        let aligned_deconvolutions = alignment
            .deconvolutions()
            .iter()
            .map(|deconvolution| deconvolution.clone().into())
            .collect::<Vec<Deconvolution>>();

        List::from_values(aligned_deconvolutions)
    }
}

extendr_module! {
    mod aligner;
    impl Aligner;
}
