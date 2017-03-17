# Minimal, Complete, and Verifiable example

This is a minimal, complete and verifiable example exposing a problem with Rust's cargo nightly and Rust's tensorflow bindings.

## Actual Problem

The [TensorFlow Rust binding](https://github.com/tensorflow/rust) crate contains an low-level FFI crate named `tensorflow-sys`, exposing TensorFlow's unsage C API in Rust.

Running the tests on this crate using Rust stable (1.16 at the time of writing) works:
```
$ git clone https://github.com/tensorflow/rust.git tensorflow_rust.git
$ cd tensorflow_rust.git/tensorflow-sys
$ cargo test
cargo test
   Compiling regex-syntax v0.3.9
   Compiling winapi v0.2.8
   Compiling utf8-ranges v0.1.3
   Compiling winapi-build v0.1.1
   Compiling kernel32-sys v0.2.2
   Compiling libc v0.2.21
   Compiling lazy_static v0.2.4
   Compiling pkg-config v0.3.9
   Compiling gcc v0.3.43
   Compiling thread-id v2.0.0
   Compiling memchr v0.1.11
   Compiling filetime v0.1.10
   Compiling thread_local v0.2.7
   Compiling xattr v0.1.11
   Compiling aho-corasick v0.5.3
   Compiling tar v0.4.10
   Compiling curl-sys v0.3.10
   Compiling miniz-sys v0.1.9
   Compiling flate2 v0.2.17
   Compiling libz-sys v1.0.13
   Compiling regex v0.1.80
   Compiling curl v0.4.6
   Compiling semver-parser v0.6.2
   Compiling semver v0.5.1
   Compiling tensorflow-sys v0.7.0 (file://${HOME}/tensorflow_rust.git/tensorflow-sys)
    Finished debug [unoptimized + debuginfo] target(s) in 18.74 secs
     Running ${HOME}/tensorflow_rust.git/target/debug/deps/lib-9a41e386c0fcf161

running 1 test
test linkage ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running ${HOME}/tensorflow_rust.git/target/debug/deps/tensorflow_sys-f354f5d8ed627f89

running 4 tests
test bindgen_test_layout_TF_AttrMetadata ... ok
test bindgen_test_layout_TF_Buffer ... ok
test bindgen_test_layout_TF_Input ... ok
test bindgen_test_layout_TF_Output ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests tensorflow-sys

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Running the test under Rust nightly fails though:
```
$ cargo clean
$ cargo +nightly test
   Compiling winapi v0.2.8
   Compiling pkg-config v0.3.9
   Compiling utf8-ranges v0.1.3
   Compiling libc v0.2.21
   Compiling regex-syntax v0.3.9
   Compiling gcc v0.3.43
   Compiling memchr v0.1.11
   Compiling xattr v0.1.11
   Compiling aho-corasick v0.5.3
   Compiling filetime v0.1.10
   Compiling tar v0.4.10
   Compiling winapi-build v0.1.1
   Compiling kernel32-sys v0.2.2
   Compiling libz-sys v1.0.13
   Compiling miniz-sys v0.1.9
   Compiling curl-sys v0.3.10
   Compiling thread-id v2.0.0
   Compiling thread_local v0.2.7
   Compiling flate2 v0.2.17
   Compiling lazy_static v0.2.4
   Compiling regex v0.1.80
   Compiling curl v0.4.6
   Compiling semver-parser v0.6.2
   Compiling semver v0.5.1
   Compiling tensorflow-sys v0.7.0 (file://${HOME}/tensorflow_rust.git/tensorflow-sys)
    Finished dev [unoptimized + debuginfo] target(s) in 20.15 secs
     Running ${HOME}/tensorflow_rust.git/target/debug/deps/lib-9ad1f1a1d018a241
dyld: Library not loaded: bazel-out/local-opt/bin/tensorflow/libtensorflow.so
  Referenced from: ${HOME}/tensorflow_rust.git/target/debug/deps/lib-9ad1f1a1d018a241
  Reason: image not found
error: process didn't exit successfully: `${HOME}/tensorflow_rust.git/target/debug/deps/lib-9ad1f1a1d018a241` (signal: 6, SIGABRT: process abort signal)

To learn more, run the command again with --verbose.
```

This problem was reported on [tensorflow/rust's issue tracker](https://github.com/tensorflow/rust/issues/71).


## Investigation

This repository is meant to provide a minimal, complete and verifiable example of problem.


## Usage

Use [`rustup`](https://rustup.rs/) to install Rust stable and nightly:
* 1.16
* 1.17.0-nightly (b1e31766d 2017-03-03)


Running the tests on stable works fine:
```sh
$ cargo clean
$ cargo +stable test -vv --lib
       Fresh gcc v0.3.45
       Fresh pkg-config v0.3.9
       Fresh libc v0.2.21
       Fresh filetime v0.1.10
       Fresh xattr v0.1.11
       Fresh tar v0.4.10
       Fresh miniz-sys v0.1.9
       Fresh flate2 v0.2.17
       Fresh libz-sys v1.0.13
       Fresh curl-sys v0.3.10
       Fresh curl v0.4.6
   Compiling tfsys v0.1.0 (file://${HOME}/tfsys.git)
     Running `rustc --crate-name tfsys src/lib.rs -g --test -C metadata=bad6671e460bf032 -C extra-filename=-bad6671e460bf032 --out-dir ${HOME}/tfsys.git/target/debug/deps --emit=dep-info,link -L dependency=${HOME}/tfsys.git/target/debug/deps -L ${HOME}/tfsys.git/target/libtensorflow-cpu-darwin-x86_64-1.0.0/lib -l dylib=tensorflow`
    Finished debug [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `${HOME}/tfsys.git/target/debug/deps/tfsys-bad6671e460bf032`

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

But not on nightly:
```sh
$ cargo clean
$ cargo +nightly test -vv --lib
       Fresh libc v0.2.21
       Fresh gcc v0.3.45
       Fresh pkg-config v0.3.9
       Fresh xattr v0.1.11
       Fresh filetime v0.1.10
       Fresh tar v0.4.10
       Fresh miniz-sys v0.1.9
       Fresh libz-sys v1.0.13
       Fresh flate2 v0.2.17
       Fresh curl-sys v0.3.10
       Fresh curl v0.4.6
   Compiling tfsys v0.1.0 (file://${HOME}/tfsys.git)
     Running `rustc --crate-name tfsys src/lib.rs --emit=dep-info,link -C debuginfo=2 --test -C metadata=0d818779fed6bac8 -C extra-filename=-0d818779fed6bac8 --out-dir ${HOME}/tfsys.git/target/debug/deps -L dependency=${HOME}/tfsys.git/target/debug/deps -L ${HOME}/tfsys.git/target/libtensorflow-cpu-darwin-x86_64-1.0.0/lib -l dylib=tensorflow`
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `${HOME}/tfsys.git/target/debug/deps/tfsys-0d818779fed6bac8`
dyld: Library not loaded: bazel-out/local-opt/bin/tensorflow/libtensorflow.so
  Referenced from: ${HOME}/tfsys.git/target/debug/deps/tfsys-0d818779fed6bac8
  Reason: image not found
error: process didn't exit successfully: `${HOME}/tfsys.git/target/debug/deps/tfsys-0d818779fed6bac8` (signal: 6, SIGABRT: process abort signal)

Caused by:
  process didn't exit successfully: `${HOME}/tfsys.git/target/debug/deps/tfsys-0d818779fed6bac8` (signal: 6, SIGABRT: process abort signal)
```
