use ethcontract_generate::loaders::TruffleLoader;
use ethcontract_generate::ContractBuilder;

fn main() {
    // Prepare filesystem paths.
    let dest = std::path::Path::new("./src/generated/contracts/entity_manager.rs");

    // Load a contract.
    let contract = TruffleLoader::new()
        .load_contract_from_file("./EntityManager.json")
        .expect("issue loading entity manager abi");

    // Generate bindings for it.
    ContractBuilder::new()
        // .add_method_alias(
        //     "initialize(string,string,uint256)",
        //     "another_init(string,string,uint256)",
        // )
        .add_event_derive("serde::Serialize")
        .add_event_derive("serde::Deserialize")
        .generate(&contract)
        .expect("issue generating contract bindings")
        .write_to_file(dest)
        .expect("issue writing generated contract output");
}
