#!/bin/bash

while read repository; do
    dir=`basename $repository | sed 's/.git//g'`
    if [[ -d bundle/$dir ]] ; then
        git subtree pull --squash --prefix bundle/$dir $repository master
    else 
        git subtree add --squash --prefix bundle/$dir $repository master
    fi
done < modules
