# Thesis Expose: Performing rewrites in MimIR with an equality saturation engine

## Introduction

In this thesis my aim will be to represent arbitrary programs expressed in MimIR into an equality saturation library like egg or slotted-egg in order to be able to perform a set of rewrites, inferred from rewrite rules defined in MimIR, using the equality saturation engine of the chosen library.

There are a few high-level milestones that I would like to focus on in order to achieve this goal. These can be introduced as follows:

- Encoding MimIR into s-Expressions

My targeted equality saturation libraries expect language terms to be expressed in the form of s-Expressions (or string-Expressions) that combine operators and primitives in a structured string notation to then build an internal representation that forms the basis of an egraph. Therefore, I will need to find a way of encoding arbitrary MimIR programs into s-Expressions that the equality saturation library can correctly construct into an egraph.

- Encoding C++ and Mim rewrite rules into egg rewrite rules

Since there are already many existing rewrite rules defined in the various MimIR plugins in the form of normalizers written in C++ and more recently in the Mim language itself, my goal here will be to translate these rewrite rules into the format expected by the egg libraries.
These rules can then be applied via an equality saturation engine to the encoded MimIR to find an optimal order of application.

More interesting here would be the encoding and application of rewrite rules in which different orders of application may lead to the discovery of optimizations otherwise overlooked as I would hope to demonstrate later on in a case study.

- Performing rewrites in egg

This part should not involve a great amount of work on my part as long as I manage to correctly represent MimIR and the relevant rewrite rules in egg. The provided library functions should then be able to find sufficiently optimized equivalent program terms according to a cost function.

- Decoding from s-Expressions to MimIR

After having applied the encoded rewrite rules and hopefully finding an optimized program term I would then need to find a way to translate back from the s-Expression-encoded MimIR to the actual MimIR. To perform the tasks of encoding and decoding MimIR I am looking to extend the existing backends in the Mim compiler by an s-Expression encoder and decoder, which I will outline in more detail in the following section.

## Approach

In the following I will describe in more detail how I am planning to achieve the previously introduced partial goals and the technical challenges that I am expecting to face for each milestone.

### Encoding from MimIR to s-Expr

### Encoding Mim rewrite rules into egg rewrite rules

### Performing rewrites in egg

### Decoding from s-Expr to MimIR

## Case Study
