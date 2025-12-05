# About

Implementing equality saturation for MimIR

## Emitting a processed Mim program to stdout

```bash
MimIR/build/bin/mim example.mim -o -
```

## Creating a MimIR graph of a Mim program

```bash
MimIR/build/bin/mim example.mim --output-dot example.dot
dot -Tpng example.dot -o example.png
```
