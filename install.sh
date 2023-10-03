#!/usr/bin/env bash
# -*- coding: utf-8 -*-
#
# DESCRIPTION: install Install mow, lmow and rmow utilities
#
# VERSION: 23.10.2
#
# AUTHOR: Jordi Roca
# CREATED: 2023/10/02
#
# GITHUB: https://github.com/jordiroca/
# WEBSITE: https://jordiroca.com
#
# LICENSE: See LICENSE file.
#

echo "Compiling mow, lmow and rmow utilities"
rustc mow.rs
if [ "$?" -eq 0 ]; then
    cp mow lmow
    cp mow rmow
    echo "OS may ask for sudo password to copy files to /usr/local/bin/"
    sudo cp -fv mow lmow rmow /usr/local/bin/
    echo
fi
