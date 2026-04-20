## MimIR EqSat

This repository contains the [equality saturation](https://en.wikipedia.org/wiki/E-graph#Equality_saturation) implementations for [MimIR](https://github.com/AnyDSL/MimIR).

## Provided Methods

There are two separate implementations in [egg](https://github.com/egraphs-good/egg) and [slotted-egraphs](https://github.com/memoryleak47/slotted-egraphs) that expose the following methods:

### Rewriting

```cpp
/**
 *  Rewrites an sexpr in `egg` format
 *
 *  sexpr:     a symbolic expr in `egg` format (emitted by the `mim` compiler via --output-sexpr)
 *  rulesets:  provides a list of identifiers to rulesets that should be used for rewriting (see src/mim_egg/rulesets)
 *  cost_fn:   provides a cost function that should be used for extraction (currently only AstSize and AstDepth)
 */
rust::Vec<RewriteResult> equality_saturate(std::string sexpr, rust::Vec<RuleSet> rulesets, CostFn cost_fn);
```

```cpp
/**
 *  Rewrites an sexpr in `slotted-egraphs` format
 *
 *  sexpr:     a symbolic expr in `slotted-egraphs` format (emitted by the `mim` compiler via --output-sexpr-slotted)
 *  rulesets:  provides a list of identifiers to rulesets that should be used for rewriting (see src/mim_slotted/rulesets)
 *  cost_fn:   provides a cost function that should be used for extraction (currently only AstSize)
 */
rust::Vec<RewriteResult> equality_saturate_slotted(std::string sexpr, rust::Vec<RuleSet> rulesets, CostFn cost_fn);
```

### Pretty-printing

```cpp
/**
 *  Pretty-prints an sexpr in `egg` format
 *
 *  sexpr:     a symbolic expr in `egg` format (emitted by the `mim` compiler via --output-sexpr)
 *  line_len:  the maximal line length after which the sexpr continues on a new line
 */
rust::String pretty(std::string sexpr, size_t line_len);
```

```cpp
/**
 *  Pretty-prints an sexpr in `slotted-egraphs` format
 *
 *  sexpr:     a symbolic expr in `slotted-egraphs` format (emitted by the `mim` compiler via --output-sexpr-slotted)
 *  line_len:  the maximal line length after which the sexpr continues on a new line
 */
rust::String pretty(std::string sexpr, size_t line_len);
```

## Installation

There are multiple methods of integrating this library into an existing C++ project. The following lists the simplest one.

**1. Add `mimir-eqsat` as a submodule to your project**

```bash
git submodule add https://github.com/ashiven/mimir-eqsat external/mimir-eqsat
cd external/mimir-eqsat
git checkout v1.0.0 # whichever version you require

cd ../..
git add external/mimir-eqsat
git commit -m "Add mimir-eqsat submodule"
```

**2. Add the following to your `CMakeLists.txt`**

```cmake
include(${PROJECT_SOURCE_DIR}/external/mimir-eqsat/dist/cmake/mimir-eqsat.cmake)
configure_file(
    "${PROJECT_SOURCE_DIR}/external/mimir-eqsat/dist/include/mimir_eqsat.h"
    "${CMAKE_BINARY_DIR}/include/rust/mimir_eqsat.h" # choose a path and name for the header
)
target_link_libraries(target PRIVATE mimir-eqsat)
```

## Updating

To update the submodule to a particular release, simply do the following:

```bash
cd external/mimir-eqsat
git fetch
git checkout v1.1.0 # whichever version you require

cd ../..
git add external/mimir-eqsat
git commit -m "Update mimir-eqsat submodule"
```
