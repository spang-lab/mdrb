# Spectrum Class

Environment containing methods for the Spectrum class.

## Usage

``` r
Spectrum
```

## Format

An object of class `environment` of length 18.

## Methods

    x <- Spectrum$new(chemical_shifts, intensities, signal_boundaries)
    x$chemical_shifts()
    x$frequency()
    x$intensities()
    x$nucleus()
    x$read_bin(path)
    x$read_bruker(path, experiment, processing, signal_boundaries)
    x$read_bruker_set(path, experiment, processing, signal_boundaries)
    x$read_jcampdx(path, signal_boundaries)
    x$read_json(path)
    x$reference_compound()
    x$set_frequency(frequency)
    x$set_nucleus(nucleus)
    x$set_reference_compound(reference)
    x$set_signal_boundaries(signal_boundaries)
    x$signal_boundaries()
    x$write_bin(path)
    x$write_json(path)

For more information on the methods, see the Rust documentation at
<https://github.com/SombkeMaximilian/metabodecon-rust>.
