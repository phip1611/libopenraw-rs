#!/bin/sh

set -e

#########################################################################
# nice "hack" which make the script work, even if not executed from "./"
DIR=$(dirname "$(realpath "$0")")
cd "$DIR" || exit
#########################################################################

cd ../../ || exit
git submodule update --init
cd libopenraw || exit
./autogen.sh
./configure --prefix $(pwd)-outdir
cd lib || exit
make -j 8
make install
