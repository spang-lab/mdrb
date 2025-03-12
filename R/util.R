#' @noRd
#' @example cat(document_details("Spectrum"))
document_details <- function(name) {
    class_env <- get(name)
    members <- capture.output(print(ls.str(class_env)))
    members <- paste(members, collapse = "\n")
    members <- gsub(":", "=", members)
    doc <- paste(sep = "\n",
        paste("The following methods are available for", name, "objects:"),
        paste(""),
        paste("```R"),
        paste(members),
        paste("```"),
        paste(""),
        paste("For more information on the methods, see the Rust documentation at"),
        paste("<https://github.com/SombkeMaximilian/metabodecon-rust>.")
    )
    doc
}

