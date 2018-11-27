use models::participant::{NewParticipant, Participant};
use std::collections::HashMap;

pub const DEFAULT_IMPACT: f64 = 40.0;

#[derive(PartialEq, Clone, Copy, Serialize)]
pub enum GameOutcome {
    WIN,
    LOSE,
    DRAW,
}

pub trait Elo
where
    Self: Sized,
{
    fn compute_elo(entities: &Vec<Self>) -> Vec<Self>;
}

pub trait Rankable
where
    Self: Sized,
{
    fn clone(&self, new_elo: f64) -> Self;
    fn get_unique_id(&self) -> i32;
    fn get_win(&self) -> bool;
    fn get_elo(&self) -> f64;
}

impl Rankable for Participant {
    fn clone(&self, new_elo: f64) -> Self {
        Participant {
            id: self.id,
            deck_id: self.deck_id,
            game_id: self.game_id,
            win: self.win,
            elo: new_elo,
        }
    }

    fn get_unique_id(&self) -> i32 {
        self.deck_id
    }

    fn get_win(&self) -> bool {
        self.win
    }

    fn get_elo(&self) -> f64 {
        self.elo
    }
}

impl Rankable for NewParticipant {
    fn clone(&self, new_elo: f64) -> Self {
        NewParticipant {
            deck_id: self.deck_id,
            game_id: self.game_id,
            win: self.win,
            elo: new_elo,
        }
    }
    fn get_unique_id(&self) -> i32 {
        self.deck_id
    }

    fn get_win(&self) -> bool {
        self.win
    }

    fn get_elo(&self) -> f64 {
        self.elo
    }
}

impl<T> Elo for T
where
    T: Rankable,
{
    fn compute_elo(entities: &Vec<T>) -> Vec<T> {
        let mut transactions = HashMap::new();
        let win_count = entities
            .into_iter()
            .filter(|entity| entity.get_win())
            .count();

        if win_count > 0 {
            for i in entities.into_iter() {
                for opponent in entities.into_iter() {
                    if i.get_unique_id() != opponent.get_unique_id() {
                        let expected = expected_score(
                            transformed_rating(i.get_elo()),
                            transformed_rating(opponent.get_elo()),
                        );

                        let i_win = i.get_win();
                        let i_elo = i.get_elo();

                        let opponent_win = opponent.get_win();

                        // We both won call it a draw
                        let new_elo = if i_win && opponent_win {
                            elo_rating(i_elo, GameOutcome::DRAW, expected)
                        } else if i_win && !opponent_win {
                            elo_rating(i_elo, GameOutcome::WIN, expected)
                        } else if !i_win && opponent_win {
                            elo_rating(i_elo, GameOutcome::LOSE, expected)
                        } else {
                            i_elo
                        };

                        let entry = transactions.entry(i.get_unique_id()).or_insert(0.0);
                        *entry += (new_elo - i_elo) / (win_count as f64)
                    }
                }
            }
        }

        entities
            .into_iter()
            .map(|entity| {
                entity.clone(
                    entity.get_elo()
                        + *transactions
                            .get(&entity.get_unique_id())
                            .unwrap_or(&(0.0 as f64)),
                )
            })
            .collect()
    }
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

fn elo_rating(current_rating: f64, outcome: GameOutcome, expected_score: f64) -> f64 {
    current_rating + DEFAULT_IMPACT * (f64::from(outcome) - expected_score)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_elo_rating() {
        assert_eq! {
            elo_rating(1000.0, GameOutcome::WIN, expected_score(1000.0, 1000.0)),
            1020.0
        }

        assert_eq! {
            elo_rating(1000.0, GameOutcome::LOSE, expected_score(1000.0, 1000.0)),
            980.0
        }

        assert_eq! {
            elo_rating(1000.0, GameOutcome::DRAW, expected_score(1000.0, 1000.0)),
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

    #[test]
    fn test_compute_elo_with_4_participants_and_1_winner() {
        let test_case = vec![
            NewParticipant::new(0, 1, false, 1005.84301848691),
            NewParticipant::new(0, 2, true, 1115.24769033892),
            NewParticipant::new(0, 3, false, 861.083933121132),
            NewParticipant::new(0, 4, false, 976.272195229627),
        ];

        let result = NewParticipant::compute_elo(&test_case);

        assert_eq!(result[0].elo, 991.9406370569111);
        assert_eq!(result[1].elo, 1149.0708727628635);
        assert_eq!(result[2].elo, 853.564090777122);
        assert_eq!(result[3].elo, 963.8712365796924);
    }

    #[test]
    fn test_compute_elo_with_5_participants_and_1_winner() {
        let test_case = vec![
            NewParticipant::new(0, 1, false, 1005.84301848691),
            NewParticipant::new(0, 2, true, 1115.24769033892),
            NewParticipant::new(0, 3, false, 861.083933121132),
            NewParticipant::new(0, 4, false, 976.272195229627),
            NewParticipant::new(0, 5, false, 954.114406112793),
        ];

        let result = NewParticipant::compute_elo(&test_case);

        assert_eq!(result[0].elo, 991.9406370569111);
        assert_eq!(result[1].elo, 1160.407691064819);
        assert_eq!(result[2].elo, 853.564090777122);
        assert_eq!(result[3].elo, 963.8712365796924);
        assert_eq!(result[4].elo, 942.7775878108375);
    }
}
