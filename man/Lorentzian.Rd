% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/extendr-wrappers.R
\docType{data}
\name{Lorentzian}
\alias{Lorentzian}
\title{Lorentzian Class}
\format{
An object of class \code{environment} of length 12.
}
\usage{
Lorentzian
}
\description{
Environment containing methods for the Lorentzian class.
}
\section{Methods}{


\if{html}{\out{<div class="sourceCode R">}}\preformatted{x <- Lorentzian$new(sf, hw, maxp)  
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
}\if{html}{\out{</div>}}

For more information on the methods, see the Rust documentation at
\url{https://github.com/SombkeMaximilian/metabodecon-rust}.
}

\keyword{datasets}
