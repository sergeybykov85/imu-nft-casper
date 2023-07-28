#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, Group, Key, Parameter, RuntimeArgs, URef, U256, ApiError
};
use nft_cep47::{Meta, TokenId, CEP47};
use contract_utils::{ContractContext, OnChainContractStorage};


#[derive(Default)]
struct NFTToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for NFTToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CEP47<OnChainContractStorage> for NFTToken {}
impl NFTToken {
    fn constructor(&mut self, admin:Key, minter:Key, name: String, symbol: String, meta: Meta) {
        CEP47::init(self, admin, minter, name, symbol, meta);
    }
}

#[no_mangle]
fn constructor() {
    let name = runtime::get_named_arg::<String>("name");
    let symbol = runtime::get_named_arg::<String>("symbol");
    let meta = runtime::get_named_arg::<Meta>("meta");
    let admin = runtime::get_named_arg::<Key>("admin");
    let minter = runtime::get_named_arg::<Key>("minter");
    NFTToken::default().constructor(admin, minter, name, symbol, meta);
}

#[no_mangle]
fn name() {
    let ret = NFTToken::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn symbol() {
    let ret = NFTToken::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn meta() {
    let ret = NFTToken::default().meta();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    let ret = NFTToken::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let ret = NFTToken::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_token_by_index() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let index = runtime::get_named_arg::<U256>("index");
    let ret = NFTToken::default().get_token_by_index(owner, index);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn owner_of() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = NFTToken::default().owner_of(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = NFTToken::default().token_meta(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn update_token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    NFTToken::default()
        .set_token_meta(token_id, token_meta)
        .unwrap_or_revert();
}

#[no_mangle]
fn update_minter() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    NFTToken::default()
        .update_minter(recipient)
        .unwrap_or_revert();
}

#[no_mangle]
fn update_admin() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    NFTToken::default()
        .update_admin(recipient)
        .unwrap_or_revert();
}


#[no_mangle]
fn mint() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    let next_id = NFTToken::default().total_supply().clone() + 1u64;
    NFTToken::default()
        .mint(recipient, next_id, token_meta)
        .unwrap_or_revert();
}

#[no_mangle]
fn mint_copies() {
    runtime::revert(ApiError::PermissionDenied);

}

#[no_mangle]
fn burn() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .burn(owner, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .transfer(recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer_one() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let token_ids: Vec<U256> = vec![token_id];
    NFTToken::default()
        .transfer(recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer_from() {
    let sender = runtime::get_named_arg::<Key>("sender");
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .transfer_from(sender, recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn approve() {
    let spender = runtime::get_named_arg::<Key>("spender");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .approve(spender, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn get_approved() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = NFTToken::default().get_approved(owner, token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn install_contract () {
    // Read arguments for the constructor call.
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let meta: Meta = runtime::get_named_arg("meta");
    let admin : Key = runtime::get_caller().into();
    let minter : Key = runtime::get_caller().into();

    let contract_name: String = runtime::get_named_arg("contract_name");
    let contract_package_name = &format!("{}_package_name", contract_name);
    let contract_access_uref = &format!("{}_access_uref", contract_name);
    let contract_version_key = &format!("{}_version", contract_name);
    let contract_key = &format!("{}", contract_name);

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "name" => name,
        "symbol" => symbol,
        "meta" => meta,
        "admin" => admin,
        "minter" => minter
    };


    let (contract_hash, contract_version) = storage::new_contract(
        get_entry_points(),
        None,
        Some(String::from(contract_package_name)),
        Some(String::from(contract_access_uref)),
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(contract_package_name)
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

        let version_uref = storage::new_uref(contract_version);
        runtime::put_key(contract_version_key, version_uref.into());
    
        // Create a named key for the contract hash.
        runtime::put_key(contract_key, contract_hash.into());

}
 
fn upgrade_contract () {
    let contract_name: String = runtime::get_named_arg("contract_name");
    let contract_package_name = &format!("{}_package_name", contract_name);
    let contract_version_key = &format!("{}_version", contract_name);
    let contract_key = &format!("{}", contract_name);

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(contract_package_name)
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );


    // Add a new contract version to the package with the new list of entry points.
    let (contract_hash, contract_version) = storage::add_contract_version(
        package_hash,
        get_entry_points(),
        NamedKeys::default(),
    ); 
    storage::disable_contract_version(
        package_hash,
        runtime::get_key(contract_key)
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert()
            .into(),
    )
    .unwrap_or_revert();

    runtime::put_key(contract_version_key, storage::new_uref(contract_version).into());

    // Create a named key for the contract hash.
    runtime::put_key(contract_key, contract_hash.into());   

}

#[no_mangle]
fn call() {
    // check if update/install
    let contract_name: String = runtime::get_named_arg("contract_name");
    let contract_package_name = &format!("{}_package_name", contract_name);
    match runtime::get_key(contract_package_name) {
        None => {
            install_contract();
        }
        Some(_contract_key) => {
            upgrade_contract();
        }
    }  
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("meta", Meta::cl_type()),
            Parameter::new("admin", Key::cl_type()),  
            Parameter::new("minter", Key::cl_type()),            
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "meta",
        vec![],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner_of",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token_meta",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_admin",
        vec![
            Parameter::new("recipient", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));     
    entry_points.add_entry_point(EntryPoint::new(
        "update_minter",
        vec![
            Parameter::new("recipient", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));    
    entry_points.add_entry_point(EntryPoint::new(
        "update_token_meta",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("token_meta", Meta::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_meta", Meta::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_copies",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_meta", Meta::cl_type()),
            Parameter::new("count", CLType::U32),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_one",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));    
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("sender", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_approved",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_token_by_index",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("index", U256::cl_type()),
        ],
        CLType::Option(Box::new(TokenId::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
