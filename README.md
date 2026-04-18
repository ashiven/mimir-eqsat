## MimIR EqSat

This repository contains the [equality saturation](https://en.wikipedia.org/wiki/E-graph#Equality_saturation) implementations for [MimIR](https://github.com/AnyDSL/MimIR).

## Implementations

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
 * Rewrites an sexpr in `slotted-egraphs` format
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
 * Pretty-prints an sexpr in `egg` format
 *
 *  sexpr:     a symbolic expr in `egg` format (emitted by the `mim` compiler via --output-sexpr)
 *  line_len:  the maximal line length after which the sexpr continues on a new line
 */
rust::String pretty(std::string sexpr, size_t line_len);
```

```cpp
/**
 * Pretty-prints an sexpr in `slotted-egraphs` format
 *
 *  sexpr:     a symbolic expr in `slotted-egraphs` format (emitted by the `mim` compiler via --output-sexpr-slotted)
 *  line_len:  the maximal line length after which the sexpr continues on a new line
 */
rust::String pretty(std::string sexpr, size_t line_len);
```

## Installation

There are multiple methods of integrating this library into an existing C++ project. The following lists the simplest ones.
