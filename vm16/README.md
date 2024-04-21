## Building "vm16".

This directory contains C++ source code that implements AM16 as a virtual
machine, vm16. Compile with

```
   g++ fponeasm.cpp vmfplusone.cpp -o vm16
```

The program reads a series of instructions from stdin.  There's a sample
program, `sample.am16` which can be executed with

```
   ./vm16 < sample.am16
```

This should produce the following output:

```
push 2: ax=0, bx=0, cx=0, sp=4097, bp=0, ma=0, pc=2, tos=2
push 9: ax=0, bx=0, cx=0, sp=4098, bp=0, ma=0, pc=3, tos=9
pop ax: ax=9, bx=0, cx=0, sp=4097, bp=0, ma=0, pc=4, tos=2
pop bx: ax=9, bx=2, cx=0, sp=4096, bp=0, ma=0, pc=5
div bx ax:      ax=4, bx=2, cx=1, sp=4096, bp=0, ma=0, pc=6
push ax:        ax=4, bx=2, cx=1, sp=4097, bp=0, ma=0, pc=7, tos=4
```
