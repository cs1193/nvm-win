#!/bin/bash

version = $1

cargo bump $1 --git-tag
git add .
git commit -m "fix: dev: master: bumped version"
git push -u origin master --tags