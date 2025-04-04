use crate::deconvolution::Deconvolution;
use crate::spectrum::Spectrum;
use extendr_api::prelude::*;
use metabodecon::deconvolution;
use std::collections::HashMap;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub(crate) struct Deconvoluter {
    inner: deconvolution::Deconvoluter,
    threads: Option<Arc<ThreadPool>>
}

/// @eval make_r_docs("Deconvoluter")
#[extendr]
impl Deconvoluter {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn smoothing_settings(&self) -> Result<List> {
        match self.inner.smoothing_settings() {
            deconvolution::SmoothingSettings::MovingAverage {
                iterations,
                window_size,
            } => {
                let mut result = HashMap::<&str, Robj>::new();
                result.insert("method", "Moving Average Filter".into());
                result.insert("iterations", iterations.into());
                result.insert("window_size", window_size.into());

                List::from_hashmap(result)
            }
            _ => {
                throw_r_error(format!(
                    "Unknown smoothing settings, {:?}",
                    self.inner.smoothing_settings()
                ));
            }
        }
    }

    pub(crate) fn selection_settings(&self) -> Result<List> {
        match self.inner.selection_settings() {
            deconvolution::SelectionSettings::NoiseScoreFilter {
                scoring_method,
                threshold,
            } => {
                let mut result = HashMap::<&str, Robj>::new();
                result.insert("method", "Noise Score Filter".into());
                result.insert("scoring_method", scoring_method.to_string().into());
                result.insert("threshold", threshold.into());

                List::from_hashmap(result)
            }
            _ => {
                throw_r_error(format!(
                    "Unknown selection settings, {:?}",
                    self.inner.selection_settings()
                ));
            }
        }
    }

    pub(crate) fn fitting_settings(&self) -> Result<List> {
        match self.inner.fitting_settings() {
            deconvolution::FittingSettings::Analytical { iterations } => {
                let mut result = HashMap::<&str, Robj>::new();
                result.insert("method", "Analytical Fitter".into());
                result.insert("iterations", iterations.into());

                List::from_hashmap(result)
            }
            _ => {
                throw_r_error(format!(
                    "Unknown fitting settings, {:?}",
                    self.inner.fitting_settings()
                ));
            }
        }
    }

    pub(crate) fn ignore_regions(&self) -> Nullable<List> {
        if let Some(ignore_regions) = self.inner.ignore_regions() {
            let ignore_regions: Vec<Robj> = ignore_regions
                .iter()
                .map(|(start, end)| {
                    let mut result = HashMap::<&str, Robj>::new();
                    result.insert("start", start.into());
                    result.insert("end", end.into());

                    List::from_hashmap(result).into()
                })
                .collect();

            NotNull(List::from_values(ignore_regions))
        } else {
            Null
        }
    }

    pub(crate) fn set_identity_smoother(&mut self) {
        match self.inner.set_smoothing_settings(deconvolution::SmoothingSettings::Identity) {
            Ok(_) => (),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn set_moving_average_smoother(&mut self, iterations: usize, window_size: usize) {
        match self
            .inner
            .set_smoothing_settings(deconvolution::SmoothingSettings::MovingAverage {
                iterations,
                window_size,
            }) {
            Ok(_) => (),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn set_detector_only(&mut self) {
        match self.inner.set_selection_settings(deconvolution::SelectionSettings::DetectorOnly) {
            Ok(_) => (),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn set_noise_score_selector(&mut self, threshold: f64) {
        match self.inner.set_selection_settings(
            deconvolution::SelectionSettings::NoiseScoreFilter {
                scoring_method: deconvolution::ScoringMethod::MinimumSum,
                threshold,
            },
        ) {
            Ok(_) => (),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn set_analytical_fitter(&mut self, iterations: usize) {
        match self
            .inner
            .set_fitting_settings(deconvolution::FittingSettings::Analytical { iterations })
        {
            Ok(_) => (),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn add_ignore_region(&mut self, start: f64, end: f64) {
        match self.inner.add_ignore_region((start, end)) {
            Ok(_) => (),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn clear_ignore_regions(&mut self) {
        self.inner.clear_ignore_regions();
    }

    /// WARNING: These persist when the object is cloned, meaning that two
    /// Deconvoluter objects can share the same thread pool.
    pub(crate) fn set_threads(&mut self, threads: usize) {
        if threads <= 1 {
            throw_r_error("number of threads must be greater than 1");
        } else {
            let thread_pool = match ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
            {
                Ok(thread_pool) => thread_pool,
                Err(error) => throw_r_error(error.to_string()),
            };
            self.threads = Some(Arc::new(thread_pool));
        }
    }

    pub(crate) fn clear_threads(&mut self) {
        self.threads = None;
    }

    pub(crate) fn deconvolute_spectrum(&self, spectrum: &Spectrum) -> Deconvolution {
        match self.inner.deconvolute_spectrum(spectrum.as_ref()) {
            Ok(deconvolution) => deconvolution.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn par_deconvolute_spectrum(&self, spectrum: &Spectrum) -> Deconvolution {
        let deconvolution = match &self.threads {
            Some(threads) => threads.install(|| {
                self.inner.par_deconvolute_spectrum(spectrum.as_ref())
            }),
            None => self.inner.par_deconvolute_spectrum(spectrum.as_ref()),
        };

        match deconvolution {
            Ok(deconvolution) => deconvolution.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn deconvolute_spectra(&self, spectra: List) -> List {
        let spectra = match Spectrum::recover_list(&spectra) {
            Ok(spectra) => spectra,
            Err(error) => throw_r_error(error.to_string()),
        };
        let deconvolutions = match self.inner.deconvolute_spectra(&spectra) {
            Ok(deconvolutions) => deconvolutions
                .into_iter()
                .map(|deconvolution| deconvolution.into())
                .collect::<Vec<Deconvolution>>(),
            Err(error) => throw_r_error(error.to_string()),
        };

        List::from_values(deconvolutions)
    }

    pub(crate) fn par_deconvolute_spectra(&self, spectra: List) -> List {
        let spectra = match Spectrum::recover_list(&spectra) {
            Ok(spectra) => spectra,
            Err(error) => throw_r_error(error.to_string()),
        };
        let deconvolutions = match &self.threads {
            Some(threads) => threads.install(|| {
                self.inner.par_deconvolute_spectra(&spectra)
            }),
            None => self.inner.par_deconvolute_spectra(&spectra),
        };
        let deconvolutions = match deconvolutions {
            Ok(deconvolutions) => deconvolutions
                .into_iter()
                .map(|deconvolution| deconvolution.into())
                .collect::<Vec<Deconvolution>>(),
            Err(error) => throw_r_error(error.to_string()),
        };

        List::from_values(deconvolutions)
    }

    pub(crate) fn optimize_settings(&mut self, reference: &Spectrum) -> f64 {
        match self.inner.optimize_settings(reference.as_ref()) {
            Ok(mse) => mse,
            Err(error) => throw_r_error(error.to_string()),
        }
    }
}

extendr_module! {
    mod deconvoluter;
    impl Deconvoluter;
}
