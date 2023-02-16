use ark_bls12_381::{Bls12_381, Parameters};
use ark_crypto_primitives::{crh::TwoToOneCRH, Path, CRH};
use ark_ec::bls12::Bls12;
use ark_groth16::{Groth16, Proof};
use ark_snark::SNARK;

use super::circuit::{LeafHash, MerkleConfig, MyCircuit, Root, TwoToOneHash};

fn build_my_circuit(
    //
    leaf: u8,
    root: Root,
    path: Path<MerkleConfig>,
) -> MyCircuit {
    let mut rng = ark_std::test_rng();

    let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
    let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

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

pub fn gen_proof(
    //
    leaf: u8,
    root: Root,
    path: Path<MerkleConfig>,
) -> Proof<Bls12<Parameters>> {
    let circuit_for_key_gen = build_my_circuit(leaf, root, path);

    let mut rng = ark_std::test_rng();

    let (pk, vk) =
        Groth16::<Bls12_381>::circuit_specific_setup(circuit_for_key_gen, &mut rng).unwrap();

    let circuit = build_my_circuit(leaf, root, path);

    let proof = Groth16::prove(&pk, circuit, &mut rng).unwrap();

    proof
}
