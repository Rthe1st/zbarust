# TODO

* Strip out build script logic for linking to prebuilt lib
  * Doesn't make sense once parts are rewritten in rust
  * Get rid of version check test
* replace c_void's with pointers to c-compatible rust types
* better test coverage of ffi bindings
* improve CI
  * Use caching (installing code coverage tool takes forever)
  * use 1 yaml file/reuse actions between the files
