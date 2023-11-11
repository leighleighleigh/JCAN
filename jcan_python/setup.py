from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension
from pathlib import Path
import toml

# The version of the python package will always match the version
# of the jcan-python cargo package.
# To do this, we read the version from the jcan-python/Cargo.toml file.
def get_version() -> str:
    version = {}
    # the Cargo.toml and setup.py file are in the same directory,
    # so we can use the __file__ variable to get the path to the Cargo.toml file.
    cargo_toml_path = Path(__file__).parent / "Cargo.toml"
    # Open the toml file and retrieve the [package].version value.
    with open(cargo_toml_path, "r") as f:
        cargo_toml = toml.load(f)
        version = cargo_toml["package"]["version"]

    return version

setup(
    name="jcan",
    version=get_version(),
    packages=["jcan"],
    zip_safe=False,
    rust_extensions=[
        RustExtension(
            "jcan.jcan_python",
            path="Cargo.toml",
            binding=Binding.PyO3,
            py_limited_api='auto'
        )
    ],
    include_package_data=True,
    # Requirements
    # install_requires=[
    #     "setuptools-rust",
    # ],
    # setup_requires=[
    #     "setuptools-rust",
    # ],
)
