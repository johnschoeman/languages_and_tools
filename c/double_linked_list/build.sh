# build.sh

rm -rf build
mkdir build

# gcc main.c -o build/main
gcc -g3 -O0 main.c -o build/main
