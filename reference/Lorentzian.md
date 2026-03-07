# Lorentzian Class

Environment containing methods for the Lorentzian class.

## Usage

``` r
Lorentzian
```

## Format

An object of class `environment` of length 12.

## Methods

    x <- Lorentzian$new(sf, hw, maxp)
    x$evaluate(x)
    x$evaluate_vec(x)
    x$hw()
    x$maxp()
    x$par_superposition_vec(x, sf, hw, maxp)
    x$set_hw(hw)
    x$set_maxp(maxp)
    x$set_sf(sf)
    x$sf()
    x$superposition(x, sf, hw, maxp)
    x$superposition_vec(x, sf, hw, maxp)

For more information on the methods, see the Rust documentation at
<https://github.com/SombkeMaximilian/metabodecon-rust>.
