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

My first challenge will be to find an encoding that allows arbitrary MimIR programs to be accurately represented in the form of s-Expressions to allow egg to construct an egraph from them.

My idea here is to expand the existing MimIR compiler backends with another backend whose purpose will be to output a program in the form of an s-Expression. To achieve this I have been studying the existing backends in the compiler, specifically the dot backend, along with the way that programs are internally represented. My understanding thus far is that programs are internally represented inside of the **World** class where the program is represented as a set of interconnected **Def**initions where each of them represents an operator or primitive that can have a type and depend on other **Def**initions.

The way that the dot backend constructs a graph from a given program is by recursing over external, mutable definitions until all definitions have been written to a dot file, optionally including types and annexes based on user-provided flags. Since the goal will be to decode the program back into that internal representation from an s-Expression representing a rewritten program, a requirement for the encoding will be to preserve all of the information necessary to ensure a correctly constructed, equivalent MimIR program. It is therefore likely that this encoding will have to include a representation of both types and annexes as well.

Another idea I had for implementing the s-Expression backend was to directly translate the AST that is constructed upon parsing a program. The way I understand it is that a Mim input file is internally represented as a **Module** containing a list of parsed imports/plugins and a list of declarations. In this case it might suffice to iterate over the list of declarations and recursively generate an s-Expression for each declaration.

### Encoding Mim rewrite rules into egg rewrite rules

The next challenge will be to translate a variety of rules defined in different formats into the uniform rule format expected by egg.
There are two types of rewrite rules that I could make out in the MimIR ecosystem; the first variety are rewrites defined as part of the normalizers bundled with a plugin, the second variety are rewrite rules defined directly in Mim. I might be able to utilize parts of the encoding system from the previous task in order to translate arbitrary rewrite rules defined in Mim but this will likely prove more difficult for the manual rewrites defined in C++.

I am also considering to manually translate a portion of existing C++ normalizers into corresponding Mim rules as a chapter in the thesis or as part of a later case study to demonstrate the effectiveness of applying rewrite rules as part of an equality saturation procedure.

### Performing rewrites in egg

After having converted both MimIR programs and rewrite rules into the appropriate formats, it should then become possible to perform rewrites via the provided equality saturation engine and library functions for finding an optimal rewrite sequence. My plan here is to first extract a most optimal equivalent program via a term-size cost function and maybe later look towards more sophisticated cost functions that utilize integer linear programming.

The part that will be more challenging here will be to find a way to integrate the entire process of parsing Mim compiler generated s-Expressions in egg, performing rewrites, and then emitting another s-Expression to be decoded by the s-Expression backend of the Mim compiler, into the existing Mim compiler pipeline. To achieve this, I will try to implement a compile pass and integrate it into the compiler using the **PassMan** interface. I will need to perform some more research on how that will work exactly.

### Decoding from s-Expr to MimIR

The last step after having encoded and rewritten MimIR programs in egg is to decode the emitted s-Expression back into a format that correctly represents the program for the Mim compiler.

## Case Study
