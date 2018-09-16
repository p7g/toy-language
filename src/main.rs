mod parser;
mod engine;

use self::parser::*;
use self::engine::{ evaluate, Environment };

fn main() {
    let code = &r#"
# this is a comment

println("Hello World!");

println(2 + 3 * 4);

# functions are introduced with `lambda` or `λ`
fib = fn (n) if n < 2 then n else fib(n - 1) + fib(n - 2);

println(fib(15));

print_range = fn(a, b)             # `λ` is synonym to `lambda`
                if a <= b then {  # `then` here is optional as you can see below
                  print(a);
                  if a + 1 <= b then {
                    print(", ");
                    print_range(a + 1, b);
                  } else println("");        # newline
                };
print_range(1, 5);
"#.to_string();

    let input_stream = InputStream::new(code);
    let lexer = TokenStream::new(input_stream);
    let mut parser = Parser::new(lexer);

    let mut env = Environment::new(None);

    let func = |args: Vec<AST>| {
        let mut string = "".to_string();
        for i in args.iter() {
            let s = match i {
                AST::String_(lit) => lit.clone(),
                AST::Number(num) => format!("{}", num),
                AST::Boolean(b) => if *b { "true" } else { "false" }.to_string(),
                x => format!("{:?}", x)
            };
            string.push_str(&s);
        }
        print!("{}", string);
        AST::Boolean(true)
    };

    env.def(&"print".to_string(), AST::Function {
        parameters: vec!("string".to_string()),
        body: Box::new(AST::Boolean(true)),
        native: Some(Box::new(func))
    });

    env.def(&"println".to_string(), AST::Function {
        parameters: vec!("string".to_string()),
        body: Box::new(
            Parser::new(
                TokenStream::new(
                    InputStream::new(
                        &"print(string);print(\"\\\n\");".to_string()
                    )
                )
            ).parse()
        ),
        native: None
    });

    evaluate(parser.parse(), &mut env);
}
