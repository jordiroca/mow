#!/usr/bin/env bash
# -*- coding: utf-8 -*-
#
# DESCRIPTION: run_me Example usage of mow, rmow and lmow utilities
#
# VERSION: 23.10.2
#
# AUTHOR: Jordi Roca
# CREATED: 2023/10/02
#
# LICENSE: See LICENSE file.
#


chmod +x *.sh
./install.sh
./center_haikus.sh < sample.txt > haikus.txt

echo "Left mowing"
echo "============="
cat haikus.txt | ./lmow | tr " " "_"
echo -e "\n\nRight mowing"
echo "=============="
cat haikus.txt | ./rmow | tr " " "_"
echo -e "\n\nMowing"
echo "========"
cat haikus.txt | ./mow | tr " " "_"
echo
