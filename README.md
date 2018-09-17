# toy-language

This is a really ~~horrible~~
simple programming language (that currently doesn't really work) adapted from [this series](http://lisperator.net/pltut/) to Rust. I am using this as a way to get comfortable with the language, and don't expect this to be something anyone would ever want to use. Not that this has stopped anyone before...

## TODO

- ~~Make it work (currently, it seems like variables aren't being defined properly)~~
- Add error messages that actually help (I just threw a bunch of `panic!()`s in there because I'm lazy, but now I'm suffering)
- Clean up (the code is a mess... way too much repetition)
- Replace the horrible hack in `parser.rs` (the Clone impl for `Fn(Vec<AST>) -> AST`)
  - without fixing this, any sort of standard lib will be impossible... only `print()`

## Syntax

Here is a sample of the greatness you can expect once this thing is working:

```c
# this is a comment

println("Hello World!");

println(2 + 3 * 4);

# functions are introduced with fn
fib = fn (n) if n < 2 then n else fib(n - 1) + fib(n - 2);

println(fib(15));

print_range = fn(a, b)
                if a <= b then {
                  print(a);
                  if a + 1 <= b then {
                    print(", ");
                    print_range(a + 1, b);
                  } else println("");        # newline
                };
print_range(1, 5);
```
