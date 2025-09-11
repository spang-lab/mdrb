use crate::deconvolution::Deconvolution;
use extendr_api::prelude::*;
use metabodecon::alignment;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub(crate) struct Aligner {
    inner: alignment::Aligner,
    threads: Option<Arc<ThreadPool>>,
}

#[extendr]
impl Aligner {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn alignment_strategy(&self) -> Result<List> {
        match self.inner.alignment_strategy() {
            alignment::AlignmentStrategy::Reference(index) => {
                let mut result = HashMap::<&str, Robj>::new();
                result.insert("method", "Reference-based Alignment".into());
                result.insert("reference", index.into());

                List::from_hashmap(result)
            }
            alignment::AlignmentStrategy::Pairwise => {
                let mut result = HashMap::<&str, Robj>::new();
                result.insert("method", "Pairwise Alignment".into());

                List::from_hashmap(result)
            }
            _ => {
                throw_r_error(format!(
                    "Unknown alignment strategy, {:?}",
                    self.inner.alignment_strategy()
                ));
            }
        }
    }

    pub(crate) fn filtering_settings(&self) -> Result<List> {
        match self.inner.filtering_settings() {
            alignment::FilteringSettings::DistanceSimilarity {
                similarity_metric,
                max_distance,
                min_similarity,
            } => {
                let mut result = HashMap::<&str, Robj>::new();
                result.insert("method", "Distance Similarity Filter".into());
                result.insert("similarity_metric", similarity_metric.to_string().into());
                result.insert("max_distance", max_distance.into());
                result.insert("min_similarity", min_similarity.into());

                List::from_hashmap(result)
            }
            _ => {
                throw_r_error(format!(
                    "Unknown filtering settings, {:?}",
                    self.inner.filtering_settings()
                ));
            }
        }
    }

    pub(crate) fn solving_settings(&self) -> Result<List> {
        match self.inner.solving_settings() {
            alignment::SolvingSettings::LinearProgramming => {
                let mut result = HashMap::<&str, Robj>::new();
                result.insert("method", "Linear Programming Solver".into());

                List::from_hashmap(result)
            }
            _ => {
                throw_r_error(format!(
                    "Unknown solving settings, {:?}",
                    self.inner.solving_settings()
                ));
            }
        }
    }

    pub(crate) fn set_reference_alignment(&mut self, index: usize) {
        match self
            .inner
            .set_alignment_strategy(alignment::AlignmentStrategy::Reference(index))
        {
            Ok(_) => {}
            Err(error) => throw_r_error(format!("{}", error)),
        }
    }

    pub(crate) fn set_pairwise_alignment(&mut self) {
        match self
            .inner
            .set_alignment_strategy(alignment::AlignmentStrategy::Pairwise)
        {
            Ok(_) => {}
            Err(error) => throw_r_error(format!("{}", error)),
        }
    }

    pub(crate) fn set_distance_similarity_filter(
        &mut self,
        similarity_metric: &str,
        max_distance: f64,
        min_similarity: f64,
    ) {
        let similarity_metric = match similarity_metric {
            "shape" => alignment::SimilarityMetric::Shape,
            "shape_distance" => alignment::SimilarityMetric::ShapeDistance,
            _ => throw_r_error(format!("Unknown similarity metric: {}", similarity_metric)),
        };

        match self
            .inner
            .set_filtering_settings(alignment::FilteringSettings::DistanceSimilarity {
                similarity_metric,
                max_distance,
                min_similarity,
            }) {
            Ok(_) => {}
            Err(error) => throw_r_error(format!("{}", error)),
        }
    }

    pub(crate) fn set_linear_programming_solver(&mut self) {
        match self
            .inner
            .set_solving_settings(alignment::SolvingSettings::LinearProgramming)
        {
            Ok(_) => {}
            Err(error) => throw_r_error(format!("{}", error)),
        }
    }

    /// WARNING: These persist when the object is cloned, meaning that two
    /// Aligner objects can share the same thread pool.
    pub(crate) fn set_threads(&mut self, threads: usize) {
        if threads <= 1 {
            throw_r_error("number of threads must be greater than 1");
        } else {
            let thread_pool = match ThreadPoolBuilder::new().num_threads(threads).build() {
                Ok(thread_pool) => thread_pool,
                Err(error) => throw_r_error(error.to_string()),
            };
            self.threads = Some(Arc::new(thread_pool));
        }
    }

    pub(crate) fn clear_threads(&mut self) {
        self.threads = None;
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
