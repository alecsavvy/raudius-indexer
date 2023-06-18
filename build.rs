use ethcontract_generate::loaders::TruffleLoader;
use ethcontract_generate::ContractBuilder;

fn main() {
    // Prepare filesystem paths.
    let dest = std::path::Path::new("./src/entity_manager.rs");

    // Load a contract.
    let contract = TruffleLoader::new()
        .load_contract_from_file("./EntityManager.json")
        .unwrap();

    // Generate bindings for it.
    ContractBuilder::new()
        // .add_method_alias(
        //     "initialize(string,string,uint256)",
        //     "another_init(string,string,uint256)",
        // )
        .add_event_derive("serde::Serialize")
        .add_event_derive("serde::Deserialize")
        .generate(&contract)
        .unwrap()
        .write_to_file(dest)
        .unwrap();
}
