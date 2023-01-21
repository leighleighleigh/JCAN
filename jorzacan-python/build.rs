#[allow(unused_must_use)]

fn main() {
    // Set environment variable PYO3_NO_PYTHON
    // to disable the automatic detection of Python
    std::env::set_var("PYO3_NO_PYTHON", "1");
    pyo3_build_config::add_extension_module_link_args();
}
