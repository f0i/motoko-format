#!/usr/bin/env bash

set -eu -o pipefail

file="./repos"

while read -a repo;
do
    name=${repo[0]}
    remote=${repo[1]}

    if [[ -d "$name" ]]
    then
        echo "Updating $name"
        cd "$name"
        git stash; git pull
        cd -
    else
        echo "get ${name} from ${remote}"
        git clone "$remote" "$name"
    fi

done < "$file"

time dprint fmt
