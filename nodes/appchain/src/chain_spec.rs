use uonenet_appchain_runtime::{
	AccountId, BabeConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	Signature, SudoConfig, SystemConfig, BABE_GENESIS_EPOCH_CONFIG, WASM_BINARY,
};
use sc_service::{ChainType, Properties};
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};

use beefy_primitives::crypto::AuthorityId as BeefyId;
use hex_literal::hex;
use uonenet_appchain_runtime::{
	currency::UON, opaque::SessionKeys, Balance, BeefyConfig, ImOnlineConfig,
	OctopusAppchainConfig, SessionConfig, StakingConfig,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use pallet_staking::StakerStatus;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::crypto::UncheckedInto;
use sp_runtime::Perbill;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	beefy: BeefyId,
	octopus: OctopusId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<BeefyId>(seed),
		get_from_seed::<OctopusId>(seed),
	)
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	properties
}

pub fn staging_tesnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UON", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Unique One Staging AppChain",
		// ID
		"uonenet_staging_appchain",
		ChainType::Live,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
				hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5GpLLa9frPJepnquJosVBCc9kH74SEaFf1LRgrRcN3uskM8c
						hex!["d2374ae2756d053da9f4c8f5ed36fd381326b3b73986a1a0cd6fcc19254b3924"].into(),
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].into(),
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].unchecked_into(),
						// 5HgtmpdnGgKJ1Wia5XTsFPGw3JYjKXFrrCXQq7TparBwLzJi
						hex!["f8c6bff47eec7c2fe49be0b0d6c638757ff19562ea9ae1a76ed63d22f979b79c"].unchecked_into(),
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].unchecked_into(),
						// 5CtT5c71rGrnnj3VJqvNqvJQKeBwyh73jGm4EduQ32igi1ym
						hex!["02e23d37735cebc72732f31c86cca3a70f2c5d3087854ac94fe5aa1fec1bbb0e18"].unchecked_into(),
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].unchecked_into(),

					),
					(
						// 5CZmRtq8ptpiBSh93rEwXSJmMAgXLHzSpDhZWMHDGGek8cV9
						hex!["162a24df69ad68f581a49b1374f8ec6bca2f9ab93f1d59495f4d76e72fbbdf1a"].into(),
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].into(),
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].unchecked_into(),
						// 5CYZaymcu6jdq3bzsch7dgfsQdG347VEjPNbygwrtUXqGqmu
						hex!["153f07c39a47483fbdc7be025a78139fec54e350894fbaf567125135571c0e66"].unchecked_into(),
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].unchecked_into(),
						// 5CvRworpEZfn7FYeeXJmTHBVoEZvow3HRDxWCEZ5cN9dJKF6
						hex!["036ce9ccdf3a1a4a6f5e65c44cd2453640ba4cee5a148e6362baa5d2129aed94fb"].unchecked_into(),
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].unchecked_into(),

					),
				],
				// Pre-funded accounts
				vec![
					// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
					hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),
				],
				// Appchain Id
				"uonenet_staging_appchain",
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uonenet-staging-appchain"),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn development_tesnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UON", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Unique One Development AppChain",
		// ID
		"uonenet_development_appchain",
		ChainType::Live,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
				hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5CXF4c7rRuX3NfEbToR32ScG3mDNvJ1aFGVg7Wd6YZS5cU37
						hex!["143d6dbd1fa1906ef35a1308afd7940cf8e987271d47a8c196062eca1ef87a5b"].into(),
						// 5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
						hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"].into(),
						// 5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
						hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"]
							.unchecked_into(),
						// 5H83G9CMm7wPYq6FeYqrn7ueVBWBvK37xdZYamtm8re2LYn3
						hex!["dfb839beaf6fe750ca87b9059161d43f2682a6c3a0ac765f1e5054063ed9903b"]
							.unchecked_into(),
						// 5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
						hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"]
							.unchecked_into(),
						// KW7hbC4ZzNJEjkofpAhxC51PRHxPxSr6RZ8NG4UyerChmQH3E
						hex!["02d337069cb73bcefafc4e35e5189ad62932e4f2ee3f985b6bbff654cb68017ff1"]
							.unchecked_into(),
						// 5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
						hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"]
							.unchecked_into(),

					),
				],
				// Pre-funded accounts
				vec![
					// 0x9c5e883c0a7795c81d354aa2d596364e71f4bb07 - H160
					// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
					hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),
				],
				// Appchain Id
				"uonenet_development_appchain",
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uonenet-development-appchain"),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UON", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Unique One Local AppChain",
		// ID
		"uonenet_local_appchain",
		ChainType::Local,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				// Appchain Id
				"uonenet_local_appchain",
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uonenet-local-appchain"),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_development_tesnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UON", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Unique One Local Development AppChain",
		// ID
		"uonenet_local_development_appchain",
		ChainType::Development,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				// Appchain Id
				"uonenet_local_development_appchain",
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uonenet-local-development-appchain"),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		BeefyId,
		OctopusId,
	)>,
	endowed_accounts: Vec<AccountId>,
	appchain_id: &str,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 1_000_000 * UON;
	const STASH: Balance = 100 * UON;
	const OCTOPUS_STASH: Balance = 10_000_000_000_000_000;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		beefy: BeefyConfig { authorities: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			..Default::default()
		},
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					H160::from_str("7D4a82306Eb4de7C7B1D686AFC56b1E7999ba7F9")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0xD3C21BCECCEDA1000000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		},
		ethereum: EthereumConfig {},
		octopus_appchain: OctopusAppchainConfig {
			appchain_id: appchain_id.to_string(),
			relay_contract: "dev-oct-relay.testnet".to_string(),
			validators: initial_authorities.iter().map(|x| (x.0.clone(), OCTOPUS_STASH)).collect(),
			asset_id_by_name: vec![("usdc.testnet".to_string(), 0)],
		},
	}
}