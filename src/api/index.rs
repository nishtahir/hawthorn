use api::error::ApiError;
use rand;
use rand::Rng;
use rocket_contrib::Json;

const QUOTES: &'static [&'static str] = &[
    "Tap to add 3 mana of any color to your mana pool",
    "Will you pay 1 to prevent me from drawing a card?",
    "Sacrifice! Sacrifice! Sacrifice!",
    "Everything costs 1 more. It's non optional",
    "He's ramping! Kill him!",
    "Fog!",
    "TAKE THE GOLD!",
    "Oh my gosh! He revealed his hand!",
    "Did you see that swamp he had!?",
    "It's an AD!",
    "Ob Nixilis Reignited: He was on fire before, and he's on fire again",
    "Hmmmmm... Hmmm... Ponder...",
    "Wait...can I re-do this turn?",
    "I'm not trying to win",
    "I now have infinite life",
];

#[derive(Serialize)]
struct IndexResponse {
    quote: &'static str,
}

#[get("/")]
fn index() -> Result<Json<IndexResponse>, ApiError> {
    let quote = rand::thread_rng().choose(&QUOTES).map_or("", |&s| s);
    Ok(Json(IndexResponse { quote: quote }))
}
