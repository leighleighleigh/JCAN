#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# This script packages the latest wheel and c++ library
# into zipped files for upload to GitHub Releases page.

# Start by deleting a ./release/ directory if it exists
rm -rf "${SCRIPT_DIR}/release"

# Then create it
mkdir "${SCRIPT_DIR}/release"

# Ask git to describe the current tag
# This will be the latest annotated tag (such as v0.1.6)
# Or it will be a combination of tag+commit-hash 
# if the current commit is not tagged. (such as v0.1.6-1-gf3c5c5c)
# If the working tree is dirty (uncommited changes), it will append -dirty
# to the end of the tag. This ensures the build artifacts are clearly labelled.
GIT_DESCRIBED_TAG=$(git describe --tags --match 'v*' --dirty)

# Since python wheels use the hyphen '-' to separate the version number, we need to 
# replace the GIT_DESCRIBED_TAG hypens with underscores
# e.g. v0.1.6-1-gf3c5c5c -> v0.1.6_1_gf3c5c5c
GIT_DESCRIBED_TAG=${GIT_DESCRIBED_TAG//-/_}

# Get the latest annotated tag that starts with 'v'
# e.g v0.1.5
GIT_LATEST_TAG=$(git tag -l 'v*' | tail -n1)

# If the argument --latest-tag is passed, use the latest annotated tag
# instead of the git describe tag.
if [[ "$1" == "--latest-tag" ]];
then
    GIT_TAG=${GIT_LATEST_TAG}
else
    GIT_TAG=${GIT_DESCRIBED_TAG}
fi

# Check the tag is not empty, else exit
if [[ -z "${GIT_TAG}" ]];
then
    echo "Error: Could not get git tag"
    exit 1
else
    echo "Release tag: ${GIT_TAG}"
fi

# Remove leading 'v' from the GIT_TAG
# e.g. v0.1.5 -> 0.1.5
GIT_TAG=${GIT_TAG#v}

# Get the python package version from the setup.py file
# This is the version number that will be used in the wheel filename
# e.g. jcan-0.1.5-cp38-abi3-manylinux2014_aarch64.whl 
PYTHON_PACKAGE_VERSION=$(grep -oP '(?<=version=")[^"]*' "${SCRIPT_DIR}/jcan-python/setup.py")

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
