#!/bin/sh
mkdir -p tmp
(cd tmp && R CMD build ..)
