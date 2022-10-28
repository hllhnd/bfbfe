# BFBFE
BFBFE (Big Fucking Brainfuck Engine) is an optimizing transpiler for Brainfuck. It currently offers only C as a target and is relatively incomplete in many aspects.

It is relatively modular and its components are exposed as crates.

## Quick Start
```
$ cargo build --release
```
```
$ ./target/release/bfbfe-cli --help
```
```
$ echo '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.' > hello.bf
$ ./target/release/bfbfe-cli --target c -o hello.c hello.bf
Build complete in 40 μs
Tokenizing              2 μs
Instructionizing        6 μs
Optimizing              13 μs
Compiling               19 μs
$ gcc -O2 -o hello hello.c
$ ./hello
Hello World!
```

## Licensing
BFBFE and all its components are licensed under the GNU General Public License Version 3 or any later version.

See [LICENSE](LICENSE) for more details.
