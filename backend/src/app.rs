use chrono::{
    DateTime,
    TimeDelta,
    TimeZone,
};
use libsql::{
    Connection,
};
use std::{
    error::Error,
    fmt,
};

use crate::cleaner::{
    Cleaner,
    CleanerError,
};
use crate::surveyor::{
    Summary,
    Surveyor,
};

/// Create tables in the database, if needed.
pub async fn initialize_database(db: &Connection) -> libsql::Result<()> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS submissions (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            monday INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS submission_timestamp_index ON submissions (timestamp);"
    ).await?;
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct AppConfig {
    pub clean_before: TimeDelta,
    pub clean_timeout: TimeDelta,
}

pub struct App<TZ>
where TZ: TimeZone {
    cleaner: Cleaner<TZ>,
    surveyor: Surveyor,
}

impl<TZ> App<TZ>
where TZ: TimeZone {
    pub async fn new(db: &Connection, config: AppConfig) -> libsql::Result<Self> {
        initialize_database(db).await?;

        let cleaner = Cleaner::new(db, config.clean_before, config.clean_timeout).await?;
        let surveyor = Surveyor::new(db).await?;
        Ok(Self {
            cleaner,
            surveyor,
        })
    }

    pub async fn submit(&self, now: DateTime<TZ>, monday_status: bool) -> Result<(), AppError> {
        self.cleaner.queue_clean(now.clone()).await
            .map_err(AppError::Cleaner)?;
        self.surveyor.submit(now, monday_status).await
            .map_err(AppError::Surveyor)?;
        Ok(())
    }

    pub async fn summary(&self, now: DateTime<TZ>) -> Result<Summary<i64>, AppError> {
        self.cleaner.queue_clean(now.clone()).await
            .map_err(AppError::Cleaner)?;
        self.surveyor.summary(now).await
            .map_err(AppError::Surveyor)
    }
}

#[derive(Debug)]
pub enum AppError {
    Cleaner(CleanerError),
    Surveyor(libsql::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cleaner(inner) => write!(f, "cleaner: {inner}"),
            Self::Surveyor(inner) => write!(f, "surveyor: {inner}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Cleaner(source) => Some(source),
            Self::Surveyor(source) => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Local;
    use libsql::Builder;

    use crate::surveyor::Question;

    fn default_now() -> DateTime<Local> {
        DateTime::parse_from_rfc3339("2020-01-10T00:00:00.000Z").unwrap().into()
    }

    #[tokio::test]
    async fn test_app_submit() {
        let config = AppConfig {
            clean_before: TimeDelta::hours(24),
            clean_timeout: TimeDelta::seconds(60),
        };

        let database = Builder::new_local(":memory:")
            .build()
            .await
            .expect("open database");
        let connection = database.connect().expect("connect");
        initialize_database(&connection).await.expect("initialize database");
        let app = App::new(&connection, config)
            .await
            .expect("new app");

        let now = default_now();
        for i in 0..=30 {
            app.submit(now - TimeDelta::hours(i), true).await.expect("submit");
            app.submit(now - TimeDelta::hours(i), false).await.expect("submit");
        }

        let summary = app.summary(now).await.expect("summary");
        assert_eq!(Summary {
            last_24_hours: Question {
                yes: 25,
                no: 25,
            },
            last_12_hours: Question {
                yes: 13,
                no: 13,
            },
            last_6_hours: Question {
                yes: 7,
                no: 7,
            },
            last_3_hours: Question {
                yes: 4,
                no: 4,
            },
            last_hour: Question {
                yes: 2,
                no: 2,
            },
        }, summary);
    }

    #[tokio::test]
    async fn test_app_keep_duplicates() {
        let config = AppConfig {
            clean_before: TimeDelta::hours(24),
            clean_timeout: TimeDelta::seconds(60),
        };

        let database = Builder::new_local(":memory:")
            .build()
            .await
            .expect("open database");
        let connection = database.connect().expect("connect");
        initialize_database(&connection).await.expect("initialize database");
        let app = App::new(&connection, config)
            .await
            .expect("new app");

        let now = default_now();
        for i in 0..=30 {
            app.submit(now - TimeDelta::hours(i), true).await.expect("submit");
            app.submit(now - TimeDelta::hours(i), true).await.expect("submit");
            app.submit(now - TimeDelta::hours(i), false).await.expect("submit");
            app.submit(now - TimeDelta::hours(i), false).await.expect("submit");
            app.submit(now - TimeDelta::hours(i), false).await.expect("submit");
        }

        let summary = app.summary(now).await.expect("summary");
        assert_eq!(Summary {
            last_24_hours: Question {
                yes: 25 * 2,
                no: 25 * 3,
            },
            last_12_hours: Question {
                yes: 13 * 2,
                no: 13 * 3,
            },
            last_6_hours: Question {
                yes: 7 * 2,
                no: 7 * 3,
            },
            last_3_hours: Question {
                yes: 4 * 2,
                no: 4 * 3,
            },
            last_hour: Question {
                yes: 2 * 2,
                no: 2 * 3,
            },
        }, summary);
    }

    #[tokio::test]
    async fn test_app_clean() {
        let config = AppConfig {
            clean_before: TimeDelta::hours(5),
            clean_timeout: TimeDelta::seconds(0),
        };

        let database = Builder::new_local(":memory:")
            .build()
            .await
            .expect("open database");
        let connection = database.connect().expect("connect");
        initialize_database(&connection).await.expect("initialize database");
        let app = App::new(&connection, config)
            .await
            .expect("new app");

        let now = default_now();
        for i in -30..=0 {
            app.submit(now + TimeDelta::hours(i), true).await.expect("submit");
            app.submit(now + TimeDelta::hours(i), false).await.expect("submit");
        }

        let summary = app.summary(now).await.expect("summary");
        assert_eq!(Summary {
            last_24_hours: Question {
                yes: 6,
                no: 6,
            },
            last_12_hours: Question {
                yes: 6,
                no: 6,
            },
            last_6_hours: Question {
                yes: 6,
                no: 6,
            },
            last_3_hours: Question {
                yes: 4,
                no: 4,
            },
            last_hour: Question {
                yes: 2,
                no: 2,
            },
        }, summary);
    }
}
