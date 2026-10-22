# c2c
A colon type notation to c style type notation translator.

## Overview
c2c is a preprocessor that converts colon type notations (notation seen most often in Rust, Typescript, Odin, etc) to C style type notations to allow users to use modern notations in C. You write down the type notations you want transformed between pairs(`[<` and `>]`), and the preprocessor will replace the contents in the pairs with the equivalent C code. 

## Installation
The binary is available in [crates.io](https://crates.io/crates/c2c) so install rustup from the [rust](https://rust-lang.org/) website and then do

```bash
cargo install c2c
```
To update run the above command again occasionally. <br>
Make sure `~/.cargo/bin` is in the PATH and you'll be able to do
```bash
c2c hello_world.c2c
``` 
to create `hello_world.c`

## Usage
```
Usage: <path/to/c2c> [-h|--help] [-s|--stdout] [--debug] <arg1> <arg2> ...
    -s | --stdout    : print translation results to stdout instead of writing to file.
    -x | --extension : define fallback extension. default is c.
         --debug     : for debugging purposes.
    -h | --help      : print this message.
Args:
    *.c2c     : will be translated to *.c.
    *.c2h     : will be translated to *.h.
    otherwise : will be translated to the fallback extension or *.c if not specified. 
```

## Examples
A few example c2c code snippets and how it translates to c.

<details open>
  <summary>typedefing a function pointer</summary>
  
  ```c
  typedef [< hello: ptr<fn (int, y: ptr<uint8_t>) -> ptr<ptr<void>>> >];
  ```
  ```c
  typedef void **(*hello)(int , uint8_t *y);
  ```
</details>


<details open>
  <summary>typedefing a pointer to an array</summary>

  ```c
  typedef [< row: ptr<arr<int,5>> >]; 
  ```
  ```c
  typedef int (*row)[5]; 
  ```
</details>

<details open>
  <summary>declaring a function prototype</summary>

  ```c
  [< apply: fn (ptr<fn (int, ptr<struct map>) -> int>, struct map) -> struct map >];
  ```
  ```c
  struct map apply(int (*)(int , struct map *), struct map );
  ```
</details>

<details open>
  <summary>declaring a pointer to a pointer to an array</summary>

  ```c
  [< x: ptr<ptr< arr<int, MAXLEN>>> >];  
  ```
  
  ```c
  int (**x)[MAXLEN];
  ```
</details>

<details open>
  <summary>example code for a stack</summary>

  ```c
  #include<stdlib.h>

  struct Stack {
   	[< top: ptr<struct Node> >];
   	[< size: int >];
  };

  struct Node {
   	[< next: ptr<struct node> >];
   	[< val: int >];
  };

  [< stack_init: fn (stack: ptr<struct Stack>) -> void >] {
   	stack->top = null;
   	stack->size = 0;
  }

  [< stack_push: fn(stack: ptr<struct Stack>, val: int) -> void >] {
 	[< node: ptr<struct Node> >] = ([< ptr<struct Node> >])malloc(sizeof([<struct Node>]));
   	node->val = val;
   	node->next = stack->top;
   	stack->top = node;
  }
  ```
  ```c
  #include<stdlib.h>

  struct Stack {
   	struct Node *top;
   	int size;
  };

  struct Node {
   	struct Node *next;
   	int val;
  };

  void stack_init(struct Stack *stack) {
   	stack->top = NULL;
   	stack->size = 0;
  }

  void stack_push(struct Stack *stack, int val) {
 	struct Node *node = (struct Node *)malloc(sizeof(struct Node ));
   	node->val = val;
   	node->next = stack->top;
   	stack->top = node;
  }

  ```
</details>

## Grammar
The c2c parser searches and simply converts segments surrounded by `[<` and `>]`. In these pairs we can either have a name-type-pair (`<name>: <type>`) for variables/struct field/function declaration/definition, or a type (`<type>`) for casting or nameless parameters in function prototypes. Below are how all the types are defined. Note that `{type}` refers to an arbitrary type and `atom` refers to an arbitrary number or word.
- `ptr<{type}>` gets converted into the pointer of the type `{type}`. 

  ```c
  [< x: ptr<int> >] = NULL;
  ```
  ```c
  int *x = NULL;
  ```

- `arr<{type}, {atom}>` gets converted into an array of elements of type `{type}` and total length `{atom}`

  ```c
  [< x: arr<arr<char, 64>> 3>];
  ```
  ```c
  char x[3][64];
  ```

- `struct {atom}` gets converted into a struct of type `struct {atom}`.
  ```c
  [< x: struct Queue >];
  ```
  ```c
  struct Queue x;
  ```

- `static<{type}>` gets converted into a static type.
  ```c
  ([< static<ptr<char>> >])
  ```
  ```c
  (static char *)
  ```

- `const<{type}>` gets converted into a constant type.
  ```c
  ([< const<ptr<char>> >])
  ```
  ```c
  (const char *)
  ```

- `fn ( {name1}: {type}, {name2}: {type}, ... ) -> {type}` gets converted into a function type. Note that the parameters can be a mix of name-type-pairs and types for function prototype declarations.
  ```c
  [< add: fn (a: int, b: char) -> int >] { ... }
  ```
  ```c
  int add(int a, char b) { ... }
  ```

## BNF
```
  <S>     ::= '[<' <NTP> | <type>  '>]'
  <NTP>   ::= <id> ':' <type>
  <type>  ::= <id>
             | 'struct' <id>
             | 'ptr' '<' <type> '>'
             | 'static' '<' <type> '>'
             | 'const' '<' <type> '>'
             | 'arr' '<' <type> ',' <id> | <num> '>'
             | 'fn' '(' <args> ')' '->' <type>
  <args>  ::= <arg>
             | <args> ',' <arg>
             | ε
  <arg>   ::= <NTP> | <type>
  <id>    ::= <alpha>+(<alpha>|<digit>)*
  <num>   ::= <digit>+ 
  <alpha> ::= [a-zA-Z_]
  <digit> ::= [0-9]
```
