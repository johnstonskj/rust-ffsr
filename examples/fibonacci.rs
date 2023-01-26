use ffsr::{lexer::Lexer, reader::Reader};

const EXAMPLE: &str = r##"
(define (fib n)
  (if (<= n 2)
      1
      (+ (fib (- n 1)) (fib (- n 2)))))

(fib 7) ;; gives 13
"##;

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    let reader = Reader::from(Lexer::from(EXAMPLE));

    for datum in reader.iter() {
        println!("DATUM: {:?}", datum);
    }
}
