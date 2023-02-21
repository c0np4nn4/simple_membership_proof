use crate::{common::*, SimpleMerkleTree};
use crate::{Root, SimplePath};
use ark_bls12_381::Parameters;
use ark_crypto_primitives::crh::{TwoToOneCRH, TwoToOneCRHGadget, CRH};
use ark_crypto_primitives::merkle_tree::constraints::PathVar;
use ark_ec::bls12::Bls12;
use ark_groth16::{Proof, VerifyingKey};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use ark_std::rand::rngs::StdRng;
use serde::de::IntoDeserializer;
use serde::{Serialize, Deserialize};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

use serde_big_array::BigArray;
use ark_ff::BigInteger256;
use ark_ff::Fp256;

// (You don't need to worry about what's going on in the next two type definitions,
// just know that these are types that you can use.)

/// The R1CS equivalent of the the Merkle tree root.
pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::OutputVar;

/// The R1CS equivalent of the the Merkle tree path.
pub type SimplePathVar =
    PathVar<crate::MerkleConfig, LeafHashGadget, TwoToOneHashGadget, ConstraintF>;

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

#[derive(Debug, Serialize, Deserialize)]
struct cr {
    #[serde(with = "BigArray")]
    arr: [u64; 4],
}

#[test]
fn merkle_tree_groht16() {
    use ark_bls12_381::Bls12_381;
    use ark_groth16::Groth16;
    use ark_snark::SNARK;

    fn build_my_circuit(leaf: u8, leaf_idx: usize) -> MyCircuit {
        let mut rng = ark_std::test_rng();

        let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
        let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

        let tree = crate::SimpleMerkleTree::new(
            &leaf_crh_params,
            &two_to_one_crh_params,
            &[10u8, 20u8, 30u8, 40u8, 50u8, 60u8, 70u8, 80u8], // the i-th entry is the i-th leaf.
        )
        .unwrap();

        // public input
        let root = tree.root();

        // private input
        let leaf = leaf;
        let path = tree.generate_proof(leaf_idx).unwrap();

        MyCircuit {
            // constants
            leaf_crh_params,
            two_to_one_crh_params,

            // public input
            root,

            // private inputs (witness)
            leaf,
            authentication_path: Some(path),
        }
    }

    let circuit_for_key_gen = build_my_circuit(20, 1);

    let mut rng = ark_std::test_rng();

    let (pk, vk) =
        Groth16::<Bls12_381>::circuit_specific_setup(circuit_for_key_gen, &mut rng).unwrap();

    let circuit = build_my_circuit(20, 1);

    // let public_input = [circuit.root];
    let proof = Groth16::prove(&pk, circuit.clone(), &mut rng).unwrap();

    // --------------------------

    // serialize "proof"
    let mut proof_bytes: Vec<u8> = vec![];
    let ser_proof = proof.serialize(&mut proof_bytes).unwrap();    

    // serialize "vk"
    let mut vk_bytes: Vec<u8> = vec![];
    let ser_vk = vk.serialize(&mut vk_bytes).unwrap();

    // serialize "public_input"
    // let ser_circuit_root = convert_u64_array_to_u8_vec(circuit.root.0.0);
    
    let ser_circuit_root = cr {
        arr: circuit.root.0.0,
    };

    let ser_public_input = serde_json::to_string(&ser_circuit_root).unwrap();
    let a = circuit.root.1;

    // deserializing all the parameters
    let des_proof = Proof::<Bls12_381>::deserialize(proof_bytes.as_slice()).unwrap();
    let des_vk = VerifyingKey::<Bls12_381>::deserialize(vk_bytes.as_slice()).unwrap();
    let des_public_input_1: [u64; 4] = serde_json::from_str(&ser_public_input).unwrap();
    let des_public_input_2 = BigInteger256(des_public_input_1);
    let des_public_input_3 = Fp256(des_public_input_2, circuit.root.1);
    let des_public_input = [des_public_input_3];

    let b = serde_json::to_string(&a).unwrap();

    let valid_proof = Groth16::verify(&des_vk, &des_public_input, &des_proof).unwrap();

    println!("valid: {:?}", valid_proof);

    assert!(valid_proof);
}