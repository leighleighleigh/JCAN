from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension


setup(
    name="jorzacan",
    version="0.1.1",
    packages=["jorzacan"],
    zip_safe=False,
    rust_extensions=[
        RustExtension(
            "jorzacan.jorzacan_python",
            path="Cargo.toml",
            binding=Binding.PyO3,
            py_limited_api='auto'
        )
    ],
    include_package_data=True,
)
