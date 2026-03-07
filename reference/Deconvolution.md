# Deconvolution Class

Environment containing methods for the Deconvolution class.

## Usage

``` r
Deconvolution
```

## Format

An object of class `environment` of length 9.

## Methods

    # Assuming x is an object of class Deconvolution
    x$lorentzians()
    x$mse()
    x$par_superposition_vec(chemical_shifts)
    x$read_bin(path)
    x$read_json(path)
    x$superposition(chemical_shift)
    x$superposition_vec(chemical_shifts)
    x$write_bin(path)
    x$write_json(path)

For more information on the methods, see the Rust documentation at
<https://github.com/SombkeMaximilian/metabodecon-rust>.
