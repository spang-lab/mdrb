# Deconvoluter Class

Environment containing methods for the Deconvoluter class.

## Usage

``` r
Deconvoluter
```

## Format

An object of class `environment` of length 19.

## Methods

    x <- Deconvoluter$new()
    x$add_ignore_region(start, end)
    x$clear_ignore_regions()
    x$clear_threads()
    x$deconvolute_spectra(spectra)
    x$deconvolute_spectrum(spectrum)
    x$fitting_settings()
    x$ignore_regions()
    x$optimize_settings(reference)
    x$par_deconvolute_spectra(spectra)
    x$par_deconvolute_spectrum(spectrum)
    x$selection_settings()
    x$set_analytical_fitter(iterations)
    x$set_detector_only()
    x$set_identity_smoother()
    x$set_moving_average_smoother(iterations, window_size)
    x$set_noise_score_selector(threshold)
    x$set_threads(threads)
    x$smoothing_settings()

For more information on the methods, see the Rust documentation at
<https://github.com/SombkeMaximilian/metabodecon-rust>.
