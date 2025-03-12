#' @noRd
#' @examples
#' cat(make_r_docs("Spectrum"), sep = "\n")
#' cat(make_r_docs("Deconvolution"), sep = "\n")
make_r_docs <- function(name) {
    class_env <- get(name)
    usage <- capture.output(print(ls.str(class_env)))
    usage <- gsub("^", "x$", usage)
    usage <- gsub(" : function ", "", usage)
    idx_new <- which(grepl("^x\\$new\\(", usage))
    if (length(idx_new) > 0) {
        call <- sprintf("x <- %s$new(", name)
        usage <- gsub("^x\\$new\\(", call, usage)
        usage <- c(usage[idx_new], usage[-idx_new])
    } else {
        call <- paste("# Assuming x is an object of class", name)
        usage <- c(call, usage)
    }
    c(
        paste("@export"),
        paste(""),
        paste("@title"),
        paste(name, "Class"),
        paste(""),
        paste("@description"),
        paste("Environment containing methods for the", name, "class."),
        paste(""),
        paste("@usage"),
        paste(usage),
        paste(""),
        paste("@details"),
        paste("For more information on the methods, see the Rust documentation at"),
        paste("<https://github.com/SombkeMaximilian/metabodecon-rust>."),
        paste("")
    )
}
