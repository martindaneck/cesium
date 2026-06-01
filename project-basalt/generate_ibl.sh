#!/bin/bash
# run this script from the root directory
# USAGE: ./generate_ibl.sh <filename_without_extension>
# be careful not to overwrite any files

mkdir temp && cd temp

echo "Executing: cmgen --type=equirect --format=hdr --size=32 --ibl-samples=4096 --ibl-irradiance irradiance.hdr ../$1.hdr"

cmgen --type=equirect --format=hdr --size=32 --ibl-samples=4096 --ibl-irradiance irradiance.hdr ../$1.hdr

echo "Executing: cmgen --format=png --size=512 --ibl-dfg brdf_lut.png ../$1.hdr"

cmgen --format=png --size=512 --ibl-dfg brdf_lut.png ../$1.hdr

echo "Executing: cmgen --type=cubemap --format=dds --size=1024 --ibl-samples=8192 --ibl-ld prefiltered.dds ../$1.hdr"

cmgen --type=cubemap --format=dds --size=1024 --ibl-samples=8192 --ibl-ld prefiltered.dds ../$1.hdr

mkdir $1

# cleaning up the mess from cmgen
mv ../$1.hdr $1/environment.hdr # this is the original equirectangular
mv irradiance.hdr/$1/irradiance.hdr $1/irradiance.hdr
mv brdf_lut.png $1/brdf_lut.png
mv prefiltered.dds/$1 $1/prefiltered

# move it to ibl directory and clean up
cd ..
mv temp/$1 assets/ibl/$1
rm -rf temp