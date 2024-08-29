use rand::seq::SliceRandom;

const NAMES: &[&str] = &[
    "lovelace",
    "turing",
    "hopper",
    "einstein",
    "curie",
    "davinci",
    "tesla",
    "feynman",
    "torvalds",
    "stallman",
    "ritchie",
    "knuth",
    "dijkstra",
    "wozniak",
    "bernerslee",
    "gosling",
    "matsumoto",
    "thompson",
    "moore",
    "bohr",
    "hawking",
    "ramajuan",
    "dawkins",
    "sagan",
    "darwin",
    "newton",
    "galileo",
    "diffie",
    "hellman",
    "oppenheimer",
    "morse",
    "neumann",
    "pike",
    "kernighan",
];

pub fn get_random_name() -> Option<String> {
    let mut rng = rand::thread_rng();
    NAMES.choose(&mut rng).map(|&name| name.to_string())
}
