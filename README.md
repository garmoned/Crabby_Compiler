# Crabby_Compiler
Successfully made a worse version of C in rust!
LLVM did a lot of the heavy lifting here

Features:
INTS!
While loops!
if statements!
Probably turing complete!

Sadly still has to use Clang just for the linking step because linking is hard.

Sample program:

int i = 0
int f1 = 0
int f2 = 1

while (i < 20) {
    int x = f1 + f2
    print(x)
    f1 = f2
    f2 = x
    i = i + 1
}

This program will out put the first 20 fibonacci numbers
