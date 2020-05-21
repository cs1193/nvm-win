#!/bin/bash

version = $1

if [ -z "$version" ]
then
    echo "Bumping to $version"
    cargo bump $version --git-tag

    echo "Adding files to git"
    git add .
    git commit -m "fix: dev: master: bumped $version version"
    git push -u origin master --tags

    echo "Publishing the package"
    cargo publish
else 
    echo "Please provide version <major|minor|patch>"
fi