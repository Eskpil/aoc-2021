#!/bin/sh

base_prod=~/Projects/aoc-2021/utils/base/prod.sh
base_dev=~/Projects/aoc-2021/utils/base/dev.sh

for d in *; do
    if [[ -d $d ]]; then
        echo $d
        if [ "$d" == "utils" ]; then 
                echo "Skipping"
        else
            prod=~/Projects/aoc-2021/$d/prod.sh       
            dev=~/Projects/aoc-2021/$d/dev.sh
            path=~/Projects/aoc-2021/$d

            cp $base_prod $prod
            cp $base_dev $dev

            sed -i "s/@@/$d/g" $prod
            sed -i "s/@@/$d/g" $dev

            chmox $prod
            chmox $dev

            echo $path
            bash "cd $path && cargo init"
        fi
    fi
done
