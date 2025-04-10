# Always start with an issue

To contribute to this package, you should follow the below steps:

1. Create a issue at [github.com/spang-lab/mdrb/issues](https://github.com/spang-lab/mdrb/issues) describing the problem or feature you want to work on.
2. Wait until the issue is approved by a package maintainer.
3. Create a fork of the repository at [github.com/spang-lab/mdrb](https://github.com/spang-lab/mdrb)
4. Make your edits as described in section [Making Edits](#making-edits)
5. Create a pull request at [github.com/spang-lab/mdrb/pulls](https://github.com/spang-lab/mdrb/pulls)

# Making Edits

## Install Dependencies

Before you make edits, ensure you have the following dependencies installed:

1. R version 4.2 (from [CRAN](https://cran.r-project.org/))
2. Rust and Cargo (from [rust-lang.org](https://www.rust-lang.org/tools/install))
3. RTools (from [CRAN](https://cran.r-project.org/), Windows only)
4. R packages `devtools` and `rextendr` (via `install.packages("devtools", "rextendr")`)

## Update the Package

After you installed all dependencies, you can edit:

1. R code and documentation in folder [R](R)
2. Rust code and documentation in folder [src/rust](src/rust)
3. Package documentation in folder `vignettes`
4. Test cases in folder [tests](tests)
5. Authors in file [DESCRIPTION](DESCRIPTION)
6. R dependencies in file [DESCRIPTION](DESCRIPTION)
7. Rust dependencies in file [src/rust/Cargo.toml](src/rust/Cargo.toml).
8. Updating Rust build scripts e.g. [Makevars](src/Makevars) or [cleanup](cleanup). For details see question [Why do we need a cleanup script?](#why-do-we-need-cleanup) in the [FAQ](#faq).

## Test the Package

After every update to the package, you should:

1. run `devtools::load_all()` and interactively test that everything still works as expected
2. run the commands below to do a formal check of the package

```R
rextendr::document() # Build the shared Rust object and R wrapper functions
devtools::document() # Build documentation in man folder
devtools::spell_check() # Check spelling (add false positives to inst/WORDLIST)
urlchecker::url_check() # Check URLs
devtools::test() # Execute tests from tests folder inkl. slow tests
devtools::run_examples(run_donttest = TRUE) # Run all examples in the package
devtools::check() # Check package formalities
devtools::install() # Install as required by next commands
toscutil::check_pkg_docs() # Check function documentation for missing tags
pkgdown::build_site() # Build website in docs folder
system("sh test_build.sh") # Test building of bundled and binary packages (1)
# (1) The test_build.sh script should be used with care. It has been written
# very recently and has only been tested on Windows 11 and WSL-Ubuntu-24.
# If it fails, it's very possible that the script is failing and not the package.
# If this happens, dont ignore the failure, but fix the script!
```

After doing these steps, you can push your changes to Github and create a pull request.

# Releasing to CRAN

Whenever a package maintainer wants to release a new version of the package to CRAN, they should:

1. Check whether the [release requirements](https://r-pkgs.org/release.html#sec-release-initial) are fulfilled
2. Use the following commands to do a final check of the package and release it to CRAN

```R
# Check spelling and URLs. False positive findings of spell check should be
# added to inst/WORDLIST.
devtools::spell_check()
urlchecker::url_check()

# Slower, but more realistic tests than devtools::check()
rcmdcheck::rcmdcheck(
    args = c("--no-manual", "--as-cran"),
    build_args = ("--no-manual"),
    error_on = ("warning"),
    check_dir = "../mdrb-RCMDcheck"
)
devtools::check(
    remote = TRUE,
    manual = TRUE,
    run_dont_test = TRUE
)

# Check reverse dependencies. For details see:
# https://r-pkgs.org/release.html#sec-release-revdep-checks
revdepcheck::revdep_check(num_workers = 8)

# Send your package to CRAN's builder services. You should receive an e-mail
# within about 30 minutes with a link to the check results. Checking with
# check_win_devel is required by CRAN policy and will (also) be done as part
# of CRAN's incoming checks.
devtools::check_win_oldrelease()
devtools::check_win_release()
devtools::check_win_devel()
devtools::check_mac_release()

# Use the following command to submit the package to CRAN of submit via the web
# interface available at https://cran.r-project.org/submit.html.
devtools::submit_cran()
```

# FAQ

## How to use the Rust bindings from R?

```R
# Read the spectra
spectra_path <- metabodecon::metabodecon_file("bruker/urine")
metabodecon_spectra <- metabodecon::read_spectra(spectra_path, "bruker", 10, 10)
mdrb_spectra <- lapply(metabodecon_spectra, function(s) Spectrum$new(s$cs, s$si, c(-2.2, 11.8)))

# Configure the Deconvoluter
deconvoluter <- Deconvoluter$new()
deconvoluter$set_moving_average_smoother(4, 3)
deconvoluter$add_ignore_region(4.7, 4.9)

# Deconvolute the spectra
deconvolutions <- deconvoluter$deconvolute_spectra(mdrb_spectra)
deconvolutions <- deconvoluter$par_deconvolute_spectra(mdrb_spectra)

# Serialization
mdrb_spectra[[1]]$write_json("spectrum.json")
mdrb_spectra[[1]]$write_bin("spectrum.bin")
deconvolutions[[1]]$write_json("deconvolution.json")
deconvolutions[[1]]$write_bin("deconvolution.bin")

# Deserialization
json_spectrum <- Spectrum$read_json("spectrum.json")
bin_spectrum <- Spectrum$read_bin("spectrum.bin")
json_deconvolution <- Deconvolution$read_json("deconvolution.json")
bin_deconvolution <- Deconvolution$read_bin("deconvolution.bin")

# Getting the Lorentzian parameters
lorentzians <- lapply(deconvolutions, function(d) d$lorentzians())

# Compute the superposition of the Lorentzians for the first spectrum
superposition_internal <- deconvolutions[[1]]$par_superposition_vec(spectra[[1]]$chemical_shifts())

# Alternative method
A <- lorentzians[[1]]$A
lambda <- lorentzians[[1]]$lambda
x0 <- lorentzians[[1]]$x0
superposition_parameters <- Lorentzian$par_superposition_vec(spectra[[1]]$chemical_shifts(), A, lambda, x0)
```

## How does R compile Rust during package installation?

Assume the following `src/Makevars` file:

```Makefile
PKG_LIBS = -Lrust/target/release -l_mdrb

$(SHLIB): rust/target/release/lib_mdrb.a

rust/target/release/lib_mdrb.a:
    export PATH="$(PATH):$(HOME)/.cargo/bin" && \
    cargo build \
        --lib \
        --release \
        --manifest-path=rust/Cargo.toml \
        --target-dir rust/target
```

The following happens when you run `install.packages("mdrb", repos = NULL, type = "source")`:

1. R detects that the package contains Rust code.

2. R defines the following environment variables, which can be used to customize the build process in the following steps:
   - `R_HOME`: Root directory of R installation
   - `SHLIB`:  Default name of shared library to be built (mdrb.so on
     Linux and mdrb.dll on Windows)
   - `R_ARCH`: Architecture, e.g. x86_64 or i386

3. R calls `make -f $R_HOME/**/Makeconv -f Makevars` to build the package

4. In `Makeconf` we have:

   ```Makefile
   ALL_LIBS = $(PKG_LIBS) $(LOCAL_LIBS) $(SHLIB_LIBADD) $(LIBR) $(LIBINTL)
   %.dll:
       @echo EXPORTS > $*.def
       @$(NM) $^ | $(SED) -n $(SYMPAT) >> $*.def
       $(SHLIB_LD) $(SHLIB_LDFLAGS) $(DLLFLAGS) -o $@ $*.def $^ $(ALL_LIBS)
       @$(RM) $*.def
   ```

   This means that the dynamic link library is built by linking against the libraries defined by `PKG_LIBS`. By setting `PKG_LIBS` to `-Lrust/target/release -l_mdrb`, we tell the linker to link against a library named `lib_mdrb` from directory `rust/target/release`.

5. Because `SHLIB` was set to `mdrb.dll` in step two, the `%.dll` rule will trigger the `SHLIB` rule, which will cause the creation of `rust/target/release/lib_mdrb.a`.

6. After that, make will continue to execute the commands defined by `%.dll` and invoke the linker to create `mdrb.dll` from `lib_mdrb.a`

## Why do we need a cleanup script?

According to [R Packages](https://r-pkgs.org/structure.html#fig-package-files) there are three major states an R package can exist in:

1. *Source*: A source package is the package in its raw form, as you get after a fresh git clone of the package repo

2. *Bundled*: A bundled package is a source package that has been preprocessed by `R CMD build` or `devtools::build()`, i.e. it contains compiled vignettes and has been stripped of all files ignored by .Rbuildignore

3. *Binary*: A binary package is what you get after performing an installation of the package and compressing the resulting files into a single file. I.e. all data files, R files and manuals have been compiled into binary representations and low level source code has been compiled into machine code.

Depending on the state of the package, different actions and files are required or forbidden. E.g.:

| File                   | Source  | Bundle    | Binary    |
| ---------------------- | ------- | --------- | --------- |
| src/rust/vendor.tar.xz | ignored | required  | ignored   |
| src/rust/vendor        | ignored | forbidden | forbidden |
| src/mdrb.dll           | ignored | forbidden | required  |
| downloading            | allowed | forbidden | required  |

I.e., we need to setup the package scripts in a way that they produce the right set of files for each of the three states. `cleanup` is called during conversion from Source to Bundle and Bundle to Binary. `Makevars` is used during conversion from Source to Binary and Bundle to Binary, but **not** during conversion from Source to Bundle.

```txt
Source -------> Bundle --------> Binary
       cleanup         Makevars
                       cleanup

Source ------------------------> Binary
                       Makevars
                       cleanup
```

This means, that `cleanup` needs not only to "clean up" but also to create `src/rust/vendor.tar.xz` if not yet existing. Furthermore, `Makevars` needs not only to be able to create `src/mdrb.dll` from `src/rust/vendor.tar.xz` (offline), but also to create `src/rust/vendor.tar.xz` if it does not exist (when installing from Source). I.e., both `Makevars` and `cleanup` need to be able to `src/rust/vendor.tar.xz`, when called from the Source package (where downloads are still allowed). Therefor, the functionality for creating `src/rust/vendor.tar.xz` is implemented in a seperate script `vendor.sh`, that is used by both `Makevars` and `cleanup`.


## Why do we need a cleanup script?

According to [R Packages](https://r-pkgs.org/structure.html#fig-package-files), an R package can exist in three major states:

1. *Source*: The raw form of the package, as obtained after a fresh `git clone` of the repository.

2. *Bundled*: A preprocessed version of the source package created using `R CMD build` or `devtools::build()`. It includes compiled vignettes and excludes files ignored by `.Rbuildignore`.

3. *Binary*: The installed version of the package, where all R files, data, and manuals are compiled into binary representations, and low-level source code is compiled into machine code.

Each state requires or forbids specific files:

| File                     | Source  | Bundle    | Binary    |
| ------------------------ | ------- | --------- | --------- |
| `src/rust/vendor.tar.xz` | ignored | required  | ignored   |
| `src/rust/vendor`        | ignored | forbidden | forbidden |
| `src/mdrb.dll`           | ignored | forbidden | required  |
| `downloading`            | allowed | forbidden | forbidden |

To handle these requirements, package scripts must ensure the correct files are present for each state. The `cleanup` script is invoked during transitions from Source to Bundle and Bundle to Binary. The `Makevars` file is used during transitions from Source to Binary and Bundle to Binary but **not** from Source to Bundle.

```txt
Source -------> Bundle --------> Binary
       cleanup         Makevars
                       cleanup

Source ------------------------> Binary
                       Makevars
                       cleanup
```

This means the `cleanup` script must not only remove unnecessary files but also create `src/rust/vendor.tar.xz` if it does not already exist. Similarly, `Makevars` must be able to:

1. Create `src/mdrb.dll` from `src/rust/vendor.tar.xz` (offline).
2. Create `src/rust/vendor.tar.xz` if it is missing (when installing from Source).

To avoid duplication, the functionality for creating `src/rust/vendor.tar.xz` is implemented in a separate script, `vendor.sh`, which is shared by both `Makevars` and `cleanup`.
