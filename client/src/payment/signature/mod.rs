use ark_crypto_primitives::Error;
use ark_ff::bytes::ToBytes;
use ark_std::hash::Hash;
use ark_std::rand::Rng;

#[cfg(feature = "r1cs")]
pub mod constraints;
#[cfg(feature = "r1cs")]
pub use constraints::*;

pub mod schnorr;

pub trait SignatureScheme {
    type Parameters: Clone + Send + Sync;
    type PublicKey: ToBytes + Hash + Eq + Clone + Default + Send + Sync;
    type SecretKey: ToBytes + Clone + Default;
    type Signature: Clone + Default + Send + Sync;

    fn setup<R: Rng>(rng: &mut R) -> Result<Self::Parameters, Error>;

    fn keygen<R: Rng>(
        pp: &Self::Parameters,
        rng: &mut R,
    ) -> Result<(Self::PublicKey, Self::SecretKey), Error>;

    fn sign<R: Rng>(
        pp: &Self::Parameters,
        sk: &Self::SecretKey,
        message: &[u8],
        rng: &mut R,
    ) -> Result<Self::Signature, Error>;

    fn verify(
        pp: &Self::Parameters,
        pk: &Self::PublicKey,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<bool, Error>;
}
