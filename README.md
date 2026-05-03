## MimIR EqSat

This repository contains the [equality saturation](https://en.wikipedia.org/wiki/E-graph#Equality_saturation) implementations for [MimIR](https://github.com/mimir/mimir).

## Provided Methods

There are two separate implementations in [egg](https://github.com/egraphs-good/egg) and [slotted-egraphs](https://github.com/memoryleak47/slotted-egraphs) that expose the following methods:

### Rewriting

```cpp
/**
 *  Rewrites an sexpr in `egg` format
 *
 *  sexpr:     a symbolic expr in `egg` format (emitted by the `mim` compiler via `--output-sexpr`)
 *  rulesets:  provides a list of identifiers to rulesets that should be used for rewriting (see src/mim_egg/rulesets)
 *  cost_fn:   provides a cost function that should be used for extraction (currently only AstSize and AstDepth)
 */
rust::Vec<RecExprFFI> eqsat_egg(std::string sexpr, rust::Vec<RuleSet> rulesets, CostFn cost_fn);
```

```cpp
/**
 *  Rewrites an sexpr in `slotted-egraphs` format
 *
 *  sexpr:     a symbolic expr in `slotted-egraphs` format (emitted by the `mim` compiler via `--slotted --output-sexpr`)
 *  rulesets:  provides a list of identifiers to rulesets that should be used for rewriting (see src/mim_slotted/rulesets)
 *  cost_fn:   provides a cost function that should be used for extraction (currently only AstSize)
 */
rust::Vec<RecExprFFI> eqsat_slotted(std::string sexpr, rust::Vec<RuleSet> rulesets, CostFn cost_fn);
```

### Pretty-printing

```cpp
/**
 *  Pretty-prints an sexpr in `egg` format
 *
 *  sexpr:     a symbolic expr in `egg` format (emitted by the `mim` compiler via `--output-sexpr`)
 *  line_len:  the maximal line length after which the sexpr continues on a new line
 */
rust::String pretty_egg(std::string sexpr, size_t line_len);
```

```cpp
/**
 *  Pretty-prints an sexpr in `slotted-egraphs` format
 *
 *  sexpr:     a symbolic expr in `slotted-egraphs` format (emitted by the `mim` compiler via `--slotted --output-sexpr`)
 *  line_len:  the maximal line length after which the sexpr continues on a new line
 */
rust::String pretty_slotted(std::string sexpr, size_t line_len);
```

```cpp
/**
 *  Pretty-prints an sexpr represented by a Vec<RecExprFFI>
 *
 *  sexprs:    a vector of symbolic expressions in RecExprFFI format (the result of equality saturation)
 *  line_len:  the maximal line length after which the sexpr continues on a new line
 */
rust::String pretty_ffi(rust::Vec<RecExprFFI> sexprs, size_t line_len);
```

## Installation

There are multiple methods of integrating this library into an existing C++ project. The following lists the simplest one.

**1. Add `eqsat` as a submodule to your project**

```bash
git submodule add https://github.com/ashiven/eqsat external/eqsat
git add external/eqsat
git commit -m "Add eqsat submodule"
```

**2. Add the following to your `CMakeLists.txt`**

```cmake
include(${PROJECT_SOURCE_DIR}/external/eqsat/dist/cmake/eqsat-rs.cmake)
configure_file(
    "${PROJECT_SOURCE_DIR}/external/eqsat/dist/include/eqsat_rs.h"
    "${CMAKE_BINARY_DIR}/include/rust/eqsat_rs.h" # choose a path and name for the header
)
target_link_libraries(target PRIVATE eqsat-rs)
```

## Updating

To update the submodule to a particular release, simply do the following:

```bash
cd external/eqsat
git pull
cd ../..
git add external/eqsat
git commit -m "Update eqsat submodule"
```
