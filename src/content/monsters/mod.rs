//! Monster family content entrypoints for DDGC monster migration.
//!
//! Each monster family gets its own submodule (e.g., `mantis_magic_flower`)
//! that provides `archetype()` and `skill_pack()` factory functions, following
//! the same contract as hero content modules.
//!
//! The `register_content` function wires all migrated monster families into
//! the `ContentPack`. Future family migration slices add their submodule
//! declaration here and a registration call in `register_content`.

pub mod dry_tree_genie;
pub mod mantis_magic_flower;
pub mod mantis_spiny_flower;
pub mod mantis_walking_flower;

use crate::content::ContentPack;

/// Register all migrated monster family content into the content pack.
///
/// Each family migration slice (US-405 through US-426 for commons,
/// US-430 through US-441 for bosses) adds a registration call here.
pub fn register_content(pack: &mut ContentPack) {
    // K4: Mantis Magic Flower (US-405)
    pack.register_archetype(mantis_magic_flower::archetype());
    for skill in mantis_magic_flower::skill_pack() {
        pack.register_skill(skill);
    }

    // K5: Mantis Spiny Flower (US-406)
    pack.register_archetype(mantis_spiny_flower::archetype());
    for skill in mantis_spiny_flower::skill_pack() {
        pack.register_skill(skill);
    }

    // K6: Mantis Walking Flower (US-407)
    pack.register_archetype(mantis_walking_flower::archetype());
    for skill in mantis_walking_flower::skill_pack() {
        pack.register_skill(skill);
    }

    // K7: Dry Tree Genie (US-408)
    pack.register_archetype(dry_tree_genie::archetype());
    for skill in dry_tree_genie::skill_pack() {
        pack.register_skill(skill);
    }
}
