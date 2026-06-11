import type {
  DdgcFrontendSnapshot,
  DdgcFrontendIntent,
  ExpeditionResultViewModel,
  ReturnViewModel,
  TownViewModel,
  ExpeditionPlanningViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
  HeroDetailViewModel,
  BuildingDetailViewModel
} from "../bridge/contractTypes";

export type ScreenKey = "startup" | "loading" | "town" | "hero-detail" | "building-detail" | "expedition-planning" | "dungeon-select" | "provisioning" | "dungeon-hint" | "expedition" | "dungeon-assist" | "dungeon-map" | "combat" | "dungeon-interaction" | "result" | "return" | "unsupported" | "fatal";

export function resolveScreen(snapshot: DdgcFrontendSnapshot): ScreenKey {
  if (snapshot.lifecycle === "fatal") {
    return "fatal";
  }

  if (snapshot.lifecycle === "unsupported") {
    return "unsupported";
  }

  if (snapshot.lifecycle === "loading" || snapshot.lifecycle === "booting") {
    return "loading";
  }

  if (snapshot.viewModel.kind === "hero-detail") {
    return "hero-detail";
  }

  if (snapshot.viewModel.kind === "building-detail") {
    return "building-detail";
  }

  if (snapshot.viewModel.kind === "expedition-planning") {
    return "expedition-planning";
  }

  if (snapshot.viewModel.kind === "dungeon-select") {
    return "dungeon-select";
  }

  if (snapshot.viewModel.kind === "provisioning") {
    return "provisioning";
  }

  if (snapshot.viewModel.kind === "dungeon-hint") {
    return "dungeon-hint";
  }

  if (snapshot.viewModel.kind === "expedition") {
    return "expedition";
  }

  if (snapshot.viewModel.kind === "dungeon-assist") {
    return "dungeon-assist";
  }

  if (snapshot.viewModel.kind === "dungeon-map") {
    return "dungeon-map";
  }

  if (snapshot.viewModel.kind === "combat") {
    return "combat";
  }

  if (snapshot.viewModel.kind === "dungeon-interaction") {
    return "dungeon-interaction";
  }

  if (snapshot.viewModel.kind === "result") {
    return "result";
  }

  if (snapshot.viewModel.kind === "return") {
    return "return";
  }

  if (snapshot.flowState === "town") {
    return "town";
  }

  return "startup";
}

export interface TransitionValidation {
  allowed: boolean;
  reason?: string;
}

function activeCombatHero(snapshot: DdgcFrontendSnapshot) {
  const viewModel = snapshot.viewModel;
  if (viewModel.kind !== "combat") {
    return undefined;
  }

  return viewModel.party.find((hero) => hero.id === viewModel.activeHeroId);
}

function isCharacterHitAcknowledgement(snapshot: DdgcFrontendSnapshot): boolean {
  return snapshot.viewModel.kind === "combat" && snapshot.viewModel.phase === "character-hit";
}

