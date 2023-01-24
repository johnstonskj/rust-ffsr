use ffsr::{lexer::Lexer, reader::Reader};

const EXAMPLE: &str = r##"
(define (fib n)
  (if (<= n 2)
      1
      (+ (fib (- n 1)) (fib (- n 2)))))
"##;

fn main() {
    let reader = Reader::from(Lexer::from(EXAMPLE));

    for datum in reader.iter() {
        println!("DATUM: {:#?}", datum);
    }
}
