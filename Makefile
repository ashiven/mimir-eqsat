.PHONY: build compile build-lit test bootstrap

# NOTE: use flag "-G Ninja" to generate compile_commands.json on Windows
build:
	cmake -S . -B build \
		-DBUILD_TESTING=ON \
		-DMIM_BUILD_EXAMPLES=ON \
		-DCMAKE_EXPORT_COMPILE_COMMANDS=1 \
		-DCMAKE_INSTALL_PREFIX="~\OneDrive\Dokumente\Projects\C++\MimIR\build\install"


compile:
	cmake --build ../../build -j 8 --target install -- /verbosity:quiet

build-lit:
	cmake --build ../../build -j 8 --target lit -- /verbosity:quiet

test:
	MIM_PLUGIN_PATH="~/OneDrive/Dokumente/Projects/C++/MimIR/extra/eqsat/lit" && "../../build/install/bin/mim.exe" ./lit/$(TEST) -o -

bootstrap:
	../../build/bin/Debug/mim.exe \
  ./eqsat.mim \
  -P ../../src/mim/plug \
  --bootstrap \
  --output-h ../../build/include/mim/plug/eqsat/autogen.h \
	--output-md ../../build/docs/plug/eqsat.md

# TODO: test-all command to run all tests in ./lit
