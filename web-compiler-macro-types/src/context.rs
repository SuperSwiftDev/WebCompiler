//! # Context Stack Model
//!
//! This module defines the core data structures for modeling context stacks
//! used during the provisioning phase of ContentML.
//!
//! A context stack is a layered structure of guiding text or metadata used to
//! inform LLM-generated content. Each frame in the stack represents a
//! contribution to the tone, intent, or purpose of the generated copy.
//!
//! Stacks are evaluated top-down, enabling both inheritance and refinement of
//! context across nested provisions. This encourages modular, idempotent
//! content generation with clear framing at every level.
//!
//! This module only defines structure; evaluation and I/O are handled separately.

use std::collections::BTreeMap;
use std::path::PathBuf;

// ————————————————————————————————————————————————————————————————————————————
// GLOBAL PROJET SETTINGS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct SiteContextSettings {
    pub default_prompt_template: PathBuf,
}

impl SiteContextSettings {
    pub fn new(default_prompt_template: impl Into<PathBuf>) -> Self {
        Self {
            default_prompt_template: default_prompt_template.into(),
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// FRAMES
// ————————————————————————————————————————————————————————————————————————————

/// A single contextual input (prompt fragment, tone instruction, audience note, etc.)
#[derive(Debug, Clone)]
pub struct ContextFrame {
    /// Free-form natural language prompt fragment or instruction.
    pub content: String,
}

impl ContextFrame {
    pub fn from(text: impl Into<String>) -> Self {
        Self { content: text.into() }
    }
}

impl From<String> for ContextFrame {
    fn from(value: String) -> Self {
        ContextFrame::from(value)
    }
}


// ————————————————————————————————————————————————————————————————————————————
// STACKS
// ————————————————————————————————————————————————————————————————————————————

/// A stack of context frames that together form a full prompt context.
#[derive(Debug, Clone, Default)]
pub struct ContextStack {
    pub frames: Vec<ContextFrame>,
}

impl ContextStack {
    pub fn push(&mut self, frame: impl Into<ContextFrame>) {
        self.frames.push(frame.into());
    }

    pub fn extend(&mut self, other: ContextStack) {
        self.frames.extend(other.frames);
    }

    pub fn combined_with(&self, other: ContextStack) -> ContextStack {
        let mut result = self.clone();
        result.extend(other);
        result
    }
}

// ————————————————————————————————————————————————————————————————————————————
// PROFILES
// ————————————————————————————————————————————————————————————————————————————

/// A named context stack used as a reusable profile in provisioning.
#[derive(Debug, Clone, Default)]
pub struct ContextProfile {
    pub stack: ContextStack,
    pub prompt_template: Option<PathBuf>,
}

impl ContextProfile {
    pub fn with_prompt_template(mut self, path: impl Into<PathBuf>) -> Self {
        self.prompt_template = Some(path.into());
        self
    }
}

// ————————————————————————————————————————————————————————————————————————————
// REGISTRY
// ————————————————————————————————————————————————————————————————————————————

/// A registry of named context stacks.
#[derive(Debug, Clone, Default)]
pub struct ContextRegistry {
    pub profiles: BTreeMap<String, ContextProfile>,
    pub default: ContextProfile,
}

impl ContextRegistry {
    pub fn new(default: ContextProfile) -> Self {
        Self {
            profiles: Default::default(),
            default,
        }
    }
}

