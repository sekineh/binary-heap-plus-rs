fn main() {
    let ac = autocfg::new();

    // Required for stabilization of `unsafe_op_in_unsafe_fn` lint.
    ac.emit_rustc_version(1, 52);

    // Required for stabilization of `Vec::shrink_to()`.
    ac.emit_rustc_version(1, 56);

    autocfg::rerun_path("build.rs");
}
