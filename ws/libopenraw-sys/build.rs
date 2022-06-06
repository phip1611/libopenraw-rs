use std::process::Command;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // IMPORTANT to have this before the "Command::new". Otherwise, it get's
    // executed every time.
    //
    // PS: It may happen that Rust executes this whole file twice:
    //     1) initially when the submodule doesn't exist yet
    //     2) another time after the file has changed after the checkout
    //
    //     After that, it returns early from builds if nothing changed.
    println!("cargo:rerun-if-changed=../../libopenraw/include/libopenraw/libopenraw.h");

    // Build C-library "libopenraw"
    let _outpu_ = Command::new("./build_original_lib.sh").output().expect(
        "
            Must build original lib/C-project successfully.
            Check './build_original_lib.sh' manually to see why it fails.
            ",
    );
    // printing the output doesn't work
    // eprintln!("{:#?}", output);

    // Tell cargo to look for shared libraries in the specified directory
    // "outdir" comes from the configure-script for the C-lib
    println!("cargo:rustc-link-search=./libopenraw-outdir/lib");

    // Tell cargo to tell rustc to link the shared library.
    println!("cargo:rustc-link-lib=openraw");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // .clang_arg("-v")
        // The input header we would like to generate
        // bindings for.
        .header("../../libopenraw/include/libopenraw/libopenraw.h")
        // help to find the other header files
        .clang_arg("-I../../libopenraw/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
