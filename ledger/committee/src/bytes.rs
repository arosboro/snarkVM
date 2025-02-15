// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> FromBytes for Committee<N> {
    /// Reads the committee from the buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 0 {
            return Err(error("Invalid committee version"));
        }

        // Read the starting round.
        let starting_round = u64::read_le(&mut reader)?;
        // Read the number of members.
        let num_members = u32::read_le(&mut reader)?;
        // Read the members.
        let mut members = IndexMap::with_capacity(num_members as usize);
        for _ in 0..num_members {
            // Read the address.
            let member = Address::read_le(&mut reader)?;
            // Read the stake.
            let stake = u64::read_le(&mut reader)?;
            // Insert the member and stake.
            members.insert(member, stake);
        }
        // Read the total stake.
        let total_stake = u64::read_le(&mut reader)?;
        // Construct the committee.
        let committee = Self::new(starting_round, members).map_err(|e| error(e.to_string()))?;
        // Ensure the total stake matches.
        match committee.total_stake() == total_stake {
            true => Ok(committee),
            false => Err(error("Invalid total stake in committee during deserialization")),
        }
    }
}

impl<N: Network> ToBytes for Committee<N> {
    /// Writes the committee to the buffer.
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        // Write the version.
        0u8.write_le(&mut writer)?;
        // Write the starting round.
        self.starting_round.write_le(&mut writer)?;
        // Write the number of members.
        u32::try_from(self.members.len()).map_err(|e| error(e.to_string()))?.write_le(&mut writer)?;
        // Write the members.
        for (address, stake) in &self.members {
            // Write the address.
            address.write_le(&mut writer)?;
            // Write the stake.
            stake.write_le(&mut writer)?;
        }
        // Write the total stake.
        self.total_stake.write_le(&mut writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_bytes() {
        let rng = &mut TestRng::default();

        for expected in crate::test_helpers::sample_committees(rng) {
            // Check the byte representation.
            let expected_bytes = expected.to_bytes_le().unwrap();
            assert_eq!(expected, Committee::read_le(&expected_bytes[..]).unwrap());
            assert!(Committee::<CurrentNetwork>::read_le(&expected_bytes[1..]).is_err());
        }
    }
}
