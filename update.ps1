git pull origin master
git submodule update --init
git submodule foreach git pull origin master
git add bundle/
git commit -m "updating modules"
