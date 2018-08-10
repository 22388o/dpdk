// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive side scaling toeplitz hash function key data strategy.
#[derive(Deserialize, Serialize)]
pub enum ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	/// Use fixed values.
	Fixed(ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes, ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes),
	
	/// Generate a Layer 4 hash key using the number of queues as an input.
	ForNumberOfQueues,
}

impl ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	/// Generates a vector of bytes.
	#[inline(always)]
	pub fn generate(&self, hash_key_size: u8, number_of_receive_queues: u16) -> Box<[u8]>
	{
		use self::ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy::*;
		
		const SomePollModeDriversSuchAsMellanox5ReportZeroInsteadOfForty: u8 = 0;
		const _40Bytes: u8 = 40;
		const _52Bytes: u8 = 52;
		
		let vec = match *self
		{
			Fixed(ref _40_bytes, ref _52_bytes) =>
			{
				match hash_key_size
				{
					SomePollModeDriversSuchAsMellanox5ReportZeroInsteadOfForty | _40Bytes => _40_bytes.to_vec(),
					
					_52Bytes => _52_bytes.to_vec(),
					
					_ => panic!("Invalid hash_key_size, '{}'", hash_key_size),
				}
			}
			
			ForNumberOfQueues =>
			{
				match hash_key_size
				{
					SomePollModeDriversSuchAsMellanox5ReportZeroInsteadOfForty | _40Bytes => ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes::for_layer_4_one_way_for_number_of_queues(number_of_receive_queues).to_vec(),
					
					_52Bytes => ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes::for_layer_4_one_way_for_number_of_queues(number_of_receive_queues).to_vec(),
					
					_ => panic!("Invalid hash_key_size, '{}'", hash_key_size),
				}
			}
		};
		vec.into_boxed_slice()
	}
}
