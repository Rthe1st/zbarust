# to generate FFI bindings for a new C file
# 1) `cargo install bindgen`
# (we don't use bindgen from build.sh because the generate .rs will be edited eventualy to replace the C code completely)
# 2) make sure zbar/include/config.h exists by:
# ```autoreconf -ivf
# ./configure --without-gtk --without-qt --without-java --without-imagemagick --with-jpeg=no --with-x=no --with-video=no```
# 3) Edit the C file to make any functions you need to call not static (inline)
# 4) maybe edit the white-listed types to exclude stuff you do/n't need
bindgen --raw-line 'use libc;' --whitelist-function '_?zbar.*' --ctypes-prefix 'libc' --generate-inline-functions -o "src/$1.rs" "zbar/zbar/$1.h" -- --include "zbar/include/config.h" -fno-inline-functions
