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

If you want to install this plugin, simply follow the instructions below:

**1. Clone the `mimir` repository if you haven't yet**

```bash
git clone --recursive https://github.com/mimir/mimir.git
```

**2. Clone the `eqsat` repository into `mimir/extra`**

```bash
cd mimir/extra
git clone https://github.com/ashiven/eqsat.git
cd ..
```

**3. Ensure that Rust and Cargo are installed**

```bash
curl https://sh.rustup.rs -sSf | sh
```

**4. Build the project according to the [instructions](https://mimir.github.io/index.html#autotoc_md92)**

```bash
cmake -S . -B build -DBUILD_TESTING=ON -DMIM_BUILD_EXAMPLES=ON
cmake --build build -j$(nproc)
```