export function canTransition(
  snapshot: DdgcFrontendSnapshot,
  intent: DdgcFrontendIntent
): TransitionValidation {
  const screen = resolveScreen(snapshot);

  switch (intent.type) {
    case "continue-from-result":
      if (screen !== "result") {
        return { allowed: false, reason: "continue-from-result is only valid on result screen" };
      }
      if (snapshot.viewModel.kind !== "result") {
        return { allowed: false, reason: "viewModel is not a result view model" };
      }
      if (!snapshot.viewModel.isContinueAvailable) {
        return { allowed: false, reason: "continue is not available" };
      }
      return { allowed: true };

    case "resume-from-return":
      if (screen !== "return") {
        return { allowed: false, reason: "resume-from-return is only valid on return screen" };
      }
      if (snapshot.viewModel.kind !== "return") {
        return { allowed: false, reason: "viewModel is not a return view model" };
      }
      if (!snapshot.viewModel.isTownResumeAvailable) {
        return { allowed: false, reason: "town resume is not available" };
      }
      return { allowed: true };

    case "select-skill":
      if (screen !== "combat") {
        return { allowed: false, reason: "select-skill is only valid in combat" };
      }
      if (snapshot.viewModel.kind !== "combat") {
        return { allowed: false, reason: "viewModel is not a combat view model" };
      }
      if (isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "select-skill is not valid during character-hit acknowledgement" };
      }
      if (!snapshot.viewModel.isPlayerTurn) {
        return { allowed: false, reason: "not player turn" };
      }
      {
        const activeHero = activeCombatHero(snapshot);
        if (!activeHero || !activeHero.isAlive) {
          return { allowed: false, reason: "active combat hero is missing or defeated" };
        }
        const skill = activeHero.skills.find((item) => item.id === intent.skillId);
        if (!skill) {
          return { allowed: false, reason: `combat skill ${intent.skillId} does not exist for active hero` };
        }
        if (skill.cooldownRemaining > 0) {
          return { allowed: false, reason: `combat skill ${intent.skillId} is on cooldown` };
        }
      }
      return { allowed: true };

    case "select-target":
      if (screen !== "combat") {
        return { allowed: false, reason: "select-target is only valid in combat" };
      }
      if (snapshot.viewModel.kind !== "combat") {
        return { allowed: false, reason: "viewModel is not a combat view model" };
      }
      if (isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "select-target is not valid during character-hit acknowledgement" };
      }
      if (!snapshot.viewModel.isPlayerTurn) {
        return { allowed: false, reason: "not player turn" };
      }
      {
        const target = snapshot.viewModel.enemies.find((enemy) => enemy.id === intent.enemyId);
        if (!target) {
          return { allowed: false, reason: `combat target ${intent.enemyId} does not exist` };
        }
        if (!target.isAlive) {
          return { allowed: false, reason: `combat target ${intent.enemyId} is defeated` };
        }
      }
      return { allowed: true };

    case "confirm-attack":
      if (screen !== "combat") {
        return { allowed: false, reason: "confirm-attack is only valid in combat" };
      }
      if (snapshot.viewModel.kind !== "combat") {
        return { allowed: false, reason: "viewModel is not a combat view model" };
      }
      if (isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "confirm-attack is not valid during character-hit acknowledgement" };
      }
      if (!snapshot.viewModel.isPlayerTurn) {
        return { allowed: false, reason: "not player turn" };
      }
      {
        const combatVm = snapshot.viewModel;
        const activeHero = activeCombatHero(snapshot);
        if (!activeHero || !activeHero.isAlive) {
          return { allowed: false, reason: "active combat hero is missing or defeated" };
        }
        const selectedSkill = activeHero.skills.find((skill) => skill.id === combatVm.selectedSkillId);
        if (!selectedSkill) {
          return { allowed: false, reason: "selected combat skill does not exist for active hero" };
        }
        if (selectedSkill.cooldownRemaining > 0) {
          return { allowed: false, reason: `selected combat skill ${selectedSkill.id} is on cooldown` };
        }
        const selectedTarget = combatVm.enemies.find((enemy) => enemy.isTargeted);
        if (!selectedTarget) {
          return { allowed: false, reason: "no live combat target is selected" };
        }
        if (!selectedTarget.isAlive) {
          return { allowed: false, reason: `selected combat target ${selectedTarget.id} is defeated` };
        }
      }
      return { allowed: true };

    case "continue-from-combat":
      if (screen !== "combat") {
        return { allowed: false, reason: "continue-from-combat is only valid in combat" };
      }
      if (snapshot.viewModel.kind !== "combat") {
        return { allowed: false, reason: "viewModel is not a combat view model" };
      }
      if (!isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "continue-from-combat is only valid during character-hit acknowledgement" };
      }
      return { allowed: true };

    case "flee-combat":
      if (screen !== "combat") {
        return { allowed: false, reason: "flee-combat is only valid in combat" };
      }
      if (snapshot.viewModel.kind !== "combat") {
        return { allowed: false, reason: "viewModel is not a combat view model" };
      }
      if (!snapshot.viewModel.canFlee) {
        return { allowed: false, reason: "cannot flee this combat" };
      }
      if (isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "flee-combat is not valid during character-hit acknowledgement" };
      }
      return { allowed: true };

    case "end-turn":
      if (screen !== "combat") {
        return { allowed: false, reason: "end-turn is only valid in combat" };
      }
      if (snapshot.viewModel.kind !== "combat") {
        return { allowed: false, reason: "viewModel is not a combat view model" };
      }
      if (isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "end-turn is not valid during character-hit acknowledgement" };
      }
      if (!snapshot.viewModel.isPlayerTurn) {
        return { allowed: false, reason: "not player turn" };
      }
      return { allowed: true };

    case "open-combat-settings":
      if (screen !== "combat") {
        return { allowed: false, reason: "open-combat-settings is only valid in combat" };
      }
      if (isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "open-combat-settings is not valid during character-hit acknowledgement" };
      }
      return { allowed: true };

    case "return-to-town":
      if (screen === "town" || screen === "startup" || screen === "loading") {
        return { allowed: false, reason: "already in town or transitioning" };
      }
      if (isCharacterHitAcknowledgement(snapshot)) {
        return { allowed: false, reason: "return-to-town is not valid during character-hit acknowledgement" };
      }
      return { allowed: true };

    case "start-expedition-planning":
      if (screen !== "town") {
        return { allowed: false, reason: "start-expedition-planning is only valid in town" };
      }
      return { allowed: true };

    case "select-plane":
      if (screen !== "expedition-planning") {
        return { allowed: false, reason: "select-plane is only valid in expedition-planning" };
      }
      if (snapshot.viewModel.kind !== "expedition-planning") {
        return { allowed: false, reason: "viewModel is not an expedition-planning view model" };
      }
      {
        const plane = snapshot.viewModel.planes.find((item) => item.id === intent.planeId);
        if (!plane) {
          return { allowed: false, reason: "unknown plane id" };
        }
        if (plane.isLocked) {
          return { allowed: false, reason: "plane is locked" };
        }
      }
      return { allowed: true };

    case "toggle-planning-hero":
      if (screen !== "expedition-planning") {
        return { allowed: false, reason: "toggle-planning-hero is only valid in expedition-planning" };
      }
      return { allowed: true };

    case "proceed-to-provisioning":
      if (screen !== "expedition-planning") {
        return { allowed: false, reason: "proceed-to-provisioning is only valid in expedition-planning" };
      }
      if (snapshot.viewModel.kind !== "expedition-planning") {
        return { allowed: false, reason: "viewModel is not an expedition-planning view model" };
      }
      if (!snapshot.viewModel.isReadyToProvision) {
        return { allowed: false, reason: "not ready to provision" };
      }
      return { allowed: true };

    case "start-dungeon-select":
      if (screen !== "town") {
        return { allowed: false, reason: "start-dungeon-select is only valid in town" };
      }
      return { allowed: true };

    case "select-dungeon":
      if (screen !== "dungeon-select") {
        return { allowed: false, reason: "select-dungeon is only valid in dungeon-select" };
      }
      if (snapshot.viewModel.kind !== "dungeon-select") {
        return { allowed: false, reason: "viewModel is not a dungeon-select view model" };
      }
      {
        const dungeon = snapshot.viewModel.dungeons.find((d) => d.id === intent.dungeonId);
        if (!dungeon) {
          return { allowed: false, reason: "unknown dungeon id" };
        }
        if (!dungeon.isAvailable) {
          return { allowed: false, reason: "dungeon is not available" };
        }
      }
      return { allowed: true };

    case "toggle-dungeon-hero":
      if (screen !== "dungeon-select") {
        return { allowed: false, reason: "toggle-dungeon-hero is only valid in dungeon-select" };
      }
      return { allowed: true };

    case "confirm-dungeon-selection":
      if (screen !== "dungeon-select") {
        return { allowed: false, reason: "confirm-dungeon-selection is only valid in dungeon-select" };
      }
      if (snapshot.viewModel.kind !== "dungeon-select") {
        return { allowed: false, reason: "viewModel is not a dungeon-select view model" };
      }
      {
        const vm = snapshot.viewModel;
        if (!vm.isReadyToProceed) {
          return { allowed: false, reason: "dungeon selection is not ready to proceed" };
        }
        const selectedDungeon = vm.dungeons.find((d) => d.id === vm.selectedDungeonId);
        if (!selectedDungeon) {
          return { allowed: false, reason: "selected dungeon does not exist" };
        }
        if (!selectedDungeon.isAvailable) {
          return { allowed: false, reason: "selected dungeon is not available" };
        }
      }
      return { allowed: true };

    case "start-provisioning":
      if (screen !== "town" && screen !== "expedition-planning") {
        return { allowed: false, reason: "start-provisioning is only valid in town or expedition-planning" };
      }
      return { allowed: true };

    case "confirm-provisioning":
      if (screen !== "provisioning") {
        return { allowed: false, reason: "confirm-provisioning is only valid in provisioning" };
      }
      if (snapshot.viewModel.kind !== "provisioning") {
        return { allowed: false, reason: "viewModel is not a provisioning view model" };
      }
      if (!snapshot.viewModel.isReadyToLaunch) {
        return { allowed: false, reason: "not ready to launch expedition" };
      }
      return { allowed: true };

    case "accept-dungeon-hint":
      if (screen !== "dungeon-hint") {
        return { allowed: false, reason: "accept-dungeon-hint is only valid in dungeon-hint" };
      }
      if (snapshot.viewModel.kind !== "dungeon-hint") {
        return { allowed: false, reason: "viewModel is not a dungeon-hint view model" };
      }
      if (!snapshot.viewModel.isEnterable) {
        return { allowed: false, reason: "dungeon is not enterable" };
      }
      return { allowed: true };

    case "launch-expedition":
      if (screen !== "expedition") {
        return { allowed: false, reason: "launch-expedition is only valid in expedition" };
      }
      if (snapshot.viewModel.kind !== "expedition") {
        return { allowed: false, reason: "viewModel is not an expedition view model" };
      }
      if (!snapshot.viewModel.isLaunchable) {
        return { allowed: false, reason: "expedition is not launchable" };
      }
      return { allowed: true };

    case "proceed-dungeon":
      if (screen !== "dungeon-interaction") {
        return { allowed: false, reason: "proceed-dungeon is only valid in dungeon-interaction" };
      }
      if (snapshot.viewModel.kind !== "dungeon-interaction") {
        return { allowed: false, reason: "viewModel is not a dungeon-interaction view model" };
      }
      if (!snapshot.viewModel.isProceedAvailable) {
        return { allowed: false, reason: "proceed is not available" };
      }
      return { allowed: true };

    case "interact-room":
      if (screen !== "dungeon-interaction") {
        return { allowed: false, reason: "interact-room is only valid in dungeon-interaction" };
      }
      if (snapshot.viewModel.kind !== "dungeon-interaction") {
        return { allowed: false, reason: "viewModel is not a dungeon-interaction view model" };
      }
      {
        const interaction = snapshot.viewModel.interactions.find((item) => item.id === intent.interactionId);
        if (!interaction) {
          return { allowed: false, reason: `interaction ${intent.interactionId} does not exist` };
        }
        if (!interaction.isAvailable) {
          return { allowed: false, reason: `interaction ${intent.interactionId} is not available` };
        }
      }
      return { allowed: true };

    case "retreat-dungeon":
      if (screen !== "dungeon-interaction") {
        return { allowed: false, reason: "retreat-dungeon is only valid in dungeon-interaction" };
      }
      if (snapshot.viewModel.kind !== "dungeon-interaction") {
        return { allowed: false, reason: "viewModel is not a dungeon-interaction view model" };
      }
      if (!snapshot.viewModel.isRetreatAvailable) {
        return { allowed: false, reason: "retreat is not available" };
      }
      return { allowed: true };

    case "enter-dungeon-assist":
      if (screen !== "expedition") {
        return { allowed: false, reason: "enter-dungeon-assist is only valid in expedition" };
      }
      return { allowed: true };

    case "select-assist-hero":
      if (screen !== "dungeon-assist") {
        return { allowed: false, reason: "select-assist-hero is only valid in dungeon-assist" };
      }
      if (snapshot.viewModel.kind !== "dungeon-assist") {
        return { allowed: false, reason: "viewModel is not a dungeon-assist view model" };
      }
      if (!snapshot.viewModel.party.some((hero) => hero.id === intent.heroId)) {
        return { allowed: false, reason: `assist hero ${intent.heroId} does not exist` };
      }
      return { allowed: true };

    case "use-assist-action":
      if (screen !== "dungeon-assist") {
        return { allowed: false, reason: "use-assist-action is only valid in dungeon-assist" };
      }
      if (snapshot.viewModel.kind !== "dungeon-assist") {
        return { allowed: false, reason: "viewModel is not a dungeon-assist view model" };
      }
      {
        const action = snapshot.viewModel.assistActions.find((item) => item.id === intent.actionId);
        if (!action) {
          return { allowed: false, reason: `assist action ${intent.actionId} does not exist` };
        }
        if (!action.isAvailable) {
          return { allowed: false, reason: `assist action ${intent.actionId} is not available` };
        }
      }
      return { allowed: true };

    case "enter-room":
      if (screen !== "dungeon-map") {
        return { allowed: false, reason: "enter-room is only valid on dungeon-map screen" };
      }
      if (snapshot.viewModel.kind !== "dungeon-map") {
        return { allowed: false, reason: "viewModel is not a dungeon-map view model" };
      }
      if (!intent.roomId || typeof intent.roomId !== "string") {
        return { allowed: false, reason: "roomId is required and must be a string" };
      }
      {
        const mapVm = snapshot.viewModel;
        const targetRoom = mapVm.rooms.find((r) => r.id === intent.roomId);
        if (!targetRoom) {
          return { allowed: false, reason: `room "${intent.roomId}" does not exist in the dungeon` };
        }
        const currentRoom = mapVm.rooms.find((r) => r.id === mapVm.currentRoomId);
        if (currentRoom && intent.roomId !== currentRoom.id && !currentRoom.connections.includes(intent.roomId)) {
          return { allowed: false, reason: `room "${intent.roomId}" is not connected to the current room` };
        }
        if (targetRoom && !targetRoom.isRevealed) {
          return { allowed: false, reason: `room "${intent.roomId}" is not revealed` };
        }
      }
      return { allowed: true };

    case "continue-from-dungeon":
      if (screen !== "dungeon-assist") {
        return { allowed: false, reason: "continue-from-dungeon is only valid in dungeon-assist" };
      }
      if (snapshot.viewModel.kind !== "dungeon-assist") {
        return { allowed: false, reason: "viewModel is not a dungeon-assist view model" };
      }
      if (!snapshot.viewModel.canContinue) {
        return { allowed: false, reason: "cannot continue from dungeon-assist yet" };
      }
      return { allowed: true };

    case "retreat-from-dungeon":
      if (screen !== "dungeon-map") {
        return { allowed: false, reason: "retreat-from-dungeon is only valid on dungeon-map screen" };
      }
      if (snapshot.viewModel.kind !== "dungeon-map") {
        return { allowed: false, reason: "viewModel is not a dungeon-map view model" };
      }
      if (!snapshot.viewModel.isRetreatAvailable) {
        return { allowed: false, reason: "retreat is not available" };
      }
      return { allowed: true };

    case "complete-dungeon":
      if (screen !== "dungeon-map") {
        return { allowed: false, reason: "complete-dungeon is only valid on dungeon-map screen" };
      }
      if (snapshot.viewModel.kind !== "dungeon-map") {
        return { allowed: false, reason: "viewModel is not a dungeon-map view model" };
      }
      if (!snapshot.viewModel.isComplete) {
        return { allowed: false, reason: "dungeon is not complete" };
      }
      return { allowed: true };

    case "open-hero":
      if (screen !== "town") {
        return { allowed: false, reason: "open-hero is only valid in town" };
      }
      return { allowed: true };

    case "open-building":
      if (screen !== "town") {
        return { allowed: false, reason: "open-building is only valid in town" };
      }
      return { allowed: true };

    case "building-action":
      if (screen !== "building-detail") {
        return { allowed: false, reason: "building-action is only valid in building-detail" };
      }
      return { allowed: true };

    case "toggle-hero-selection":
      if (screen !== "provisioning") {
        return { allowed: false, reason: "toggle-hero-selection is only valid in provisioning" };
      }
      return { allowed: true };

    case "boot":
      return { allowed: true };

    default:
      return { allowed: true };
  }
}
