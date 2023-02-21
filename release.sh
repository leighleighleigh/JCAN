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
# to the end of the tag. This ensures the build environment is tidy.
GIT_DESCRIBED_TAG=$(git describe --tags --match 'v*' --dirty)

# Get the current git tag
GIT_TAG=$(git tag -l | tail -n1)

# If the current git tag is not a release tag, exit
if [[ "${GIT_TAG}" != "v"* ]];
then
    echo "Current git tag is not a release tag, exiting"
    exit 1
fi

# Remove the "v" from the git tag, to match Rust convention
GIT_TAG=${GIT_TAG:1}

# Copy the wheel(s) to the release directory
cp "${SCRIPT_DIR}/out/wheels/jcan-${GIT_TAG}-"*.whl "${SCRIPT_DIR}/release/"

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
