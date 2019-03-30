extern crate libsnarkrs;
use libsnarkrs::common::chain;

trait ChainCrawlern<T> {
    fn contains(&self, val: & T) -> bool;
}

impl ChainCrawlern<std::string::String> for chain::Element<std::string::String> {
    fn contains(&self, val: & std::string::String) -> bool {
        match self {
            chain::Element::Link(ln) => {
                if ln.current.eq(val) {
                    return true;
                }
                ln.prev.contains(val)
            },
            chain::Element::Data(data) => {
                data.eq(val)
            }
        }
    }
}


fn main() {
    let chain: std::boxed::Box<chain::Element<std::string::String>> = Box::new(
        chain::Element::Data(std::string::String::from("Hello"))
    );

    let chain: std::boxed::Box<chain::Element<std::string::String>> = Box::new(
        chain::Element::Link(chain::Link {
            prev: chain,
            current: std::string::String::from(" World")
        })
    );

    let chain: std::boxed::Box<chain::Element<std::string::String>> = Box::new(
        chain::Element::Link(chain::Link {
            prev: chain,
            current: std::string::String::from(" !")
        })
    );

    println!("This is my chain {:?}", &chain);
    println!("contains 'Wrold' {:?}", (&chain).contains(&std::string::String::from("Wrold")));
    println!("contains ' World' {:?}", (&chain).contains(&std::string::String::from(" World")));
    println!("contains 'Hello' {:?}", (&chain).contains(&std::string::String::from("Hello")));
    println!("contains ' !' {:?}", (&chain).contains(&std::string::String::from(" !")));
}
