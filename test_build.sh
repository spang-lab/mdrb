#!/bin/sh

# Usage: sh test_build.sh

# echo "Testing R CMD build"
# rm -rf tmp/*
# mkdir -p tmp
# (cd tmp && R CMD build ..)
# (cd tmp && tar -xzf *.tar.gz)
# test -f tmp/mdrb/src/rust/vendor.tar.xz && echo "Success" || echo "Failed (tmp/mdrb/src/rust/vendor.tar.xz is missing)"

# echo "Testing devtools::build()"
# rm -rf tmp/*
# mkdir -p tmp
# Rscript -e 'devtools::build(path = "tmp")'
# (cd tmp && tar -xzf *.tar.gz)
# test -f tmp/mdrb/src/rust/vendor.tar.xz && echo "Success" || echo "Failed (tmp/mdrb/src/rust/vendor.tar.xz is missing)"

echo "Testing devtools::build(binary = TRUE)"
rm -rf tmp/*
mkdir -p tmp
Rscript -e 'devtools::build(binary = TRUE, path = "tmp")'
if [ "$(uname)" = "Linux" ] || [ "$(uname)" = "Darwin" ]; then
  # Unix-like OS (Linux/Mac)
  (cd tmp && tar -xzf *.tar.gz)
  test -f tmp/mdrb/libs/mdrb.so && echo "Success" || echo "Failed (tmp/mdrb/libs/x64/mdrb.so is missing)"
else
  # Windows
  (cd tmp && unzip *.zip)
  test -f tmp/mdrb/libs/x64/mdrb.dll && echo "Success" || echo "Failed (tmp/mdrb/libs/x64/mdrb.dll is missing)"
fi
test -f tmp/mdrb/src/rust/vendor.tar.xz && echo "Failed (tmp/mdrb/src/rust/vendor.tar.xz is still existing)" || echo "Success"
