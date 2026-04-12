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
pub mod black_tortoise_a;
pub mod black_tortoise_b;
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
pub mod vermilion_bird;
pub mod vermilion_bird_tail_a;
pub mod vermilion_bird_tail_b;
pub mod white_tiger_a;
pub mod white_tiger_b;
pub mod white_tiger_c;
pub mod white_tiger_terrain;
pub mod rotvine_wraith;
pub mod rotten_fruit_a;
pub mod rotten_fruit_b;
pub mod rotten_fruit_c;
pub mod skeletal_tiller;
pub mod vegetable;
pub mod necrodrake_embryosac;
pub mod egg_membrane_empty;
pub mod egg_membrane_full;
pub mod gambler;
pub mod mahjong_green;
pub mod mahjong_red;
pub mod mahjong_white;
pub mod sc_blow;
pub mod sc_bow;
pub mod sc_pluck;
pub mod scorchthroat_chanteuse;

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

    // K30: Vermilion Bird (US-431)
    pack.register_archetype(vermilion_bird::archetype());
    for skill in vermilion_bird::skill_pack() {
        pack.register_skill(skill);
    }

    // K30: Vermilion Bird Tail A (US-431)
    pack.register_archetype(vermilion_bird_tail_a::archetype());
    for skill in vermilion_bird_tail_a::skill_pack() {
        pack.register_skill(skill);
    }

    // K30: Vermilion Bird Tail B (US-431)
    pack.register_archetype(vermilion_bird_tail_b::archetype());
    for skill in vermilion_bird_tail_b::skill_pack() {
        pack.register_skill(skill);
    }

    // K31: White Tiger C (US-432)
    pack.register_archetype(white_tiger_c::archetype());
    for skill in white_tiger_c::skill_pack() {
        pack.register_skill(skill);
    }

    // K31: White Tiger A (US-432)
    pack.register_archetype(white_tiger_a::archetype());
    for skill in white_tiger_a::skill_pack() {
        pack.register_skill(skill);
    }

    // K31: White Tiger B (US-432)
    pack.register_archetype(white_tiger_b::archetype());
    for skill in white_tiger_b::skill_pack() {
        pack.register_skill(skill);
    }

    // K31: White Tiger Terrain (US-432)
    pack.register_archetype(white_tiger_terrain::archetype());
    for skill in white_tiger_terrain::skill_pack() {
        pack.register_skill(skill);
    }

    // K32: Black Tortoise A (US-433)
    pack.register_archetype(black_tortoise_a::archetype());
    for skill in black_tortoise_a::skill_pack() {
        pack.register_skill(skill);
    }

    // K32: Black Tortoise B (US-433)
    pack.register_archetype(black_tortoise_b::archetype());
    for skill in black_tortoise_b::skill_pack() {
        pack.register_skill(skill);
    }

    // K33: Rotvine Wraith (US-434)
    pack.register_archetype(rotvine_wraith::archetype());
    for skill in rotvine_wraith::skill_pack() {
        pack.register_skill(skill);
    }

    // K33: Rotten Fruit A (US-434)
    pack.register_archetype(rotten_fruit_a::archetype());
    for skill in rotten_fruit_a::skill_pack() {
        pack.register_skill(skill);
    }

    // K33: Rotten Fruit B (US-434)
    pack.register_archetype(rotten_fruit_b::archetype());
    for skill in rotten_fruit_b::skill_pack() {
        pack.register_skill(skill);
    }

    // K33: Rotten Fruit C (US-434)
    pack.register_archetype(rotten_fruit_c::archetype());
    for skill in rotten_fruit_c::skill_pack() {
        pack.register_skill(skill);
    }

    // K34: Skeletal Tiller (US-435)
    pack.register_archetype(skeletal_tiller::archetype());
    for skill in skeletal_tiller::skill_pack() {
        pack.register_skill(skill);
    }

    // K34: Vegetable (US-435)
    pack.register_archetype(vegetable::archetype());
    for skill in vegetable::skill_pack() {
        pack.register_skill(skill);
    }

    // K35: Necrodrake Embryosac (US-436)
    pack.register_archetype(necrodrake_embryosac::archetype());
    for skill in necrodrake_embryosac::skill_pack() {
        pack.register_skill(skill);
    }

    // K35: Egg Membrane Empty (US-436)
    pack.register_archetype(egg_membrane_empty::archetype());
    for skill in egg_membrane_empty::skill_pack() {
        pack.register_skill(skill);
    }

    // K35: Egg Membrane Full (US-436)
    pack.register_archetype(egg_membrane_full::archetype());
    for skill in egg_membrane_full::skill_pack() {
        pack.register_skill(skill);
    }

    // K36: Gambler (US-437)
    pack.register_archetype(gambler::archetype());
    for skill in gambler::skill_pack() {
        pack.register_skill(skill);
    }

    // K36: Mahjong Red (US-437)
    pack.register_archetype(mahjong_red::archetype());
    for skill in mahjong_red::skill_pack() {
        pack.register_skill(skill);
    }

    // K36: Mahjong Green (US-437)
    pack.register_archetype(mahjong_green::archetype());
    for skill in mahjong_green::skill_pack() {
        pack.register_skill(skill);
    }

    // K36: Mahjong White (US-437)
    pack.register_archetype(mahjong_white::archetype());
    for skill in mahjong_white::skill_pack() {
        pack.register_skill(skill);
    }

    // K37: Scorchthroat Chanteuse (US-438)
    pack.register_archetype(scorchthroat_chanteuse::archetype());
    for skill in scorchthroat_chanteuse::skill_pack() {
        pack.register_skill(skill);
    }

    // K37: SC Blow (US-438)
    pack.register_archetype(sc_blow::archetype());
    for skill in sc_blow::skill_pack() {
        pack.register_skill(skill);
    }

    // K37: SC Bow (US-438)
    pack.register_archetype(sc_bow::archetype());
    for skill in sc_bow::skill_pack() {
        pack.register_skill(skill);
    }

    // K37: SC Pluck (US-438)
    pack.register_archetype(sc_pluck::archetype());
    for skill in sc_pluck::skill_pack() {
        pack.register_skill(skill);
    }
}
