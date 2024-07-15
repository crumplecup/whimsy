/// Combat time occurs second by second. BS-362
/// Multiple partipants experience combat as overlapping seconds
/// Because they take actions in a turn order. BS-363
/// Turn order is by highest basic speed.
/// Ties go to highest DX (or maybe highest effective skill) BS-363
///
/// Maximum move is the characters full Move score.
/// A "step" is 1/10 of Move, minimum one meter. BS-363
pub enum Manuever {
    /// Full-turn maneuver.
    /// Bracing a ranged weapon adds +1 to Acc, meaning resting a sandbag, low wall, etc.  A
    /// one-handed firearm is considered braced if used two-handed.  Two-handed weapons are braced
    /// when used prone, or with bi-pod.
    /// +1 for two seconds of Aim, +2 for three or more seconds.
    /// Combined bonus cannot exceed weapons base Accuracy.
    /// Cannot take a "step" with a Braced two-handed weapon.
    /// Active defense spoils an Aim bonus.
    /// If injured, make a Will roll or lose your Aim. BS-364
    Aim,
    /// No active defense.  No dodge, parry or block.
    /// Move up to half Move, only forward.
    AllOutAttack,
    AllOutDefense,
    /// Armed or unarmed attack against an opponent.  Weapons must be ready and targets must be in reach.
    /// May step and attack or attack then step.
    Attack,
    ChangePosture,
    Concentrate,
    /// May attempt a HT roll to recover from physical stun or IQ roll to recover from mental stun,
    /// recovering at the end of the turn. BS-364
    DoNothing,
    /// Melee equivalent of Aim.
    /// +1 per turn (max +3) on an Attack, Feint, All-Out Attack, or Move and Attack against a
    /// specific opponent on the subsequent turn. BS-364/365
    Evaluate,
    /// Feint: BS-365
    /// Roll a Quick Contest of Melee Weaon skills.  May use an unarmed skill, Cloak, Shield or DX
    /// if the skill level is higher.
    ///
    /// * Failure - Faint fails.
    /// * Foe succeeds by as much as player - Faint fails.
    /// * When foe fails, subtract margin of success [`Success::Margin`] from ensuing attack.
    /// * When foe succeeds by [`Success::Margin`] less than player, subtract the difference in
    /// margin from the ensuing attack.
    ///
    /// * Applies to both attacks in an All-Out Attack (Double)
    /// * After striking with your shield, you may also Feint with your shield using the Shield
    /// skill.
    /// * Step movement.
    /// * Parrying with an unbalanced weapon makes the weapon unready on the next turn, wasting the
    /// feint.
    Feint,
    /// Move any number of meters up to your full Move score.
    /// Mounted or vehicle movement counts for full controlled movement.
    /// Sprinting grants a bonus movement on the second and later moves.
    /// No other action but Free Actions. B-364
    Move,
    MoveAndAttack,
    Ready,
    Wait,
}

/// All-Out Attack options for melee attack. BS-365
pub enum AllOutMeleeAttack {
    /// Make a single attack at +4 to hit.
    Determined,
    /// Make two attacks against the same foe.
    ///
    /// * Requires two ready weapons or a weapon that does not need to be readied after use.
    /// * Off-hand weapons are still at -4 per Handedness unless ambidextrous.
    Double,
    /// Make one Feint and one attack, receiving the immediate bonus from the feint.
    Feint,
    /// +2 to damage or +1 damage per die, if greater.
    ///
    /// * Applies to ST-based thrust or swing damage.
    Strong,
}

/// All-Out Attack options for ranged attack. BS-365
pub enum AllOutRangedAttack {
    /// Make a single attack at +1 to hit.
    Determined,
    /// Full-turn maneuver.
    /// Weapon RoF 5+ required.
    SuppressionFire,
}

pub enum Success {
    /// A skill check against a set level or threshold.
    Check,
    /// The margin by which a roll succeeds a set threshold.
    Margin,
}

pub enum FreeAction {
    Talk,
    MaintainSpell,
    DropItem,
    Crouch,
}

/// If you are lying (prone or face up), you must take a Change Poture maneuver to rise to a
/// crawling, kneeling, or sitting posture first.  A second Change Psoture maneuver lets you stand
/// from any of these postures.  Going from standing up to lying down only takes one manuever.
/// You can switch between kneeling and standing (only) as the "step" portion of any maneuver that
/// allows a step instead of using the step to move.
/// Crouching does not require a Change Posture maneuver, it is a free action. B-364
pub enum Posture {
    Standing,
    Sitting,
    Kneeling,
    Crawling,
    LyingProne,
    LyingFaceDown,
}
