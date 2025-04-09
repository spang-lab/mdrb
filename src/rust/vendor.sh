#!/bin/sh
export PATH="$HOME/.cargo/bin:$PATH"

cat <<EOF
Starting vendoring of Rust dependencies

If you are installing from CRAN or GitHub, the file vendor.tar.xz
should already exist, and its creation will be skipped.

If you are a package developer installing from the raw source package for the
first time, the file vendor.tar.xz will be created automatically using
'cargo vendor' and 'tar'.

If you are a package developer installing from the raw source package for the
n'th time (n > 1), the file vendor.tar.xz should already exist, and
its creation will be skipped. If you have updated any files in the src folder
and want to force the recreation of vendor.tar.xz, manually remove
vendor and vendor.tar.xz, then restart the installation or
build process.

EOF

if test -f "vendor.tar.xz"
then
    echo "Creation of vendor.tar.xz skipped, because it exists already."
else
    if test -d "vendor"
    then
        echo "Creation of vendor skipped, because it exists already."
    else
        echo "Calling cargo vendor"
        cargo vendor
    fi
    echo "Calling tar -cJf vendor.tar.xz vendor"
    tar -cJf vendor.tar.xz vendor
fi

echo ""
echo "Finished vendoring of Rust dependencies"
echo ""
