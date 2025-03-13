[![R-CMD-check](https://github.com/spang-lab/mdrb/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/spang-lab/mdrb/actions/workflows/R-CMD-check.yaml)
[![Test-Install](https://github.com/spang-lab/mdrb/actions/workflows/test-install.yaml/badge.svg)](https://github.com/spang-lab/mdrb/actions/workflows/test-install.yaml)

# mdrb

Provides a high-performance Rust backend for the [metabodecon](https://github.com/spang-lab/metabodecon) package, enabling efficient deconvolution, alignment, and post-processing of 1D NMR spectra. The package wraps optimized Rust functions to improve performance and scalability for large datasets. The recommended way to use *mdrb* is by installing *metabodecon* and setting the backend argument to "rust" when calling its functions. The Rust part of the package is based on the [metabodecon-rust](https://github.com/SombkeMaximilian/metabodecon-rust) crate.

⚠️ Attention: this package is experimental and its API is subject to change. When using in scripts and/or packages make sure to check the exact version first.

## Installation

1. Install R version 4.2 or higher from [CRAN](https://cran.r-project.org/).
2. If you're on Windows install RTools from [CRAN](https://cran.r-project.org/).
3. Install the Rust toolchain from [rustup.rs](https://rustup.rs/).
4. Install the *mdrb* package by running the following commands in R:

```R
install.packages("pak")
pak::pkg_install("spang-lab/mdrb")
```

## Usage

*mdrb* is primarily intended to be used through the frontend functions provided by the *metabodecon* package. However, if you prefer to use the package directly, you can follow the example below:

```R
spectrum_path <- metabodecon::metabodecon_file("bruker/urine/urine_1")
r_spectrum <- metabodecon::read_spectrum(spectrum_path)

rust_spectrum <- Spectrum$new(
    chemical_shifts = r_spectrum$cs,
    intensities = r_spectrum$si,
    signal_boundaries = quantile(r_spectrum$cs, c(0.1, 0.9))
)

deconvoluter <- Deconvoluter$new()
deconvoluter$set_moving_average_smoother(iterations = 4, window_size = 3)
deconvoluter$set_noise_score_selector(6.4)
deconvoluter$set_analytical_fitter(iterations = 5)

deconvolution <- deconvoluter$deconvolute_spectrum(rust_spectrum)

lorentzians <- deconvolution$lorentzians()
superposition_vec <- deconvolution$superposition_vec()
mse <- deconvolution$mse()
```

## Documentation

Since *mdrb* is mostly intended as dependency of *metabodecon* and not for direct usage, documentation for *mdrb* is scarce. However, users interested in deconvolution and alignment of NMR spectra are encouraged to check out the *metabodecon* package, which provides a high-level interface to the Rust backend. *metabodecon's* documentation is available at [spang-lab.github.io/metabodecon](https://spang-lab.github.io/metabodecon/) and includes pages about

- [Getting Started](https://spang-lab.github.io/metabodecon/articles/metabodecon.html)
- [Contribution Guidelines](https://spang-lab.github.io/metabodecon/articles/Contributing.html)
- [Function Reference](https://spang-lab.github.io/metabodecon/reference/index.html)

## Contributing

See [CONTRBUTING.md](CONTRIBUTING.md).
