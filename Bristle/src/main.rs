fn main() {
    println!("Hello, world!");
}

type ParsedTuple<T, X> = (T, Vec<X>);
type ParserResult<T, X> = Option<Result<ParsedTuple<T, X>, String>>;
type Parser<T, X> = Box<dyn Fn(&Vec<X>) -> ParserResult<T, X>>;

// Takes in two functions
// Returns a closure which accepts a String, and parses it

// FIXME: the static lifetimes are ugly
trait ParserCombinator {
    fn combinator<T: 'static, X: 'static, P: 'static>(t1: P, t2: P) -> Parser<T, X>
    where
        P: Fn(&Vec<X>) -> ParserResult<T, X>,
        X: Clone;
}

struct Choice {}
struct Dollar {}
struct Star {}
struct FishBone {}
struct Bind {}
struct Chain {}

impl ParserCombinator for Choice {
    fn combinator<T: 'static, X: 'static, P: 'static>(t1: P, t2: P) -> Parser<T, X>
    where
        P: Fn(&Vec<X>) -> ParserResult<T, X>,
        X: Clone,
    {
        return Box::new(move |xs: &Vec<X>| match t1(&xs) {
            None => t2(&xs),
            p => p, // The other values, either success or failure, we can just propagate up
        });
    }
}

impl ParserCombinator for Star {
    fn combinator<T: 'static, X: 'static, P: 'static>(t1: P, t2: P) -> Parser<T, X>
    where
        P: Fn(&Vec<X>) -> ParserResult<T, X>,
        X: Clone,
    {
        return Box::new(move |xs: &Vec<X>| match t1(&xs) {
            Some(Ok((value, rest))) => t2(&rest),
            p => p, // The other values, either success or failure, we can just propagate up
        });
    }
}
