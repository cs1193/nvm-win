#!/bin/bash


if [ -z "$1" ]
then
    echo "Please provide version <major|minor|patch>"
else
    version = $1
    echo "Bumping to $version"
    cargo bump $version --git-tag

    echo "Adding files to git"
    git add .
    git commit -m "fix: dev: master: bumped $version version"
    git push -u origin master --tags

    echo "Publishing the package"
    cargo publish
fi