use super::account::{AccountId, AccountInformation, AccountPublicKey, AccountSecretKey};
use super::signature::{schnorr, SignatureScheme};
use super::transaction::Transaction;
use ark_crypto_primitives::crh::{
    injective_map::{PedersenCRHCompressor, TECompressor},
    pedersen, TwoToOneCRH, CRH,
};
use ark_crypto_primitives::merkle_tree::{self, MerkleTree};
use ark_crypto_primitives::Path;
use ark_ed_on_bls12_381::EdwardsProjective;
use ark_std::rand::Rng;
use std::collections::HashMap;

/// Represents transaction amounts and account balances.
#[derive(Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord, Debug)]
pub struct Amount(pub u64);

impl Amount {
    pub fn to_bytes_le(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }
}

/// The parameters that are used in transaction creation and validation.
#[derive(Clone)]
pub struct Parameters {
    pub sig_params: schnorr::Parameters<EdwardsProjective>,
    pub leaf_crh_params: <TwoToOneHash as CRH>::Parameters,
    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl Parameters {
    pub fn sample<R: Rng>(rng: &mut R) -> Self {
        let sig_params = schnorr::Schnorr::setup(rng).unwrap();
        let leaf_crh_params = <LeafHash as CRH>::setup(rng).unwrap();
        let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(rng).unwrap();
        Self {
            sig_params,
            leaf_crh_params,
            two_to_one_crh_params,
        }
    }
}

pub type TwoToOneHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, TwoToOneWindow>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwoToOneWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl pedersen::Window for TwoToOneWindow {
    const WINDOW_SIZE: usize = 128;
    const NUM_WINDOWS: usize = 4;
}

pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, LeafWindow>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LeafWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl pedersen::Window for LeafWindow {
    const WINDOW_SIZE: usize = 144;
    const NUM_WINDOWS: usize = 4;
}

#[derive(Clone)]
pub struct MerkleConfig;
impl merkle_tree::Config for MerkleConfig {
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

/// A Merkle tree containing account information.
pub type AccMerkleTree = MerkleTree<MerkleConfig>;
/// The root of the account Merkle tree.
pub type AccRoot = <TwoToOneHash as TwoToOneCRH>::Output;
/// A membership proof for a given account.
pub type AccPath = Path<MerkleConfig>;

#[derive(Clone)]
pub struct State {
    /// What is the next available account identifier?
    pub next_available_account: Option<AccountId>,
    /// A merkle tree mapping where the i-th leaf corresponds to the i-th account's
    /// information (= balance and public key).
    pub account_merkle_tree: AccMerkleTree,
    /// A mapping from an account's identifier to its information (= balance and public key).
    pub id_to_account_info: HashMap<AccountId, AccountInformation>,
    /// A mapping from a public key to an account's identifier.
    pub pub_key_to_id: HashMap<schnorr::PublicKey<EdwardsProjective>, AccountId>,
    // for project
    //
    pub leaf_crh_params: <TwoToOneHash as CRH>::Parameters,
    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl State {
    /// Create an empty ledger that supports `num_accounts` accounts.
    pub fn new(num_accounts: usize, parameters: Parameters) -> Self {
        let height = ark_std::log2(num_accounts);

        let account_merkle_tree = MerkleTree::blank(
            &parameters.leaf_crh_params,
            &parameters.two_to_one_crh_params,
            height as usize,
        )
        .unwrap();
        let pub_key_to_id = HashMap::with_capacity(num_accounts);
        let id_to_account_info = HashMap::with_capacity(num_accounts);

        let leaf_crh_params = parameters.leaf_crh_params;
        let two_to_one_crh_params = parameters.two_to_one_crh_params;

        Self {
            next_available_account: Some(AccountId(1)),
            account_merkle_tree,
            id_to_account_info,
            pub_key_to_id,
            leaf_crh_params,
            two_to_one_crh_params,
        }
    }

    /// Return the root of the account Merkle tree.
    pub fn root(&self) -> AccRoot {
        self.account_merkle_tree.root()
    }

    /// Create a new account with public key `pub_key`. Returns a fresh account identifier
    /// if there is space for a new account, and returns `None` otherwise.
    /// The initial balance of the new account is 0.
    pub fn register(&mut self, public_key: AccountPublicKey) -> Option<AccountId> {
        self.next_available_account.and_then(|id| {
            // Construct account information for the new account.
            let account_info = AccountInformation {
                public_key,
                balance: Amount(0),
            };
            // Insert information into the relevant accounts.
            self.pub_key_to_id.insert(public_key, id);
            self.account_merkle_tree
                .update(id.0 as usize, &account_info.to_bytes_le())
                .expect("should exist");
            self.id_to_account_info.insert(id, account_info);

            // Increment the next account identifier.
            self.next_available_account
                .as_mut()
                .and_then(|cur| cur.checked_increment())?;
            Some(id)
        })
    }

    /// Samples keys and registers these in the ledger.
    pub fn sample_keys_and_register<R: Rng>(
        &mut self,
        ledger_params: &Parameters,
        rng: &mut R,
    ) -> Option<(AccountId, AccountPublicKey, AccountSecretKey)> {
        let (pub_key, secret_key) =
            schnorr::Schnorr::keygen(&ledger_params.sig_params, rng).unwrap();
        self.register(pub_key).map(|id| (id, pub_key, secret_key))
    }

    /// Update the balance of `id` to `new_amount`.
    /// Returns `Some(())` if an account with identifier `id` exists already, and `None`
    /// otherwise.
    pub fn update_balance(&mut self, id: AccountId, new_amount: Amount) -> Option<()> {
        let tree = &mut self.account_merkle_tree;
        self.id_to_account_info.get_mut(&id).map(|account_info| {
            account_info.balance = new_amount;
            tree.update(id.0 as usize, &account_info.to_bytes_le())
                .expect("should exist");
        })
    }

    /// Update the state by applying the transaction `tx`, if `tx` is valid.
    pub fn apply_transaction(&mut self, pp: &Parameters, tx: &Transaction) -> Option<()> {
        if tx.validate(pp, self) {
            let old_sender_bal = self.id_to_account_info.get(&tx.sender)?.balance;
            let old_receiver_bal = self.id_to_account_info.get(&tx.recipient)?.balance;
            let new_sender_bal = old_sender_bal.checked_sub(tx.amount)?;
            let new_receiver_bal = old_receiver_bal.checked_add(tx.amount)?;
            self.update_balance(tx.sender, new_sender_bal);
            self.update_balance(tx.recipient, new_receiver_bal);
            Some(())
        } else {
            None
        }
    }
}