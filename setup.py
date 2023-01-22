from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension


setup(
    name="jorzacan_python",
    version="0.1.0",
    packages=find_namespace_packages(include=["jorzacan_python.*"]),
    zip_safe=False,
    rust_extensions=[
        RustExtension(
            "jorzacan_python.jorzacan_python",
            path="Cargo.toml",
            binding=Binding.PyO3,
            debug=False,
        )
    ],
)
