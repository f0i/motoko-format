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
        git diff
        cd -
    else
        echo "repo ${name} from ${remote} does not exist locally"
    fi

done < "$file"
