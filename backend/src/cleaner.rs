use chrono::{
    DateTime,
    TimeDelta,
    TimeZone,
};
use libsql::{
    Connection,
    Statement,
};
use std::{
    error::Error,
    fmt,
    ops::Deref,
    pin::Pin,
    sync::{
        PoisonError,
        RwLock,
    },
};

use crate::db::DateTimeValue;

pub struct Cleaner<TZ>
where TZ: TimeZone {
    clean_before: TimeDelta,
    clean_timeout: TimeDelta,
    clean: Pin<Box<Statement>>,
    last_clean: RwLock<Option<DateTime<TZ>>>,
}

impl<TZ> Cleaner<TZ>
where TZ: TimeZone {
    pub async fn new(db: &Connection, clean_before: TimeDelta, clean_timeout: TimeDelta) -> libsql::Result<Self> {
        Ok(Self {
            clean_before,
            clean_timeout,
            clean: Box::pin(db.prepare("DELETE FROM submissions WHERE unixepoch(timestamp) < unixepoch(?1)").await?),
            last_clean: RwLock::new(None),
        })
    }

    async fn remove_before(&self, time: DateTime<TZ>) -> Result<(), CleanerError> {
        self.clean.reset();
        self.clean.execute([DateTimeValue::from(&time)]).await?;
        *self.last_clean.write()? = Some(time);
        Ok(())
    }

    pub async fn queue_clean(&self, now: DateTime<TZ>) -> Result<(), CleanerError> {
        if let Some(last_clean) = self.last_clean.read()?.deref() && now.clone() - last_clean < self.clean_timeout {
            return Ok(());
        }

        let clean_before = now - self.clean_before;
        self.remove_before(clean_before).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CleanerError {
    Poisoned,
    LibSql(libsql::Error),
}

impl<T> From<PoisonError<T>> for CleanerError {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poisoned
    }
}

impl From<libsql::Error> for CleanerError {
    fn from(error: libsql::Error) -> Self {
        Self::LibSql(error)
    }
}

impl fmt::Display for CleanerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Poisoned => write!(f, "lock poisoned"),
            Self::LibSql(inner) => write!(f, "libsql: {inner}"),
        }
    }
}

impl Error for CleanerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Poisoned => None,
            Self::LibSql(inner) => Some(inner),
        }
    }
}

