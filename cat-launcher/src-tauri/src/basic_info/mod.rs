use crate::variants::GameVariant;

pub trait GameVariantBasicInfo {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

impl GameVariantBasicInfo for GameVariant {
    fn name(&self) -> &str {
        match self {
            GameVariant::DarkDaysAhead => "Dark Days Ahead",
            GameVariant::BrightNights => "Bright Nights",
            GameVariant::TheLastGeneration => "The Last Generation",
        }
    }

    fn description(&self) -> &str {
        match self {
            GameVariant::DarkDaysAhead => "A turn-based survival game set in a post-apocalyptic world. Struggle to survive in a harsh, persistent, procedurally generated world. Scavenge the remnants of a dead civilization for food, equipment, or, if you are lucky, a vehicle with a full tank of gas to get you the hell out of Dodge. Fight to defeat or escape from a wide variety of powerful monstrosities, from zombies to giant insects to killer robots and things far stranger and deadlier, and against the others like yourself, that want what you have.",
            GameVariant::BrightNights => "A post-apocalyptic survival rogue-like that tests players to eke a supplies to survive against an onslaught of undead, eldritch abominations and more. Bright Nights emphasizes game balance and interesting combat with heavier sci-fi aspects.",
            GameVariant::TheLastGeneration => "Tells the story of a world where the dead walk, alien horrors stalk the land, and the fabric of reality as we know it has come undone. Despite all this, a few unlucky souls yet survive, and whether the last generation of humanity is destined to die off, or to become something fit to survive in this brave new world is up to you.",
        }
    }
}
