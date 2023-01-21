#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# This script packages the latest wheel and c++ library
# into zipped files for upload to GitHub Releases page.

# Start by deleting a ./release/ directory if it exists
rm -rf "${SCRIPT_DIR}/release"

# Then create it
mkdir "${SCRIPT_DIR}/release"

# Get the current git tag
GIT_TAG=$(git describe --tags)

# If the current git tag is not a release tag, exit
if [[ "${GIT_TAG}" != "v"* ]];
then
    echo "Current git tag is not a release tag, exiting"
    exit 1
fi

# Remove the "v" from the git tag, to match Rust convention
GIT_TAG=${GIT_TAG:1}

# Copy the wheel(s) to the release directory
cp "${SCRIPT_DIR}/out/wheels/jorzacan_python-${GIT_TAG}-"*.whl "${SCRIPT_DIR}/release/"

# For each subdirectory of out/release/, representing a <target>/jorzacan combination
# we will copy the jorzacan library to the release directory with the name jorzacan_<target>
for dir in "${SCRIPT_DIR}/out/release/"*;
do
    # Get the target name from the directory name
    target_name=$(basename "${dir}")

    # Copy the jorzacan library to the release directory, and zip it
    FROM="${dir}"
    TO="${SCRIPT_DIR}/release/jorzacan-${GIT_TAG}-${target_name}"

    # Print from/to/
    echo "Copying ${FROM} to ${TO}"

    # Zip the files contained within `FROM` into `TO.zip`,
    # keeping the 'jorzacan' folder inside.
    # This requires moving to the FROM directory
    pushd "${FROM}" > /dev/null
    # Zip
    zip -rv "${TO}.zip" .
    # Move out 
    popd > /dev/null
done