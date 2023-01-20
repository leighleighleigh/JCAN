# Builds demo.cc with the jorzacan/main.rs.h and rust/cxx.h headers
# and links it with the libjorzacan.a library.
# The resulting executable is called demo.

# The main make target
all: demo

# The demo executable
demo: src/demo.cc
	g++ -std=c++11 -I ./include -o demo src/demo.cc