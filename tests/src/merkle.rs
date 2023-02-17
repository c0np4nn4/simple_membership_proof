use crate::{common::*, SimpleMerkleTree};
use crate::{Root, SimplePath};
use ark_crypto_primitives::crh::{TwoToOneCRH, TwoToOneCRHGadget, CRH};
use ark_crypto_primitives::merkle_tree::constraints::PathVar;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use ark_std::rand::rngs::StdRng;

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

    let public_input = [circuit.root];
    let proof = Groth16::prove(&pk, circuit, &mut rng).unwrap();

    // --------------------------

    let valid_proof = Groth16::verify(&vk, &public_input, &proof).unwrap();

    println!("valid: {:?}", valid_proof);

    assert!(valid_proof)
}

