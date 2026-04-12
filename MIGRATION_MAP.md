# DDGC Migration Map

Maps the first DDGC systems to their framework targets, organized by migration batch.

## Batch 1: Core Attributes and Actors

Migrate the foundational actor model — heroes, monsters, and their base stats.

| DDGC Concept | Framework Target | Notes |
|---|---|---|
| `Hero` / `HeroClass` | `Archetype` (Ally side) | `Archetype::create_actor(id)` produces `ActorAggregate` |
| `Monster` / `MonsterData` | `Archetype` (Enemy side) | Same factory pattern as heroes |
| `HitPoints` current/max | `ATTR_HEALTH` + `ATTR_MAX_HEALTH` | DDGC uses paired attributes; framework uses two separate `AttributeKey` entries |
| `AttackRating` | `ATTR_ATTACK` | Direct mapping |
| `DefenseRating` / `ProtectionRating` | `ATTR_DEFENSE` | Consolidated into one key; DDGC splits physical/magic protection |
| `SpeedRating` | `ATTR_SPEED` | Direct mapping |
| `Stress` current/max | Custom `AttributeKey("stress")` + `AttributeKey("max_stress")` | No paired-attribute concept in framework — approximated with separate keys |
| `Chaos` current/max | Custom `AttributeKey("chaos")` + `AttributeKey("max_chaos")` | Same pattern as stress |
| `CritChance` | Custom `AttributeKey("crit_chance")` | Consumer-defined key |

**First systems:** `actors.rs` (archetype definitions for 2 heroes + 2 monsters), attribute key constants.

## Batch 2: Status Effects and Buffs

Migrate DDGC's buff/status system onto `StatusEffect` + `Modifier`.

| DDGC Concept | Framework Target | Notes |
|---|---|---|
| `Buff` (StatAdd) | `Modifier { source: ModifierSource::Buff, key, value }` | Direct additive mapping |
| `Buff` (StatMultiply) | `Modifier { source: ModifierSource::Buff, key, value }` | Consumer applies multiplication in resolution logic |
| `BuffRule` (conditions) | `EffectCondition` where applicable; game-layer filtering otherwise | See MIGRATION_BLOCKERS.md |
| `StackRule` | `StackRule::Replace / Refresh / Stack { max }` | Direct framework match |
| Stun | `StatusEffect { kind: "stun", modifiers: [], stack_rule: Replace }` | Game-layer enforces "skip turn" |
| Bleed (DoT) | `StatusEffect { kind: "bleed", modifiers: [Modifier damaging HP], stack_rule: Stack { max: 3 } }` | `CombatResolver::end_turn()` auto-ticks |
| Poison (DoT) | Same pattern as bleed | Kind string differentiates |
| Burn (DoT) | Same pattern as bleed | Kind string differentiates |
| Guard (protect ally) | `StatusEffect { kind: "guard", ... }` | Game-layer enforces redirect logic |
| Riposte (counter-attack) | `StatusEffect { kind: "riposte", ... }` | Game-layer triggers counter on hit |

**First systems:** `statuses.rs` (3 statuses: bleed, stun, riposte prototype).

## Batch 3: Combat Skills and Effects

Migrate DDGC's combat skill system onto `SkillDefinition` + `EffectNode`.

| DDGC Concept | Framework Target | Notes |
|---|---|---|
| `CombatSkill` | `SkillDefinition { id, effects, target_selector, action_cost, cooldown }` | Direct structural match |
| `SubEffect` chain | `Vec<EffectNode>` in `SkillDefinition.effects` | Sequential execution |
| `DamageMin/Max` | `EffectNode::damage(amount)` | DDGC uses range; framework uses single value — use average or min |
| `Heal` | `EffectNode::heal(amount)` | Direct mapping |
| `Stun` | `EffectNode::apply_status("stun", duration)` | Direct mapping |
| `Push` / `Pull` | `EffectNode::push(steps)` / `EffectNode::pull(steps)` | Formation-based positioning |
| `LaunchRanks` / `TargetRanks` | `TargetSelector::ByPosition(SlotRange)` or `RelativePosition` | DDGC uses rank-based formation; framework uses slot-based |
| `MultiHitCount` | Multiple `EffectNode::damage()` entries in effects | No built-in multi-hit; replicate via repeated damage nodes |
| `Accuracy` / `CritMod` | `EffectCondition::Probability(chance)` | Approximated; DDGC accuracy is more complex |
| `LimitPerTurn` / `PerBattle` | Game-layer cooldown tracking | Not in framework — see MIGRATION_BLOCKERS.md |

**First systems:** `skills.rs` (5 skills: strike, power_strike, heal, rend, enemy_attack).

## Batch 4: AI Decision Policies

Migrate DDGC's MonsterBrain onto `DesireCalculator`.

| DDGC Concept | Framework Target | Notes |
|---|---|---|
| `MonsterBrain` | `DesireCalculator` trait impl | Consumer provides full decision logic |
| Weighted skill selection | `ActionCandidate { command, desire, target_candidates }` | Map brain weights to desire floats |
| Conditional behavior | `DecisionContext { combat_state, run_state }` | Access to `CombatViewModel` and `RunViewModel` |

**First systems:** `policy.rs` (AggressivePolicy for simple monsters, targeted policy for bosses).

## Batch 5: Progression and Run Flow

Migrate DDGC's dungeon/run structure onto `Run`, `Floor`, `Room`.

| DDGC Concept | Framework Target | Notes |
|---|---|---|
| Dungeon map (rooms, corridors) | `Floor { rooms, rooms_map, entry_room }` | Direct structural match |
| Room types (Combat, Event, Shop, etc.) | `RoomKind` enum | Framework has: Combat, Event, Shop, Treasure, Boss, Corridor, Custom |
| Run state (Active, Victory, Defeat) | `RunState` enum | Direct match |
| Room progression (enter → clear → next) | `Run::enter_room()` / `clear_room()` | Direct match |
| Floor advancement | `Run::advance_floor()` | Direct match |
| Deterministic generation | `DefaultRoomGenerator` with seed | LCG PRNG — deterministic and seed-stable |

**First systems:** Run flow using `DefaultRoomGenerator` with DDGC-appropriate room weights.

## Migration Dependency Order

```
Batch 1 (actors/attrs)
    → Batch 2 (statuses) — statuses reference attribute keys defined in Batch 1
        → Batch 3 (skills) — skills reference statuses and target actors
            → Batch 4 (AI) — AI selects from skills defined in Batch 3
                → Batch 5 (run flow) — run flow triggers combat via Batch 1–4
```

Each batch produces runnable, testable content. No batch requires framework modifications unless a true generic gap is identified.