// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-linux-labs: linux labs component model.
//!
//! Real implementation: labss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Labs {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinuxLabsError {
    EmptyLabsName,
    DuplicateLabs(String),
    EmptyEventSubject,
}

impl fmt::Display for LinuxLabsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLabsName => write!(f, "labs name must not be empty"),
            Self::DuplicateLabs(id) => write!(f, "duplicate labs {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LinuxLabsError {}

pub struct LinuxLabs {
    pub name: String,
    labss: BTreeMap<String, Labs>,
    events: BTreeMap<String, Event>,
}

impl LinuxLabs {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            labss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_labs(&mut self, item: Labs) -> Result<(), LinuxLabsError> {
        if item.name.trim().is_empty() {
            return Err(LinuxLabsError::EmptyLabsName);
        }
        let id = item.id.clone();
        if self.labss.contains_key(&id) {
            return Err(LinuxLabsError::DuplicateLabs(id.clone()));
        }
        self.labss.insert(id, item);
        Ok(())
    }

    pub fn labs_count(&self) -> usize {
        self.labss.len()
    }

    pub fn get_labs(&self, id: &str) -> Option<&Labs> {
        self.labss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LinuxLabsError> {
        if event.subject.trim().is_empty() {
            return Err(LinuxLabsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_labs(id: &str, name: &str) -> Labs {
        Labs {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LinuxLabs::new("LinuxLabs");
        assert_eq!(m.name, "LinuxLabs");
        assert_eq!(m.labs_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_labs() {
        let mut m = LinuxLabs::new("LinuxLabs");
        m.add_labs(sample_labs("a1", "Alpha")).unwrap();
        assert_eq!(m.labs_count(), 1);
        assert!(m.get_labs("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LinuxLabs::new("LinuxLabs");
        assert!(m.add_labs(sample_labs("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LinuxLabs::new("LinuxLabs");
        m.add_labs(sample_labs("x", "One")).unwrap();
        assert!(m.add_labs(sample_labs("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LinuxLabs::new("LinuxLabs");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
