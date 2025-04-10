use extendr_api::prelude::*;
use metabodecon::deconvolution;
use metabodecon::spectrum;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::HashMap;
use std::sync::Arc;

// Deconvoluter ---------------------------------------------------------------

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

// Deconvolution -------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Deconvolution {
    inner: deconvolution::Deconvolution,
}

impl AsRef<deconvolution::Deconvolution> for Deconvolution {
    fn as_ref(&self) -> &deconvolution::Deconvolution {
        &self.inner
    }
}

impl From<deconvolution::Deconvolution> for Deconvolution {
    fn from(value: deconvolution::Deconvolution) -> Self {
        Self { inner: value }
    }
}

/// @eval make_r_docs("Deconvolution")
#[extendr]
impl Deconvolution {
    pub(crate) fn lorentzians(&self) -> Result<List> {
        let len = self.inner.lorentzians().len();
        let mut sf = Vec::<f64>::with_capacity(len);
        let mut hw = Vec::<f64>::with_capacity(len);
        let mut maxp = Vec::<f64>::with_capacity(len);
        self.inner.lorentzians().iter().for_each(|lorentzian| {
            sf.push(lorentzian.sf());
            hw.push(lorentzian.hw());
            maxp.push(lorentzian.maxp());
        });
        let mut result = HashMap::<&str, Robj>::new();
        result.insert("A", sf.into());
        result.insert("lambda", hw.into());
        result.insert("x0", maxp.into());

        List::from_hashmap(result)
    }

    pub(crate) fn mse(&self) -> f64 {
        self.inner.mse()
    }

    pub(crate) fn superposition(&self, chemical_shift: f64) -> f64 {
        deconvolution::Lorentzian::superposition(chemical_shift, self.inner.lorentzians())
    }

    pub(crate) fn superposition_vec(&self, chemical_shifts: Vec<f64>) -> Vec<f64> {
        deconvolution::Lorentzian::superposition_vec(&chemical_shifts, self.inner.lorentzians())
    }

    pub(crate) fn par_superposition_vec(&self, chemical_shifts: Vec<f64>) -> Vec<f64> {
        deconvolution::Lorentzian::par_superposition_vec(
            &chemical_shifts,
            self.inner.lorentzians(),
        )
    }

    pub(crate) fn write_json(&self, path: &str) {
        let serialized = match serde_json::to_string_pretty(self.as_ref()) {
            Ok(serialized) => serialized,
            Err(error) => throw_r_error(error.to_string()),
        };
        std::fs::write(path, serialized).unwrap();
    }

