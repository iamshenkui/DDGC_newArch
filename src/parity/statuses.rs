//! Status semantic parity fixtures — expectations for bleed, stun, riposte, horror.
//!
//! These fixtures capture the original DDGC stacking rules, tick timing, reactive
//! semantics, and resource interactions. Parity tests use these fixtures to verify
//! the migration preserves status identity rather than flattening them into
//! interchangeable modifier containers.

use framework_rules::statuses::StackRule;

/// Parity expectations for a single status.
#[derive(Debug, Clone)]
pub struct StatusExpectation {
    pub kind: &'static str,
    pub stack_rule: StackRule,
    pub has_modifiers: bool,
}

/// Fixture bundle containing parity expectations for all 4 migrated statuses.
pub struct StatusParityFixture {
    pub bleed: StatusExpectation,
    pub stun: StatusExpectation,
    pub riposte: StatusExpectation,
    pub horror: StatusExpectation,
}

impl StatusParityFixture {
    pub fn new() -> Self {
        StatusParityFixture {
            bleed: StatusExpectation {
                kind: "bleed",
                stack_rule: StackRule::Stack { max: 3 },
                has_modifiers: true,
            },
            stun: StatusExpectation {
                kind: "stun",
                stack_rule: StackRule::Replace,
                has_modifiers: false,
            },
            riposte: StatusExpectation {
                kind: "riposte",
                stack_rule: StackRule::Replace,
                has_modifiers: false,
            },
            horror: StatusExpectation {
                kind: "horror",
                stack_rule: StackRule::Stack { max: 3 },
                has_modifiers: true,
            },
        }
    }
}

impl Default for StatusParityFixture {
    fn default() -> Self {
        Self::new()
    }
}