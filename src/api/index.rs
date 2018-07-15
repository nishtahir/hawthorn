use rand;
use rand::Rng;

const QUOTES: &'static [&'static str] = &[
    "Tap to add 3 mana of any color to your mana pool",
    "Will you pay 1 to prevent me from drawing a card?",
    "Sacrifice! Sacrifice! Sacrifice!",
    "Everything costs 1 more. It's non optional",
    "He's ramping! Kill him!",
    "Fog!",
];

#[get("/")]
fn index() -> Option<&'static str> {
    rand::thread_rng().choose(&QUOTES).map(|&s| s)
}