    pub(crate) fn read_json(path: &str) -> Self {
        let serialized = std::fs::read_to_string(path).unwrap();

        match serde_json::from_str::<deconvolution::Deconvolution>(&serialized) {
            Ok(deserialized) => deserialized.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn write_bin(&self, path: &str) {
        let serialized = match rmp_serde::to_vec(self.as_ref()) {
            Ok(serialized) => serialized,
            Err(error) => throw_r_error(error.to_string()),
        };
        std::fs::write(path, serialized).unwrap();
    }

    pub(crate) fn read_bin(path: &str) -> Self {
        let serialized = std::fs::read(path).unwrap();

        match rmp_serde::from_slice::<deconvolution::Deconvolution>(&serialized) {
            Ok(deserialized) => deserialized.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }
}

// Spectrum ------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Spectrum {
    inner: spectrum::Spectrum,
}

impl AsRef<spectrum::Spectrum> for Spectrum {
    fn as_ref(&self) -> &spectrum::Spectrum {
        &self.inner
    }
}

impl From<spectrum::Spectrum> for Spectrum {
    fn from(value: spectrum::Spectrum) -> Self {
        Self { inner: value }
    }
}

impl TryFrom<&Robj> for Spectrum {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        if let Some(class) = value.class() {
            let class = class.collect::<String>();
            match class.as_str() {
                "Spectrum" => (),
                _ => return Err(Error::from(format!("Expected Spectrum, got {:?}", class))),
            }
        } else {
            return Err(Error::from(format!("Expected Spectrum, got {:?}", value)));
        }
        let ptr: ExternalPtr<Spectrum> = value.try_into()?;

        Ok(ptr.as_ref().clone())
    }
}

impl Spectrum {
    pub(crate) fn recover_list(spectra: &List) -> Result<Vec<Spectrum>> {
        spectra
            .to_vec()
            .iter()
            .map(|r_obj| r_obj.try_into())
            .collect::<Result<Vec<Spectrum>>>()
    }
}

/// @eval make_r_docs("Spectrum")
#[extendr]
impl Spectrum {
    pub(crate) fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: Vec<f64>,
    ) -> Self {
        if signal_boundaries.len() != 2 {
            throw_r_error("signal_boundaries must be a vector of length 2");
        }
        let signal_boundaries = (signal_boundaries[0], signal_boundaries[1]);

        match spectrum::Spectrum::new(chemical_shifts, intensities, signal_boundaries) {
            Ok(spectrum) => spectrum.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn chemical_shifts(&self) -> Vec<f64> {
        self.inner.chemical_shifts().to_vec()
    }

    pub(crate) fn intensities(&self) -> Vec<f64> {
        self.inner.intensities().to_vec()
    }

    pub(crate) fn signal_boundaries(&self) -> Vec<f64> {
        vec![
            self.inner.signal_boundaries().0,
            self.inner.signal_boundaries().1,
        ]
    }

    pub(crate) fn nucleus(&self) -> String {
        self.inner.nucleus().to_string()
    }

    pub(crate) fn frequency(&self) -> f64 {
        self.inner.frequency()
    }

    pub(crate) fn reference_compound(&self) -> Result<List> {
        let reference = self.inner.reference_compound();
        let chemical_shift = reference.chemical_shift();
        let index = reference.index();
        let name = reference.name();
        let method = reference.method().map(|method| method.to_string());
        let mut result = HashMap::<&str, Robj>::new();
        result.insert("chemical_shift", chemical_shift.into());
        result.insert("index", index.into());
        result.insert("name", Nullable::from(name).into());
        result.insert("method", Nullable::from(method).into());

        List::from_hashmap(result)
    }

    pub(crate) fn set_signal_boundaries(&mut self, signal_boundaries: Vec<f64>) {
        if signal_boundaries.len() != 2 {
            throw_r_error("signal_boundaries must be a vector of length 2");
        }
        let signal_boundaries = (signal_boundaries[0], signal_boundaries[1]);

        match self.inner.set_signal_boundaries(signal_boundaries) {
            Ok(_) => (),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn set_nucleus(&mut self, nucleus: &str) {
        self.inner.set_nucleus(nucleus);
    }

    pub(crate) fn set_frequency(&mut self, frequency: f64) {
        self.inner.set_frequency(frequency)
    }

    pub(crate) fn set_reference_compound(&mut self, reference: List) {
        let reference = reference.into_hashmap();
        let chemical_shift = reference
            .get("chemical_shift")
            .unwrap_or_else(|| throw_r_error("missing chemical_shift"))
            .as_real()
            .unwrap_or_else(|| throw_r_error("chemical_shift must be a numeric"));
        let index = reference
            .get("index")
            .unwrap_or_else(|| throw_r_error("missing index"))
            .as_integer()
            .unwrap_or_else(|| throw_r_error("index must be an integer"));
        let name = reference
            .get("name")
            .map(|name| name.as_str().unwrap().to_string());
        let referencing_method = reference
            .get("referencing_method")
            .and_then(|method| std::str::FromStr::from_str(method.as_str().unwrap()).ok());
        let reference = spectrum::meta::ReferenceCompound::new(
            chemical_shift,
            index as usize,
            name,
            referencing_method,
        );
        self.inner.set_reference_compound(reference);
    }

    pub(crate) fn read_bruker(
        path: &str,
        experiment: u32,
        processing: u32,
        signal_boundaries: Vec<f64>,
    ) -> Self {
        if signal_boundaries.len() != 2 {
            throw_r_error("signal_boundaries must be a vector of length 2");
        }
        let signal_boundaries = (signal_boundaries[0], signal_boundaries[1]);

        match spectrum::Bruker::read_spectrum(path, experiment, processing, signal_boundaries) {
            Ok(spectrum) => spectrum.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn read_bruker_set(
        path: &str,
        experiment: u32,
        processing: u32,
        signal_boundaries: Vec<f64>,
    ) -> List {
        if signal_boundaries.len() != 2 {
            throw_r_error("signal_boundaries must be a vector of length 2");
        }
        let signal_boundaries = (signal_boundaries[0], signal_boundaries[1]);
        let spectra =
            match spectrum::Bruker::read_spectra(path, experiment, processing, signal_boundaries) {
                Ok(spectra) => spectra
                    .into_iter()
                    .map(|spectrum| spectrum.into())
                    .collect::<Vec<Spectrum>>(),
                Err(error) => throw_r_error(error.to_string()),
            };

        List::from_values(spectra)
    }

    pub(crate) fn read_jcampdx(path: &str, signal_boundaries: Vec<f64>) -> Self {
        if signal_boundaries.len() != 2 {
            throw_r_error("signal_boundaries must be a vector of length 2");
        }
        let signal_boundaries = (signal_boundaries[0], signal_boundaries[1]);

        match spectrum::JcampDx::read_spectrum(path, signal_boundaries) {
            Ok(spectrum) => spectrum.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn write_json(&self, path: &str) {
        let serialized = match serde_json::to_string_pretty(self.as_ref()) {
            Ok(serialized) => serialized,
            Err(error) => throw_r_error(error.to_string()),
        };
        std::fs::write(path, serialized).unwrap();
    }

    pub(crate) fn read_json(path: &str) -> Self {
        let serialized = std::fs::read_to_string(path).unwrap();

        match serde_json::from_str::<spectrum::Spectrum>(&serialized) {
            Ok(deserialized) => deserialized.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }

    pub(crate) fn write_bin(&self, path: &str) {
        let serialized = match rmp_serde::to_vec(self.as_ref()) {
            Ok(serialized) => serialized,
            Err(error) => throw_r_error(error.to_string()),
        };
        std::fs::write(path, serialized).unwrap();
    }

    pub(crate) fn read_bin(path: &str) -> Self {
        let serialized = std::fs::read(path).unwrap();

        match rmp_serde::from_slice::<spectrum::Spectrum>(&serialized) {
            Ok(deserialized) => deserialized.into(),
            Err(error) => throw_r_error(error.to_string()),
        }
    }
}

// Lorentzian -----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub(crate) struct Lorentzian {
    inner: deconvolution::Lorentzian,
}

impl AsRef<deconvolution::Lorentzian> for Lorentzian {
    fn as_ref(&self) -> &deconvolution::Lorentzian {
        &self.inner
    }
}

impl From<deconvolution::Lorentzian> for Lorentzian {
    fn from(inner: deconvolution::Lorentzian) -> Self {
        Self { inner }
    }
}

impl Lorentzian {
    pub(crate) fn inner_from_parameters(
        sf: Vec<f64>,
        hw: Vec<f64>,
        maxp: Vec<f64>,
    ) -> Vec<deconvolution::Lorentzian> {
        if sf.len() != hw.len() || sf.len() != maxp.len() {
            throw_r_error("Length of sf, hw, and maxp must be equal.");
        }

        sf.iter()
            .zip(hw.iter())
            .zip(maxp.iter())
            .map(|((sf, hw), maxp)| deconvolution::Lorentzian::new(sf * hw, hw.powi(2), *maxp))
            .collect()
    }
}

/// @eval make_r_docs("Lorentzian")
#[extendr]
impl Lorentzian {
    pub(crate) fn new(sf: f64, hw: f64, maxp: f64) -> Self {
        Self {
            inner: deconvolution::Lorentzian::new(sf * hw, hw.powi(2), maxp),
        }
    }

    pub(crate) fn sf(&self) -> f64 {
        self.inner.sf()
    }

    pub(crate) fn hw(&self) -> f64 {
        self.inner.hw()
    }

    pub(crate) fn maxp(&self) -> f64 {
        self.inner.maxp()
    }

    pub(crate) fn set_sf(&mut self, sf: f64) {
        self.inner.set_sf(sf);
    }

    pub(crate) fn set_hw(&mut self, hw: f64) {
        self.inner.set_hw(hw);
    }

    pub(crate) fn set_maxp(&mut self, maxp: f64) {
        self.inner.set_maxp(maxp);
    }

    pub(crate) fn evaluate(&self, x: f64) -> f64 {
        self.inner.evaluate(x)
    }

    pub(crate) fn evaluate_vec(&self, x: Vec<f64>) -> Vec<f64> {
        self.inner.evaluate_vec(&x)
    }

    pub(crate) fn superposition(x: f64, sf: Vec<f64>, hw: Vec<f64>, maxp: Vec<f64>) -> f64 {
        let lorentzians = Self::inner_from_parameters(sf, hw, maxp);

        deconvolution::Lorentzian::superposition(x, &lorentzians)
    }

    pub(crate) fn superposition_vec(
        x: Vec<f64>,
        sf: Vec<f64>,
        hw: Vec<f64>,
        maxp: Vec<f64>,
    ) -> Vec<f64> {
        let lorentzians = Self::inner_from_parameters(sf, hw, maxp);

        deconvolution::Lorentzian::superposition_vec(&x, &lorentzians)
    }

    pub(crate) fn par_superposition_vec(
        x: Vec<f64>,
        sf: Vec<f64>,
        hw: Vec<f64>,
        maxp: Vec<f64>,
    ) -> Vec<f64> {
        let lorentzians = Self::inner_from_parameters(sf, hw, maxp);

        deconvolution::Lorentzian::par_superposition_vec(&x, &lorentzians)
    }
}

// Module ---------------------------------------------------------------------

extendr_module! {
    mod mdrb;
    impl Deconvoluter;
    impl Deconvolution;
    impl Lorentzian;
    impl Spectrum;
}
