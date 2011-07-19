#!/bin/bash
GIT_SSL_NO_VERIFY=true
git pull 
git submodule foreach git pull origin master
