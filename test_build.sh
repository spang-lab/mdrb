#!/bin/sh

# Usage: sh test_build.sh --target=[all|bundle|binary] [--devtools=TRUE|FALSE]

# Default values
TARGET="bundle"
DEVTOOLS="TRUE"

# Parse arguments
for ARG in "$@"; do
  case "$ARG" in
    --target=all|--target=bundle|--target=binary)
      TARGET=$(echo "$ARG" | cut -d'=' -f2)
      ;;
    --devtools=FALSE|--devtools=false)
      DEVTOOLS="FALSE"
      ;;
    --devtools=TRUE|--devtools=true)
      DEVTOOLS="TRUE"
      ;;
    *)
      echo "Invalid argument: $ARG"
      echo "Usage: sh test_build.sh --target=[all|bundle|binary] [--devtools=TRUE|FALSE]"
      exit 1
      ;;
  esac
done

# Print parsed arguments
echo "Running with arguments:"
echo "--target=$TARGET"
echo "--devtools=$DEVTOOLS"
echo ""

# Test 1: R CMD build
if [ "$TARGET" = "bundle" ] && [ "$DEVTOOLS" = "FALSE" ]; then
  echo "Testing R CMD build"
  rm -rf tmp/*
  mkdir -p tmp
  (cd tmp && R CMD build ..)
  (cd tmp && tar -xzf *.tar.gz)
  test -f tmp/mdrb/src/rust/vendor.tar.xz && echo "Success" || echo "Failed (tmp/mdrb/src/rust/vendor.tar.xz is missing)"
  exit 0
fi

# Test 2: devtools::build()
if [ "$TARGET" = "bundle" ] && [ "$DEVTOOLS" = "TRUE" ]; then
  echo "Testing devtools::build()"
  rm -rf tmp/*
  mkdir -p tmp
  Rscript -e 'devtools::build(path = "tmp")'
  (cd tmp && tar -xzf *.tar.gz)
  test -f tmp/mdrb/src/rust/vendor.tar.xz && echo "Success" || echo "Failed (tmp/mdrb/src/rust/vendor.tar.xz is missing)"
  exit 0
fi

# Test 3: devtools::build(binary = TRUE)
if [ "$TARGET" = "binary" ]; then
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
  exit 0
fi

# Test all
if [ "$TARGET" = "all" ]; then
  echo "Running all tests"
  sh "$0" --target=bundle --devtools=FALSE
  sh "$0" --target=bundle --devtools=TRUE
  sh "$0" --target=binary
  exit 0
fi

# Invalid target
echo "Invalid target: $TARGET"
echo "Usage: sh test_build.sh --target=[all|bundle|binary] [--devtools=TRUE|FALSE]"
exit 1
