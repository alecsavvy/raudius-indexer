ethcontract::contract! {
    pub "EntityManager.json",
    methods {
        initialize(address, uint256) as other_initialize
    },
    event_derives (serde::Deserialize, serde::Serialize),
}
