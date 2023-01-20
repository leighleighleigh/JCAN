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
jorzacan: main.cpp
	# OLD, see build.rs
	# Generate the header file
	# cd .. && cbindgen --config cbindgen.toml --crate jorzacan --output ./target/jorzacan.h
	# NEW
	# Copy to ./cpp/jorzacan_generated.h
	cp ../target/jorzacan.hpp jorzacan_generated.hpp
	g++ -std=c++11 -I. -c jorzacan.h -o jorzacan.o
	# Compile the main.cpp example
	g++ -std=c++11 -I. main.cpp -o main.o


# END MAKEFILE

