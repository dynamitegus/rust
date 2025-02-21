#!/usr/bin/env fish
argparse --name=my_function 'n/name=' -- $argv
or return


git remote add $_flag_name ./$_flag_name
git fetch $_flag_name --tags
git merge --allow-unrelated-histories $_flag_name/master
git remote remove $_flag_name