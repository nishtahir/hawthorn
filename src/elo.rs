use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum GameOutcome {
    WIN,
    LOSE,
    DRAW,
}

impl GameOutcome {
    pub fn as_f64(&self) -> f64 {
        match *self {
            GameOutcome::WIN => 1.0,
            GameOutcome::DRAW => 0.5,
            GameOutcome::LOSE => 0.0,
        }
    }
}

struct RankableEntity {
    id: i32,
    elo: f64,
    outcome: GameOutcome,
}

impl RankableEntity {
    fn new(id: i32, elo: f64, outcome: GameOutcome) -> RankableEntity {
        RankableEntity {
            id: id,
            elo: elo,
            outcome: outcome,
        }
    }

    fn clone(&self, elo: f64) -> RankableEntity {
        RankableEntity {
            id: self.id,
            elo: elo,
            outcome: self.outcome,
        }
    }
}

fn compute_elo(entities: &Vec<RankableEntity>) -> Vec<RankableEntity> {
    let mut transactions = HashMap::new();
    let win_count = entities
        .into_iter()
        .filter(|entity| entity.outcome == GameOutcome::WIN)
        .count();

    for i in entities.into_iter() {
        for opponent in entities.into_iter() {
            if i.id != opponent.id {
                let expected = expected_score(i.elo, opponent.elo);
                let new_elo = elo_rating(i.elo, 40.0, i.outcome, expected) / (win_count as f64);
                transactions.insert(i.id, new_elo - i.elo);
            }
        }
    }

    entities
        .into_iter()
        .map(|entity| {
            entity.clone(entity.elo + *transactions.get(&entity.id).unwrap_or(&(0.0 as f64)))
        })
        .collect()
}

fn expected_score(r1: f64, r2: f64) -> f64 {
    r1 / (r1 + r2)
}

fn transformed_rating(rating: f64) -> f64 {
    (10.0_f64).powf(rating / 400.0)
}

fn elo_rating(current_rating: f64, impact: f64, outcome: GameOutcome, expected_score: f64) -> f64 {
    current_rating + impact * (outcome.as_f64() - expected_score)
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
    assert!(expected_score(0.0, 0.0).is_nan());
}

#[test]
fn test_tranformed_rating() {
    assert_eq!(transformed_rating(1000.0), 316.22776601683796);
    assert_eq!(transformed_rating(0.0), 1.0);
    assert_eq!(transformed_rating(777.777777), 87.9922539629475);
}

#[test]
fn test_compute_elo() {
    let test_case = vec![
        RankableEntity::new(1, 1000.0, GameOutcome::WIN),
        RankableEntity::new(2, 1000.0, GameOutcome::LOSE),
        RankableEntity::new(3, 1000.0, GameOutcome::LOSE),
    ];

    let result = elo(&test_case);
    assert_eq!(result[0].elo, 1020.0);
}
