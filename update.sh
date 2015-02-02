#!/bin/bash

if ! git diff-index --quiet HEAD --; then
    echo "Stash changes before upgrading dependencies"
    exit 1
fi

while read repository; do
    dir=`basename $repository | sed 's/.git//g'`
    if [[ -d bundle/$dir ]] ; then
        git subtree pull --squash --prefix bundle/$dir $repository master
    else 
        git subtree add --squash --prefix bundle/$dir $repository master
    fi

    if ! git diff-index --quiet HEAD --; then
        git add .
        git commit --amend --no-edit
    fi   
done < modules
