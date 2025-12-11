use chrono::{
    DateTime,
    TimeZone,
};
use futures::StreamExt;
use libsql::{
    Connection,
    Statement,
};
use serde::Serialize;
use std::pin::Pin;

use crate::db::DateTimeValue;

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Serialize)]
pub struct Question<I> {
    pub yes: I,
    pub no: I,
}

impl<I> Question<I> {
    fn set_answer(&mut self, yes: bool, answer: I) {
        if yes {
            self.yes = answer;
        } else {
            self.no = answer;
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Serialize)]
pub struct Summary<I> {
    pub last_24_hours: Question<I>,
    pub last_12_hours: Question<I>,
    pub last_6_hours: Question<I>,
    pub last_3_hours: Question<I>,
    pub last_hour: Question<I>,
}

impl<I> Summary<I>
where I: Default {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Surveyor {
    submit: Pin<Box<Statement>>,
    summarize: Pin<Box<Statement>>,
}

impl Surveyor {
    pub async fn new(db: &Connection) -> libsql::Result<Self> {
        Ok(Self {
            submit: Box::pin(db.prepare("INSERT INTO submissions (timestamp, monday) VALUES (?1, ?2)").await?),
            summarize: Box::pin(db.prepare(
                "WITH grouped_summary AS MATERIALIZED (
                    SELECT
                        COUNT(*) as count,
                        monday,
                        (unixepoch(?1, '-0000-00-01') <= unixepoch(timestamp)) AS within_24,
                        (unixepoch(?1, '-12:00') <= unixepoch(timestamp)) AS within_12,
                        (unixepoch(?1, '-06:00') <= unixepoch(timestamp)) AS within_6,
                        (unixepoch(?1, '-03:00') <= unixepoch(timestamp)) AS within_3,
                        (unixepoch(?1, '-01:00') <= unixepoch(timestamp)) AS within_1
                    FROM submissions
                    GROUP BY monday, within_24, within_12, within_6, within_3, within_1
                )
                SELECT
                    monday,
                    (SELECT SUM(count) FROM grouped_summary AS g2
                        WHERE g2.monday = g.monday AND g2.within_24),
                    (SELECT SUM(count) FROM grouped_summary AS g2
                        WHERE g2.monday = g.monday AND g2.within_12),
                    (SELECT SUM(count) FROM grouped_summary AS g2
                        WHERE g2.monday = g.monday AND g2.within_6),
                    (SELECT SUM(count) FROM grouped_summary AS g2
                        WHERE g2.monday = g.monday AND g2.within_3),
                    (SELECT SUM(count) FROM grouped_summary AS g2
                        WHERE g2.monday = g.monday AND g2.within_1)
                FROM grouped_summary AS g
                GROUP BY monday"
            ).await?),
        })
    }

    pub async fn submit<TZ>(&self, now: DateTime<TZ>, monday_status: bool) -> libsql::Result<()>
    where TZ: TimeZone {
        self.submit.reset();
        self.submit.execute((DateTimeValue::from(&now), monday_status)).await?;
        Ok(())
    }

    pub async fn summary<TZ>(&self, now: DateTime<TZ>) -> libsql::Result<Summary<i64>>
    where TZ: TimeZone {
        self.summarize.reset();
        self.summarize.query([DateTimeValue::from(&now)])
            .await?
            .into_stream()
            .fold(Ok(Summary::new()), async |summary_res, row_res| {
                summary_res.and_then(|mut summary| {
                    row_res.and_then(|row| {
                        let yes = row.get(0)?;

                        let questions = [
                            &mut summary.last_24_hours,
                            &mut summary.last_12_hours,
                            &mut summary.last_6_hours,
                            &mut summary.last_3_hours,
                            &mut summary.last_hour,
                        ];

                        let answers: [i64; _] = [
                            row.get(1)?,
                            row.get(2)?,
                            row.get(3)?,
                            row.get(4)?,
                            row.get(5)?,
                        ].map(|o: Option<i64>| o.unwrap_or(0));

                        for (question, answer) in questions.into_iter().zip(answers.into_iter()) {
                            question.set_answer(yes, answer);
                        }

                        Ok(summary)
                    })
                })
            })
        .await
    }
}

