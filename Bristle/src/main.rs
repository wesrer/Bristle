fn main() {
    println!("Hello, world!");
}

type ParsedTuple<T> = (Option<T>, String);
type ParserResult<T> = Result<ParsedTuple<T>, String>;
type Parser<T> = Box<dyn Fn(String) -> ParserResult<T>>;

// Takes in two functions
// Returns a closure which accepts a String, and parses it

// FIXME: the static lifetimes are ugly
trait ParserCombinator {
    fn combinator<T: 'static>(t1: Parser<T>, t2: Parser<T>) -> Parser<T>;
}

struct Choice {}
struct Dollar {}
struct Star {}
struct FishBone {}
struct Bind {}
struct Chain {}

impl ParserCombinator for Choice {
    fn combinator<T: 'static>(t1: Parser<T>, t2: Parser<T>) -> Parser<T> {
        return Box::new(move |xs: String| match t1(xs) {
            Ok((None, rest)) => t2(rest),
            p => p, // The other values, either success or failure, we can just propagate up
        });
    }
}

impl ParserCombinator for Dollar {
    fn combinator<T: 'static>(t1: Parser<T>, t2: Parser<T>) -> Parser<T> {
        return Box::new(move |xs: String| match t1(xs) {
            Ok((None, rest)) => t2(rest),
            p => p, // The other values, either success or failure, we can just propagate up
        });
    }
}
