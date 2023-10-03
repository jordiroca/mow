#!/usr/bin/env bash
#
# DESCRIPTION: center_haikus Center my haikus
#
# VERSION: 23.10.02
#
# AUTHOR: Jordi Roca
# CREATED: 2023/10/02
#
# INSTRUCCIONES:
#   chmod +x center_haikus.sh
#   ./center_haikus.sh < sample.txt > haikus.txt
#

ancho=80

calcula_padding() {
  local cadena="$1"
  local largo_cadena=${#cadena}
  local resto_ancho=$((ancho - largo_cadena))
  local padding=""
  
  if [ "$resto_ancho" -gt 0 ]; then
    local padding_length=$((resto_ancho / 2))
    for ((i = 0; i < padding_length; i++)); do
      padding+=" "
    done
  fi
  
  echo "$padding"
}

# Read lines from the input file, center them within 80 characters, and write to the output file
while IFS= read -r line; do
  padding=$(calcula_padding "$line")
  centrada="$padding$line$padding"
  centrada="${centrada:0:$ancho}"
  echo "$centrada"
done

