// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate lazy_static;

pub use snarkvm_console_network_environment as environment;
pub use snarkvm_console_network_environment::*;

pub mod testnet3;
pub use testnet3::*;

pub mod prelude {
    pub use crate::{environment::prelude::*, Network};
}

use crate::environment::prelude::*;
use snarkvm_console_algorithms::{Poseidon2, Poseidon4, BHP1024, BHP512};
use snarkvm_console_collections::merkle_tree::MerkleTree;
use snarkvm_console_types::{Field, Group, Scalar};

pub trait Network:
    'static + Environment + Copy + Clone + Debug + Eq + PartialEq + core::hash::Hash + Send + Sync
{
    /// The network ID.
    const ID: u16;

    /// The maximum recursive depth of a value and/or entry.
    /// Note: This value must be strictly less than u8::MAX.
    const MAX_DATA_DEPTH: usize = 32;
    /// The maximum number of values and/or entries in data.
    const MAX_DATA_ENTRIES: usize = 32;

    /// The maximum number of operands in an instruction.
    const MAX_OPERANDS: usize = Self::MAX_DATA_ENTRIES;
    /// The maximum number of instructions in a function.
    const MAX_INSTRUCTIONS: usize = 65536;

    /// The maximum number of inputs per transition.
    const MAX_INPUTS: usize = 8;
    /// The maximum number of outputs per transition.
    const MAX_OUTPUTS: usize = 8;
    /// The maximum number of transitions per transaction.
    const MAX_TRANSITIONS: usize = 16;
    /// The maximum number of transactions per block.
    const MAX_TRANSACTIONS: usize = 65536;

    /// The maximum number of bits in data (must not exceed u16::MAX).
    const MAX_DATA_SIZE_IN_FIELDS: u32 = ((128 * 1024 * 8) / Field::<Self>::size_in_data_bits()) as u32;

    /// Returns the balance commitment domain as a constant field element.
    fn bcm_domain() -> Field<Self>;

    /// Returns the encryption domain as a constant field element.
    fn encryption_domain() -> Field<Self>;

    /// Returns the MAC domain as a constant field element.
    fn mac_domain() -> Field<Self>;

    /// Returns the randomizer domain as a constant field element.
    fn randomizer_domain() -> Field<Self>;

    /// Returns the balance commitment randomizer domain as a constant field element.
    fn r_bcm_domain() -> Field<Self>;

    /// Returns the serial number domain as a constant field element.
    fn serial_number_domain() -> Field<Self>;

    /// Returns the powers of G.
    fn g_powers() -> &'static Vec<Group<Self>>;

    /// Returns the scalar multiplication on the group bases.
    fn g_scalar_multiply(scalar: &Scalar<Self>) -> Group<Self>;

    /// Returns a BHP commitment with an input hasher of 256-bits.
    fn commit_bhp256(input: &[bool], randomizer: &Scalar<Self>) -> Result<Field<Self>>;

    /// Returns a BHP commitment with an input hasher of 512-bits.
    fn commit_bhp512(input: &[bool], randomizer: &Scalar<Self>) -> Result<Field<Self>>;

    /// Returns a BHP commitment with an input hasher of 768-bits.
    fn commit_bhp768(input: &[bool], randomizer: &Scalar<Self>) -> Result<Field<Self>>;

    /// Returns a BHP commitment with an input hasher of 1024-bits.
    fn commit_bhp1024(input: &[bool], randomizer: &Scalar<Self>) -> Result<Field<Self>>;

    /// Returns a Pedersen commitment for the given (up to) 64-bit input and randomizer.
    fn commit_ped64(input: &[bool], randomizer: &Scalar<Self>) -> Result<Group<Self>>;

    /// Returns a Pedersen commitment for the given (up to) 128-bit input and randomizer.
    fn commit_ped128(input: &[bool], randomizer: &Scalar<Self>) -> Result<Group<Self>>;

    /// Returns the BHP hash with an input hasher of 256-bits.
    fn hash_bhp256(input: &[bool]) -> Result<Field<Self>>;

    /// Returns the BHP hash with an input hasher of 512-bits.
    fn hash_bhp512(input: &[bool]) -> Result<Field<Self>>;

    /// Returns the BHP hash with an input hasher of 768-bits.
    fn hash_bhp768(input: &[bool]) -> Result<Field<Self>>;

    /// Returns the BHP hash with an input hasher of 1024-bits.
    fn hash_bhp1024(input: &[bool]) -> Result<Field<Self>>;

    /// Returns the Pedersen hash for a given (up to) 64-bit input.
    fn hash_ped64(input: &[bool]) -> Result<Field<Self>>;

    /// Returns the Pedersen hash for a given (up to) 128-bit input.
    fn hash_ped128(input: &[bool]) -> Result<Field<Self>>;

    /// Returns the Poseidon hash with an input rate of 2.
    fn hash_psd2(input: &[Field<Self>]) -> Result<Field<Self>>;

    /// Returns the Poseidon hash with an input rate of 4.
    fn hash_psd4(input: &[Field<Self>]) -> Result<Field<Self>>;

    /// Returns the Poseidon hash with an input rate of 8.
    fn hash_psd8(input: &[Field<Self>]) -> Result<Field<Self>>;

    /// Returns the extended Poseidon hash with an input rate of 2.
    fn hash_many_psd2(input: &[Field<Self>], num_outputs: u16) -> Vec<Field<Self>>;

    /// Returns the extended Poseidon hash with an input rate of 4.
    fn hash_many_psd4(input: &[Field<Self>], num_outputs: u16) -> Vec<Field<Self>>;

    /// Returns the extended Poseidon hash with an input rate of 8.
    fn hash_many_psd8(input: &[Field<Self>], num_outputs: u16) -> Vec<Field<Self>>;

    /// Returns the Poseidon hash with an input rate of 2 on the affine curve.
    fn hash_to_group_psd2(input: &[Field<Self>]) -> Result<Group<Self>>;

    /// Returns the Poseidon hash with an input rate of 4 on the affine curve.
    fn hash_to_group_psd4(input: &[Field<Self>]) -> Result<Group<Self>>;

    /// Returns the Poseidon hash with an input rate of 8 on the affine curve.
    fn hash_to_group_psd8(input: &[Field<Self>]) -> Result<Group<Self>>;

    /// Returns the Poseidon hash with an input rate of 2 on the scalar field.
    fn hash_to_scalar_psd2(input: &[Field<Self>]) -> Result<Scalar<Self>>;

    /// Returns the Poseidon hash with an input rate of 4 on the scalar field.
    fn hash_to_scalar_psd4(input: &[Field<Self>]) -> Result<Scalar<Self>>;

    /// Returns the Poseidon hash with an input rate of 8 on the scalar field.
    fn hash_to_scalar_psd8(input: &[Field<Self>]) -> Result<Scalar<Self>>;

    /// Returns a Merkle tree with a BHP leaf hasher of 1024-bits and a BHP path hasher of 512-bits.
    #[allow(clippy::type_complexity)]
    fn merkle_tree_bhp<const DEPTH: u8>(
        leaves: &[Vec<bool>],
    ) -> Result<MerkleTree<Self, BHP1024<Self>, BHP512<Self>, DEPTH>>;

    /// Returns a Merkle tree with a Poseidon leaf hasher with input rate of 4 and a Poseidon path hasher with input rate of 2.
    #[allow(clippy::type_complexity)]
    fn merkle_tree_psd<const DEPTH: u8>(
        leaves: &[Vec<Field<Self>>],
    ) -> Result<MerkleTree<Self, Poseidon4<Self>, Poseidon2<Self>, DEPTH>>;

    /// Returns the Poseidon PRF with an input rate of 2.
    fn prf_psd2(seed: &Field<Self>, input: &[Field<Self>]) -> Result<Field<Self>>;

    /// Returns the Poseidon PRF with an input rate of 4.
    fn prf_psd4(seed: &Field<Self>, input: &[Field<Self>]) -> Result<Field<Self>>;

    /// Returns the Poseidon PRF with an input rate of 8.
    fn prf_psd8(seed: &Field<Self>, input: &[Field<Self>]) -> Result<Field<Self>>;
}
