use models::participant::NewParticipant;
use std::collections::HashMap;

pub trait Elo
where
    Self: Sized,
{
    fn compute_elo(entities: &Vec<Self>) -> Vec<Self>;
}

impl Elo for NewParticipant {
    fn compute_elo(entities: &Vec<NewParticipant>) -> Vec<NewParticipant> {
        let mut transactions = HashMap::new();
        let win_count = entities.into_iter().filter(|entity| entity.win).count();

        if win_count > 0 {
            for i in entities.into_iter() {
                for opponent in entities.into_iter() {
                    if i.deck_id != opponent.deck_id {
                        let expected = expected_score(
                            transformed_rating(i.elo),
                            transformed_rating(opponent.elo),
                        );

                        // We both won or lost - call it a draw
                        let new_elo = if i.win && opponent.win {
                            elo_rating(i.elo, 40.0, GameOutcome::DRAW, expected)
                        } else if i.win && !opponent.win {
                            elo_rating(i.elo, 40.0, GameOutcome::WIN, expected)
                        } else if !i.win && opponent.win {
                            elo_rating(i.elo, 40.0, GameOutcome::LOSE, expected)
                        } else {
                            i.elo
                        };

                        let entry = transactions.entry(i.deck_id).or_insert(0.0);
                        *entry += (new_elo - i.elo) / (win_count as f64)
                    }
                }
            }
        }

        entities
            .into_iter()
            .map(|entity| {
                entity
                    .clone(entity.elo + *transactions.get(&entity.deck_id).unwrap_or(&(0.0 as f64)))
            })
            .collect()
    }
}

impl NewParticipant {
    fn clone(&self, new_elo: f64) -> NewParticipant {
        NewParticipant {
            deck_id: self.deck_id,
            game_id: self.game_id,
            win: self.win,
            elo: new_elo,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Serialize)]
pub enum GameOutcome {
    WIN,
    LOSE,
    DRAW,
}

impl From<GameOutcome> for f64 {
    fn from(outcome: GameOutcome) -> Self {
        match outcome {
            GameOutcome::WIN => 1.0,
            GameOutcome::DRAW => 0.5,
            GameOutcome::LOSE => 0.0,
        }
    }
}

fn expected_score(r1: f64, r2: f64) -> f64 {
    if r1 == 0.0 && r2 == 0.0 {
        0.0
    } else {
        r1 / (r1 + r2)
    }
}

fn transformed_rating(rating: f64) -> f64 {
    (10.0_f64).powf(rating / 400.0)
}

fn elo_rating(current_rating: f64, impact: f64, outcome: GameOutcome, expected_score: f64) -> f64 {
    current_rating + impact * (f64::from(outcome) - expected_score)
}

#[test]
fn test_elo_rating() {
    assert_eq!{
        elo_rating(1000.0, 40.0, GameOutcome::WIN, expected_score(1000.0, 1000.0)),
        1020.0
    }

    assert_eq!{
        elo_rating(1000.0, 40.0, GameOutcome::LOSE, expected_score(1000.0, 1000.0)),
        980.0
    }

    assert_eq!{
        elo_rating(1000.0, 40.0, GameOutcome::DRAW, expected_score(1000.0, 1000.0)),
        1000.0
    }
}

#[test]
fn test_expected_score() {
    assert_eq!(
        expected_score(316.22776601683796, 87.9922539629475),
        0.7823159427696138
    );
    assert_eq!(expected_score(1000.0, 1000.0), 0.5);
    assert_eq!(expected_score(0.0, 0.0), 0.0);
}

#[test]
fn test_tranformed_rating() {
    assert_eq!(transformed_rating(1000.0), 316.22776601683796);
    assert_eq!(transformed_rating(0.0), 1.0);
    assert_eq!(transformed_rating(777.777777), 87.9922539629475);
}

#[test]
fn test_compute_elo_with_3_participants_and_1_winner() {
    let test_case = vec![
        NewParticipant::new(0, 1, true, 1000.0),
        NewParticipant::new(0, 2, false, 1000.0),
        NewParticipant::new(0, 3, false, 1000.0),
    ];

    let result = NewParticipant::compute_elo(&test_case);

    assert_eq!(result[0].elo, 1040.0);
    assert_eq!(result[1].elo, 980.0);
    assert_eq!(result[2].elo, 980.0);
}

#[test]
fn test_compute_elo_with_4_participants_and_2_winners() {
    let test_case = vec![
        NewParticipant::new(0, 1, true, 1000.0),
        NewParticipant::new(0, 2, false, 1000.0),
        NewParticipant::new(0, 3, true, 1000.0),
        NewParticipant::new(0, 4, false, 1000.0),
    ];

    let result = NewParticipant::compute_elo(&test_case);

    assert_eq!(result[0].elo, 1020.0);
    assert_eq!(result[1].elo, 980.0);
    assert_eq!(result[2].elo, 1020.0);
    assert_eq!(result[3].elo, 980.0);
}

#[test]
fn test_compute_elo_with_4_participants_and_4_winners() {
    let test_case = vec![
        NewParticipant::new(0, 1, true, 1000.0),
        NewParticipant::new(0, 2, true, 1000.0),
        NewParticipant::new(0, 3, true, 1000.0),
        NewParticipant::new(0, 4, true, 1000.0),
    ];

    let result = NewParticipant::compute_elo(&test_case);

    assert_eq!(result[0].elo, 1000.0);
    assert_eq!(result[1].elo, 1000.0);
    assert_eq!(result[2].elo, 1000.0);
    assert_eq!(result[3].elo, 1000.0);
}

#[test]
fn test_compute_elo_with_4_participants_and_0_winners() {
    let test_case = vec![
        NewParticipant::new(0, 1, false, 1000.0),
        NewParticipant::new(0, 2, false, 1000.0),
        NewParticipant::new(0, 3, false, 1000.0),
        NewParticipant::new(0, 4, false, 1000.0),
    ];

    let result = NewParticipant::compute_elo(&test_case);

    assert_eq!(result[0].elo, 1000.0);
    assert_eq!(result[1].elo, 1000.0);
    assert_eq!(result[2].elo, 1000.0);
    assert_eq!(result[3].elo, 1000.0);
}

#[test]
fn test_compute_elo_with_3_participants_and_1_winners() {
    let test_case = vec![
        NewParticipant::new(0, 1, true, 1079.0),
        NewParticipant::new(0, 2, false, 800.0),
        NewParticipant::new(0, 3, false, 700.0),
        NewParticipant::new(0, 4, false, 750.0),
    ];

    let result = NewParticipant::compute_elo(&test_case);

    assert_eq!(result[0].elo, 1094.9738566157434);
    assert_eq!(result[1].elo, 793.3145076104151);
    assert_eq!(result[2].elo, 695.9437611556393);
    assert_eq!(result[3].elo, 744.7678746182022);
}
