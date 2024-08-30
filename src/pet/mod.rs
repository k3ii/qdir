use rand::seq::SliceRandom;

const PETS: &[&str] = &[
    "dog",
    "cat",
    "mouse",
    "cow",
    "rabbit",
    "hamster",
    "parrot",
    "goldfish",
    "turtle",
    "guinea pig",
    "horse",
    "donkey",
    "goat",
    "sheep",
    "pig",
    "chicken",
    "duck",
    "goose",
    "ferret",
    "gerbil",
    "canary",
    "parakeet",
    "iguana",
    "gecko",
];

pub fn get_random_pet() -> Option<&'static str> {
    let mut rng = rand::thread_rng();
    PETS.choose(&mut rng).copied()
}
