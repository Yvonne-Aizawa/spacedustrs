/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://v2.api.spacetraders.io\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// SystemType : The type of waypoint.

/// The type of waypoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemType {
    #[serde(rename = "NEUTRON_STAR")]
    NeutronStar,
    #[serde(rename = "RED_STAR")]
    RedStar,
    #[serde(rename = "ORANGE_STAR")]
    OrangeStar,
    #[serde(rename = "BLUE_STAR")]
    BlueStar,
    #[serde(rename = "YOUNG_STAR")]
    YoungStar,
    #[serde(rename = "WHITE_DWARF")]
    WhiteDwarf,
    #[serde(rename = "BLACK_HOLE")]
    BlackHole,
    #[serde(rename = "HYPERGIANT")]
    Hypergiant,
    #[serde(rename = "NEBULA")]
    Nebula,
    #[serde(rename = "UNSTABLE")]
    Unstable,

}

impl ToString for SystemType {
    fn to_string(&self) -> String {
        match self {
            Self::NeutronStar => String::from("NEUTRON_STAR"),
            Self::RedStar => String::from("RED_STAR"),
            Self::OrangeStar => String::from("ORANGE_STAR"),
            Self::BlueStar => String::from("BLUE_STAR"),
            Self::YoungStar => String::from("YOUNG_STAR"),
            Self::WhiteDwarf => String::from("WHITE_DWARF"),
            Self::BlackHole => String::from("BLACK_HOLE"),
            Self::Hypergiant => String::from("HYPERGIANT"),
            Self::Nebula => String::from("NEBULA"),
            Self::Unstable => String::from("UNSTABLE"),
        }
    }
}

impl Default for SystemType {
    fn default() -> SystemType {
        Self::NeutronStar
    }
}




