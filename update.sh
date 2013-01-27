#!/bin/bash
git pull origin master
env GIT_SSL_NO_VERIFY=true git submodule update --init
env GIT_SSL_NO_VERIFY=true git submodule foreach git pull origin master
git add bundle/
git commit -m "updating modules"
git push origin master
