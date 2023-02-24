use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::{crh::TwoToOneCRH, Path, CRH};
use ark_groth16::Groth16;
use ark_serialize::CanonicalSerialize;
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

pub fn gen_proof_and_vk(
    //
    leaf: u8,
    root: Root,
    path: Path<MerkleConfig>,
) -> (Vec<u8>, Vec<u8>) {
    // let leaf = (leaf + 1) * 10;

    log::info!("[!] generating proof and VerifyingKey...");

    let circuit_for_key_gen = build_my_circuit(leaf, root, path.clone());

    let mut rng = ark_std::test_rng();

    let (pk, vk) =
        Groth16::<Bls12_381>::circuit_specific_setup(circuit_for_key_gen, &mut rng).unwrap();

    let mut ser_vk = Vec::<u8>::default();
    vk.serialize(&mut ser_vk).unwrap();

    // log::info!("5555 root: {:?}", root.0);
    // log::info!("5555 leaf: {:?}", leaf);
    // log::info!("5555 path: {:?}", path.leaf_sibling_hash.0);

    let circuit = build_my_circuit(leaf, root, path.clone());
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();
    let mut ser_proof = Vec::<u8>::default();
    proof.serialize(&mut ser_proof).unwrap();

    (ser_vk, ser_proof)
}
