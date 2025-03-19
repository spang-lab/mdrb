use extendr_api::prelude::*;

mod aligner;
mod deconvoluter;
mod deconvolution;
mod lorentzian;
mod spectrum;

extendr_module! {
    mod mdrb;
    use aligner;
    use deconvoluter;
    use deconvolution;
    use lorentzian;
    use spectrum;
}
