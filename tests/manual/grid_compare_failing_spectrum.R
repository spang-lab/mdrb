# Manual regression test for mdrb's grid deconvolution stability.
#
# Picks one AKI spectrum where the rust backend silently produced
# zero peaks for some configs on a prior run (AKI_8_24_18_110722)
# and re-runs the full default grid_deconvolute_spectrum() grid
# (60 cells) with both the R reference backend and the rust
# backend. Prints a side-by-side table of per-cell peak counts
# plus per-backend wall-clock runtime.
#
# Needs to be run manually for now. Should maybe be added to
# github actions at some point. Command to run:
#
#   Rscript tests/manual/grid_compare_failing_spectrum.R
#
# This script is self-contained: it pulls the AKI dataset via
# metabodecon's example-data helpers and uses the package's default
# 60-cell grid (nfit=10 x smit=1:3 x smws=c(3,5,7,9) x delta=(1:5)*1.6).
suppressPackageStartupMessages({
    library(metabodecon)
    library(mdrb)
})

target <- "AKI_8_24_18_110722"
cat(sprintf("mdrb        = %s\n", as.character(packageVersion("mdrb"))))
cat(sprintf("metabodecon = %s\n", as.character(packageVersion("metabodecon"))))
cat(sprintf("target      = %s\n\n", target))

# Load AKI, pick the failing spectrum.
aki <- metabodecon:::read_aki_data()
stopifnot(target %in% names(aki$spectra))
s <- aki$spectra[[target]]

run_one <- function(use_rust) {
    label <- if (use_rust) "rust" else "R"
    cat(sprintf("[%s] grid_deconvolute_spectrum (60 cells)...\n", label))
    t0 <- Sys.time()
    enriched <- metabodecon:::grid_deconvolute_spectrum(x=s, use_rust=use_rust, verbose=FALSE)
    dt <- as.numeric(difftime(Sys.time(), t0, units = "secs"))
    cat(sprintf("[%s] runtime: %.2fs\n", label, dt))
    list(deg = enriched$deg, runtime = dt)
}

res_R <- run_one(use_rust=FALSE)
res_rust <- run_one(use_rust=TRUE)

deg <- res_R$deg
stopifnot(identical(
    deg[, c("nfit", "smit", "smws", "delta")],
    res_rust$deg[, c("nfit", "smit", "smws", "delta")]
))
out <- data.frame(
    cfg=seq_len(nrow(deg)),
    nfit=deg$nfit, smit=deg$smit, smws=deg$smws, delta=deg$delta,
    np_R=deg$np, np_rust=res_rust$deg$np,
    diff=res_rust$deg$np - deg$np,
    stringsAsFactors=FALSE
)
n_zero <- sum(out$np_rust == 0)
n_R0 <- sum(out$np_R == 0)

cat("\n=========================================================\n")
cat(sprintf("Peak counts per config for spectrum '%s'\n", target))
cat("=========================================================\n")
print(out, row.names = FALSE, right = TRUE)

cat("\n---------------------------------------------------------\n")
cat(sprintf("Configs with rust np == 0 : %d / %d\n", n_zero, nrow(out)))
cat(sprintf("Configs with R    np == 0 : %d / %d\n", n_R0,   nrow(out)))
cat(sprintf("\nRuntime  R    : %7.2f s\n", res_R$runtime))
cat(sprintf("Runtime  rust : %7.2f s\n", res_rust$runtime))
cat(sprintf("Speedup  rust/R: %5.2fx\n", res_R$runtime / res_rust$runtime))

invisible(out)
