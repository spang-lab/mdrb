# =~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=
# The 'metabodecon' Rust crate
# (https://github.com/SombkeMaximilian/metabodecon-rust) provides a set of
# structs that can be used to perform deconvolution of NMR spectra. The goal of
# this module to provide access to these structs and their methods from R.
# =~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=

#' @export
#' @title Create New Spectrum
#' @description Creates a new Spectrum object. See 'Details' for a list of methods.
#' @param chemical_shifts The chemical shifts of the spectrum.
#' @param intensities The intensities of the spectrum.
#' @param signal_boundaries The signal boundaries of the spectrum.
#' @return A Spectrum object, i.e. a pointer with class "Spectrum".
#' @details `r document_details("Spectrum")`
#' @examples
#' mdrb_spectrum <- new_Spectrum(
#'      chemical_shifts   = seq(14.8, -5.2, length.out = 1024),
#'      intensities       = rnorm(1024, 10000, 1000),
#'      signal_boundaries = c(12.8, -3.2)
#' )
#' cs <- mdrb_spectrum$chemical_shifts() # Example method call
new_Spectrum <- function(chemical_shifts, intensities, signal_boundaries) {
    Spectrum$new(chemical_shifts, intensities, signal_boundaries)
}

#' @export
#' @title Create New Deconvoluter
#' @description Creates a new Deconvoluter object. See 'Details' for a list of methods.
#' @return A Deconvoluter object, i.e. a pointer with class "Deconvoluter".
#' @details `r document_details("Deconvoluter")`
#' @examples
#' mdrb_Deconvoluter <- new_Deconvoluter()
new_Deconvoluter <- function() {
    Deconvoluter$new()
}


#' @export
#' @title Deconvolution Class
#' @description Environment containing methods for the Deconvolution class.
#' @details `r document_details("Deconvolution")`
Deconvolution <- function() {
    Deconvolution$new()
}

#' @export
#' @title Create New Lorentzian
#' @description Creates a new Lorentzian object. See 'Details' for a list of methods.
#' @return A Lorentzian object, i.e. a pointer with class "Lorentzian".
#' @details `r document_details("Lorentzian")`
new_Deconvolution <- function() {
    Lorentzian$new()
}
