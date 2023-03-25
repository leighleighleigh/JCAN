#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# This script packages the latest wheel and c++ library
# into zipped files for upload to GitHub Releases page.

# Start by deleting a ./release/ directory if it exists
rm -rf "${SCRIPT_DIR}/release"

# Then create it
mkdir "${SCRIPT_DIR}/release"

# Used to use git tags for version, but now just use cargo version
GIT_TAG=$(cargo get --root jcan version)

# Get the python package version from the jcan_python Cargo.toml
# This requires that the setup.py is also updated with the same version number.
# This is the version number that will be used in the wheel filename
# e.g. jcan-0.1.5-cp38-abi3-manylinux2014_aarch64.whl 
# OLD VERSION READ THE setup.py FILE
# PYTHON_PACKAGE_VERSION=$(grep -oP '(?<=version=")[^"]*' "${SCRIPT_DIR}/jcan-python/setup.py")
# NEW VERSION USES CARGO-GET VERSION
PYTHON_PACKAGE_VERSION=$(cargo get --root jcan-python version)

# Copy the wheel(s) to the release directory
cp "${SCRIPT_DIR}/out/wheels/jcan-${PYTHON_PACKAGE_VERSION}-"*.whl "${SCRIPT_DIR}/release/"

# Rename the wheel(s) to use the GIT_TAG version
for file in "${SCRIPT_DIR}/release/jcan-${PYTHON_PACKAGE_VERSION}-"*.whl;
do
    echo "Renaming ${file} to ${file/jcan-${PYTHON_PACKAGE_VERSION}-/jcan-${GIT_TAG}-}"
    mv "${file}" "${file/jcan-${PYTHON_PACKAGE_VERSION}-/jcan-${GIT_TAG}-}"
done

# For each subdirectory of out/release/, representing a <target>/jcan combination
# we will copy the jcan library to the release directory with the name jcan_<target>
for dir in "${SCRIPT_DIR}/out/release/"*;
do
    # Get the target name from the directory name
    target_name=$(basename "${dir}")

    # Copy the jcan library to the release directory, and zip it
    FROM="${dir}"
    TO="${SCRIPT_DIR}/release/jcan-${GIT_TAG}-${target_name}"

    # Print from/to/
    echo "Copying ${FROM} to ${TO}"

    # Zip the files contained within `FROM` into `TO.zip`,
    # keeping the 'jcan' folder inside.
    # This requires moving to the FROM directory
    pushd "${FROM}" > /dev/null
    # Zip
    zip -rv "${TO}.zip" .
    # Move out 
    popd > /dev/null
done
