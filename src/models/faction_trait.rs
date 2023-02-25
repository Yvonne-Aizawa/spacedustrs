/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://v2.api.spacetraders.io\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FactionTrait {
    /// The unique identifier of the trait.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// The name of the trait.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the trait.
    #[serde(rename = "description")]
    pub description: String,
}

impl FactionTrait {
    pub fn new(symbol: Symbol, name: String, description: String) -> FactionTrait {
        FactionTrait {
            symbol,
            name,
            description,
        }
    }
}

/// The unique identifier of the trait.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Symbol {
    #[serde(rename = "BUREAUCRATIC")]
    Bureaucratic,
    #[serde(rename = "SECRETIVE")]
    Secretive,
    #[serde(rename = "CAPITALISTIC")]
    Capitalistic,
    #[serde(rename = "INDUSTRIOUS")]
    Industrious,
    #[serde(rename = "PEACEFUL")]
    Peaceful,
    #[serde(rename = "DISTRUSTFUL")]
    Distrustful,
    #[serde(rename = "WELCOMING")]
    Welcoming,
    #[serde(rename = "ANARCHIST")]
    Anarchist,
    #[serde(rename = "CONFLICTED")]
    Conflicted,
    #[serde(rename = "AUTHORITARIAN")]
    Authoritarian,
    #[serde(rename = "OLIGARCHICAL")]
    Oligarchical,
    #[serde(rename = "DYNASTIC")]
    Dynastic,
    #[serde(rename = "DEMOCRACTIC")]
    Democractic,
    #[serde(rename = "DECENTRALIZED")]
    Decentralized,
    #[serde(rename = "SMUGGLERS")]
    Smugglers,
    #[serde(rename = "SCAVENGERS")]
    Scavengers,
    #[serde(rename = "REBELLIOUS")]
    Rebellious,
    #[serde(rename = "EXILES")]
    Exiles,
    #[serde(rename = "PIRATES")]
    Pirates,
    #[serde(rename = "RAIDERS")]
    Raiders,
    #[serde(rename = "CLAN")]
    Clan,
    #[serde(rename = "GUILD")]
    Guild,
    #[serde(rename = "DOMINION")]
    Dominion,
    #[serde(rename = "FRINGE")]
    Fringe,
    #[serde(rename = "FORSAKEN")]
    Forsaken,
    #[serde(rename = "ISOLATED")]
    Isolated,
    #[serde(rename = "LOCALIZED")]
    Localized,
    #[serde(rename = "ESTABLISHED")]
    Established,
    #[serde(rename = "NOTABLE")]
    Notable,
    #[serde(rename = "DOMINANT")]
    Dominant,
    #[serde(rename = "INESCAPABLE")]
    Inescapable,
    #[serde(rename = "INNOVATIVE")]
    Innovative,
    #[serde(rename = "BOLD")]
    Bold,
    #[serde(rename = "VISIONARY")]
    Visionary,
    #[serde(rename = "CURIOUS")]
    Curious,
    #[serde(rename = "DARING")]
    Daring,
    #[serde(rename = "EXPLORATORY")]
    Exploratory,
    #[serde(rename = "RESOURCEFUL")]
    Resourceful,
    #[serde(rename = "FLEXIBLE")]
    Flexible,
    #[serde(rename = "COOPERATIVE")]
    Cooperative,
    #[serde(rename = "UNITED")]
    United,
    #[serde(rename = "STRATEGIC")]
    Strategic,
    #[serde(rename = "INTELLIGENT")]
    Intelligent,
    #[serde(rename = "RESEARCH_FOCUSED")]
    ResearchFocused,
    #[serde(rename = "COLLABORATIVE")]
    Collaborative,
    #[serde(rename = "PROGRESSIVE")]
    Progressive,
    #[serde(rename = "MILITARISTIC")]
    Militaristic,
    #[serde(rename = "TECHNOLOGICALLY_ADVANCED")]
    TechnologicallyAdvanced,
    #[serde(rename = "AGGRESSIVE")]
    Aggressive,
    #[serde(rename = "IMPERIALISTIC")]
    Imperialistic,
    #[serde(rename = "TREASURE_HUNTERS")]
    TreasureHunters,
    #[serde(rename = "DEXTEROUS")]
    Dexterous,
    #[serde(rename = "UNPREDICTABLE")]
    Unpredictable,
    #[serde(rename = "BRUTAL")]
    Brutal,
    #[serde(rename = "FLEETING")]
    Fleeting,
    #[serde(rename = "ADAPTABLE")]
    Adaptable,
    #[serde(rename = "SELF_SUFFICIENT")]
    SelfSufficient,
    #[serde(rename = "DEFENSIVE")]
    Defensive,
    #[serde(rename = "PROUD")]
    Proud,
    #[serde(rename = "DIVERSE")]
    Diverse,
    #[serde(rename = "INDEPENDENT")]
    Independent,
    #[serde(rename = "SELF_INTERESTED")]
    SelfInterested,
    #[serde(rename = "FRAGMENTED")]
    Fragmented,
    #[serde(rename = "COMMERCIAL")]
    Commercial,
    #[serde(rename = "FREE_MARKETS")]
    FreeMarkets,
    #[serde(rename = "ENTREPRENEURIAL")]
    Entrepreneurial,
}

impl Default for Symbol {
    fn default() -> Symbol {
        Self::Bureaucratic
    }
}