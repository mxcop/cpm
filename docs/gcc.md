# GCC Commands

### Linker*
Compiles the code into an executable.<br>
Generates the output file `program.exe`
```bash
$ gcc main.c -o program
```
Adding include directories is done using `-I"<dir>"`
```bash
$ gcc main.c -o program -I"inc"
```
Compiling multiple C files.
```bash
$ gcc main.c test.c -o program -I"inc"
```

<br>

# Optional Commands

### Preprocessing
Handles things like Macros and Comments.<br>
Generates the output file `main.i`
```bash
$ gcc -E main.c
```

### Compiling
Compiles the code to Intermediate Representation.<br>
Generates the output file `main.s`
```bash
$ gcc -S main.c
```

### Assembler
Assemble the code to an Object file.<br>
Generates the output file `main.o`
```bash
$ gcc -c main.c
```
