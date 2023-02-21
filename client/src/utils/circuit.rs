use ark_crypto_primitives::crh::injective_map::constraints::{
    PedersenCRHCompressorGadget, TECompressorGadget,
};
use ark_crypto_primitives::crh::{
    injective_map::{PedersenCRHCompressor, TECompressor},
    pedersen,
};
use ark_crypto_primitives::crh::{TwoToOneCRH, TwoToOneCRHGadget};
use ark_crypto_primitives::merkle_tree::{Config, MerkleTree, Path};
use ark_crypto_primitives::{CRHGadget, CRH};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective};

pub type TwoToOneHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, TwoToOneWindow>;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwoToOneWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl pedersen::Window for TwoToOneWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 128;
}

pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, LeafWindow>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LeafWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl pedersen::Window for LeafWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 144;
}

pub type TwoToOneHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    TwoToOneWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    LeafWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashParamsVar = <LeafHashGadget as CRHGadget<LeafHash, ConstraintF>>::ParametersVar;
pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::ParametersVar;

pub type ConstraintF = ark_ed_on_bls12_381::Fq;

#[derive(Clone)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    // Our Merkle tree relies on two hashes: one to hash leaves, and one to hash pairs
    // of internal nodes.
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

/// A Merkle tree containing account information.
// pub type SimpleMerkleTree = MerkleTree<MerkleConfig>;
/// The root of the account Merkle tree.
pub type Root = <TwoToOneHash as TwoToOneCRH>::Output;
/// A membership proof for a given account.
pub type SimplePath = Path<MerkleConfig>;

use ark_crypto_primitives::merkle_tree::constraints::PathVar;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

// use ark_std::rand::rngs::StdRng;

/// The R1CS equivalent of the the Merkle tree root.
pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::OutputVar;

/// The R1CS equivalent of the the Merkle tree path.
pub type SimplePathVar = PathVar<MerkleConfig, LeafHashGadget, TwoToOneHashGadget, ConstraintF>;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct MyCircuit {
    // These are constants that will be embedded into the circuit
    pub leaf_crh_params: <LeafHash as CRH>::Parameters,
    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,

    // These are the public inputs to the circuit.
    pub root: Root,

    // This is the private witness to the circuit.
    pub leaf: u8,
    pub authentication_path: Option<SimplePath>,
}

impl ConstraintSynthesizer<ConstraintF> for MyCircuit {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // First, we allocate the public inputs
        let root = RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.root))?;

        // Then, we allocate the public parameters as constants:
        let leaf_crh_params = LeafHashParamsVar::new_constant(cs.clone(), &self.leaf_crh_params)?;
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        // Finally, we allocate our path as a private witness variable:
        let leaf = UInt8::new_witness(ark_relations::ns!(cs, "leaf_var"), || Ok(self.leaf))?;
        let path = SimplePathVar::new_witness(ark_relations::ns!(cs, "path_var"), || {
            Ok(self.authentication_path.as_ref().unwrap())
        })?;

        let leaf_bytes = vec![leaf; 1];

        let is_member = path.verify_membership(
            &leaf_crh_params,
            &two_to_one_crh_params,
            &root,
            &leaf_bytes.as_slice(),
        )?;

        is_member.enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}
