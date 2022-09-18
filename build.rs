fn main() {
    let ac = autocfg::new();

    // Required for stabilization of `Vec::shrink_to()` and `IntoIterator` for arrays.
    ac.emit_rustc_version(1, 56);

    autocfg::rerun_path("build.rs");
}
