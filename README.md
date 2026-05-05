<p align="center">
  <h2 align="center">eqsat</h2>
</p>

<p align="center">
  <b>Equality Saturation</b> in <b>MimIR</b>
</p>

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/ashiven/eqsat)](https://github.com/ashiven/eqsat/releases)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/ashiven/eqsat)](https://github.com/ashiven/eqsat/issues)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/ashiven/eqsat)](https://github.com/ashiven/eqsat/pulls)

</div>

## About 

**Equality Saturation** is a compiler optimization technique that utilizes [E-Graphs](https://en.wikipedia.org/wiki/E-graph#Equality_saturation) to represent every possible way of equivalently rewriting a program and find the most optimal version of it, thereby solving the [Phase-Ordering Problem](https://www2.imm.dtu.dk/pubdb/edoc/imm5406.pdf).
This repository contains **Equality Saturation** implementations in [egg](https://github.com/egraphs-good/egg) and [slotted-egraphs](https://github.com/memoryleak47/slotted-egraphs) as a plugin for the [DSL](https://en.wikipedia.org/wiki/Domain-specific_language) compiler intermediate representation [MimIR](https://github.com/mimir/mimir).

## Usage

You may use this plugin through the **MimIR** C++ API or its textual representation **Mim**.
Consider the following lightweight examples to get you started.

### Option 1: C++ API

```cpp
#include <fstream>
#include <mim/driver.h>
#include <mim/ast/parser.h>
#include <mim/pass/optimize.h>
#include <mim/util/sys.h>
#include <mim/plug/eqsat/eqsat.h>

using namespace mim;
using namespace mim::plug;

int main(int, char**) {
    try {
        Driver driver;
        auto& w = driver.world();
        driver.log().set(&std::cerr).set(Log::Level::Debug);
        ast::load_plugins(w, View<std::string>{"compile", "core", "opt", "eqsat"});

        // rule foo (x: Nat) = %core.nat.add (x, 0) => x;
        auto foo = w.mut_rule(w.type_nat())->set("foo");
        foo->var()->set("x");
        auto lhs = w.call(core::nat::add, w.tuple(foo->var(), lit_nat(0)))
        auto rhs = foo->var();
        foo.set_lhs(lhs);
        foo.set_rhs(rhs);
        foo.set_guard(w.lit_tt());

        // Use the provided macros to quickly define config functions
        EQSAT_IMPL(w, eqsat::slotted);
        EQSAT_COST_FUN(w, eqsat::AstSize);
        EQSAT_RULESETS(w, {eqsat::standard});
        EQSAT_RULES(w, {foo});

        // fun extern main(x: Nat): Nat =
        //     return %core.nat.add (x, 0);
        auto main   = w.mut_fun({w.type_nat()}, {w.type_nat()})->set("main");
        auto x = main->var(2, 0)->set("x");
        auto ret               = main->var(2, 1);
        main->app(false, ret, x);
        main->externalize();

        optimize(w);
        std::ofstream ofs("eqsat.ll");
        driver.backend("ll")(w, ofs);
        ofs.close();

        sys::system("clang eqsat.ll -o eqsat -Wno-override-module");
        outln("exit code: {}", sys::system("./eqsat"));
    } catch (const std::exception& e) {
        errln("{}", e.what());
        return EXIT_FAILURE;
    } catch (...) {
        errln("error: unknown exception");
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
```

### Option 2: Mim

```
plugin core;
plugin eqsat;

// Here you can specify whether the plugin should use its `egg` or `slotted-egraphs` backend.
// The default implementation when nothing gets specified is `egg`.
fun extern _impl(): %eqsat.Impl =
    return %eqsat.slotted;

// To define the cost function that should be used for term extraction,
// simply provide the following config function.
//
// Config values:
// Egg:       AstSize (default), AstDepth
// Slotted:   AstSize (default)
fun extern _cost_fun(): %eqsat.CostFun =
    return %eqsat.AstSize;

// To use a set of rules directly implemented in `egg` or `slotted-egraphs`, define
// the following config function.
//
// To see the existing rulesets, have a look at `src\mim_[egg|slotted]\rulesets`.
// To implement and use your own ruleset, follow the instructions under **Adding rulesets**.
fun extern _rulesets(): %eqsat.RuleSet =
    return %eqsat.rulesets ( %eqsat.standard );

// You can also define your own syntactic rewrite-rules in `MimIR`.
rule foo (x: Nat) = %core.nat.add (x, 0) => x;

// And then tell the eqsat plugin to use them for term rewriting.
fun extern _rules(): %eqsat.Rules =
    return %eqsat.rules ( foo );


// Using your rewrite-rule 'foo', this will be rewritten to:
//
//    fun extern main(x: Nat): Nat =
//        return x;
//
fun extern main(x: Nat): Nat =
    return %core.nat.add (x, 0);
```

## Installation

To install this plugin simply follow the instructions below:

**1. Clone the `mimir` repository if you haven't already**

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
