/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://v2.api.spacetraders.io\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// ShipModule : A module can be installed in a ship and provides a set of capabilities such as storage space or quarters for crew. Module installations are permanent.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipModule {
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "requirements")]
    pub requirements: Box<crate::models::ShipRequirements>,
}

impl ShipModule {
    /// A module can be installed in a ship and provides a set of capabilities such as storage space or quarters for crew. Module installations are permanent.
    pub fn new(symbol: Symbol, name: String, requirements: crate::models::ShipRequirements) -> ShipModule {
        ShipModule {
            symbol,
            capacity: None,
            range: None,
            name,
            description: None,
            requirements: Box::new(requirements),
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Symbol {
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    MineralProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    CargoHoldI,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    CrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    EnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    PassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    MicroRefineryI,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    OreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    FuelRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    JumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    JumpDriveIi,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    JumpDriveIii,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    WarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    WarpDriveIi,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    WarpDriveIii,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ShieldGeneratorIi,
}

impl Default for Symbol {
    fn default() -> Symbol {
        Self::MineralProcessorI
    }
}
