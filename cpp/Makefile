# This Makefile replaces the command 
# cbindgen --config ../cbindgen.toml --crate jorzacan --output ./cpp/jorzacan.h
# g++ -std=c++11 -I. -c jorzacan.cpp -o jorzacan.o
# You can achieve the same result by typing
# make

# START MAKEFILE
# Step 1 is to run cbindgen to generate the header file
# This needs to be re-run if the Rust code changes
# So the Makefile will run it every time
# Step 2 is to compile the C++ file
jorzacan: jorzacan.cpp
	# Generate the header file
	cd .. && cbindgen --config cbindgen.toml --crate jorzacan --output ./cpp/jorzacan.h
	# Compile the library 
	g++ -std=c++11 -I. -c jorzacan.cpp -o jorzacan.o
	# Compile the main.cpp example
	g++ -std=c++11 -I. -c main.cpp -o main.o


# END MAKEFILE

