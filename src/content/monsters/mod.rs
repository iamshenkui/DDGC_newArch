//! Monster family content entrypoints for DDGC monster migration.
//!
//! Each monster family gets its own submodule (e.g., `mantis_magic_flower`)
//! that provides `archetype()` and `skill_pack()` factory functions, following
//! the same contract as hero content modules.
//!
//! The `register_content` function wires all migrated monster families into
//! the `ContentPack`. Future family migration slices add their submodule
//! declaration here and a registration call in `register_content`.

pub mod alligator_yangtze;
pub mod azure_dragon;
pub mod azure_dragon_ball_thunder;
pub mod azure_dragon_ball_wind;
pub mod dry_tree_genie;
pub mod fox_fire;
pub mod ghost_fire_assist;
pub mod ghost_fire_damage;
pub mod lantern;
pub mod lizard;
pub mod mantis_magic_flower;
pub mod mantis_spiny_flower;
pub mod mantis_walking_flower;
pub mod metal_armor;
pub mod moth_fire;
pub mod moth_mimicry_a;
pub mod moth_mimicry_b;
pub mod robber_melee;
pub mod robber_ranged;
pub mod snake_water;
pub mod tiger_sword;
pub mod unicorn_beetle_a;
pub mod unicorn_beetle_b;
pub mod water_grass;
pub mod monkey_water;

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

    // K8: Moth Mimicry A (US-409)
    pack.register_archetype(moth_mimicry_a::archetype());
    for skill in moth_mimicry_a::skill_pack() {
        pack.register_skill(skill);
    }

    // K9: Moth Mimicry B (US-410)
    pack.register_archetype(moth_mimicry_b::archetype());
    for skill in moth_mimicry_b::skill_pack() {
        pack.register_skill(skill);
    }

    // K10: Robber Melee (US-411)
    pack.register_archetype(robber_melee::archetype());
    for skill in robber_melee::skill_pack() {
        pack.register_skill(skill);
    }

    // K11: Robber Ranged (US-412)
    pack.register_archetype(robber_ranged::archetype());
    for skill in robber_ranged::skill_pack() {
        pack.register_skill(skill);
    }

    // K12: Metal Armor (US-413)
    pack.register_archetype(metal_armor::archetype());
    for skill in metal_armor::skill_pack() {
        pack.register_skill(skill);
    }

    // K13: Tiger Sword (US-414)
    pack.register_archetype(tiger_sword::archetype());
    for skill in tiger_sword::skill_pack() {
        pack.register_skill(skill);
    }

    // K14: Lizard (US-415)
    pack.register_archetype(lizard::archetype());
    for skill in lizard::skill_pack() {
        pack.register_skill(skill);
    }

    // K15: Unicorn Beetle A (US-416)
    pack.register_archetype(unicorn_beetle_a::archetype());
    for skill in unicorn_beetle_a::skill_pack() {
        pack.register_skill(skill);
    }

    // K16: Unicorn Beetle B (US-417)
    pack.register_archetype(unicorn_beetle_b::archetype());
    for skill in unicorn_beetle_b::skill_pack() {
        pack.register_skill(skill);
    }

    // K17: Alligator Yangtze (US-418)
    pack.register_archetype(alligator_yangtze::archetype());
    for skill in alligator_yangtze::skill_pack() {
        pack.register_skill(skill);
    }

    // K18: Ghost Fire Assist (US-419)
    pack.register_archetype(ghost_fire_assist::archetype());
    for skill in ghost_fire_assist::skill_pack() {
        pack.register_skill(skill);
    }

    // K19: Ghost Fire Damage (US-420)
    pack.register_archetype(ghost_fire_damage::archetype());
    for skill in ghost_fire_damage::skill_pack() {
        pack.register_skill(skill);
    }

    // K20: Fox Fire (US-421)
    pack.register_archetype(fox_fire::archetype());
    for skill in fox_fire::skill_pack() {
        pack.register_skill(skill);
    }

    // K21: Moth Fire (US-422)
    pack.register_archetype(moth_fire::archetype());
    for skill in moth_fire::skill_pack() {
        pack.register_skill(skill);
    }

    // K22: Lantern (US-423)
    pack.register_archetype(lantern::archetype());
    for skill in lantern::skill_pack() {
        pack.register_skill(skill);
    }

    // K23: Snake Water (US-424)
    pack.register_archetype(snake_water::archetype());
    for skill in snake_water::skill_pack() {
        pack.register_skill(skill);
    }

    // K24: Water Grass (US-425)
    pack.register_archetype(water_grass::archetype());
    for skill in water_grass::skill_pack() {
        pack.register_skill(skill);
    }

    // K25: Monkey Water (US-426)
    pack.register_archetype(monkey_water::archetype());
    for skill in monkey_water::skill_pack() {
        pack.register_skill(skill);
    }

    // K29: Azure Dragon (US-430)
    pack.register_archetype(azure_dragon::archetype());
    for skill in azure_dragon::skill_pack() {
        pack.register_skill(skill);
    }

    // K29: Azure Dragon Ball Thunder (US-430)
    pack.register_archetype(azure_dragon_ball_thunder::archetype());
    for skill in azure_dragon_ball_thunder::skill_pack() {
        pack.register_skill(skill);
    }

    // K29: Azure Dragon Ball Wind (US-430)
    pack.register_archetype(azure_dragon_ball_wind::archetype());
    for skill in azure_dragon_ball_wind::skill_pack() {
        pack.register_skill(skill);
    }
}
