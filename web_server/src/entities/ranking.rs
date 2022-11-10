use serde::Serialize;
use postgres_types::ToSql;

#[allow(non_snake_case)]
#[derive(Serialize)]
#[derive(Debug, ToSql)]
pub struct Rank {
    rank: i32,
    userName: String,
    score: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
#[derive(Debug, ToSql)]
pub struct Ranking {
    status: String,
    ranks: Vec<Rank>,
    errorMessage: Option<String>,
}

impl Rank {
    pub fn new(user_name: String, score: i32) -> Self {
        Rank {
            rank: 0,
            userName: user_name,
            score,
        }
    }

    pub fn set_rank(mut self, rank: usize) -> Self {
        self.rank = rank as i32;
        self
    }
}

impl Ranking {
    pub fn new(ranks: Vec<Rank>) -> Self {
        Ranking {
            status: "ok".to_string(),
            ranks,
            errorMessage: None,
        }
    }

    pub fn error() -> Self {
        Ranking {
            status: "ng".to_string(),
            ranks: Vec::new(),
            errorMessage: Some("failed to create ranking".to_string()),
        }
    }
}