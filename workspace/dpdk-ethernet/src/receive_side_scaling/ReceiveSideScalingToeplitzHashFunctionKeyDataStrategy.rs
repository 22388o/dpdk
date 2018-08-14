// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive side scaling toeplitz hash function key data strategy.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	/// Use fixed values.
	Fixed
	{
		/// For an ethernet device that supports 40-byte long hash keys.
		forty: ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes,
		
		/// For an ethernet device that supports 52-byte long hash keys.
		fifty_two: ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes,
	},
	
	/// Generate a Layer 4 hash key using the number of queues as an input.
	ForNumberOfQueues,
}

impl Default for ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	#[inline(always)]
	fn default() -> Self
	{
		ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy::ForNumberOfQueues
	}
}

impl ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	/// Creates an array of receive side scaling bytes.
	#[inline(always)]
	pub fn create<'a>(&'a self, ethernet_device_capabilities: &EthernetDeviceCapabilities, number_of_receive_queues: ReceiveNumberOfQueues) -> Option<ReceiveSideScalingHashKey<'a>>
	{
		use self::ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy::*;
		use self::Cow::*;
		use self::Either::*;
		
		use self::ReceiveSideScalingHashKeySize::*;
		let receive_side_scaling_hash_key_size = if let Some(receive_side_scaling_hash_key_size) = ethernet_device_capabilities.receive_side_scaling_hash_key_size()
		{
			receive_side_scaling_hash_key_size
		}
		else
		{
			return None
		};
		
		let key = match *self
		{
			Fixed { ref forty, ref fifty_two } =>
			{
				match receive_side_scaling_hash_key_size
				{
					Forty => ReceiveSideScalingHashKey(Left(Borrowed(forty))),
					
					FiftyTwo => ReceiveSideScalingHashKey(Right(Borrowed(fifty_two))),
				}
			}
			
			ForNumberOfQueues =>
			{
				match receive_side_scaling_hash_key_size
				{
					Forty => ReceiveSideScalingHashKey(Left(Owned(ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes::for_layer_4_one_way_for_number_of_queues(number_of_receive_queues)))),
					
					FiftyTwo => ReceiveSideScalingHashKey(Right(Owned(ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes::for_layer_4_one_way_for_number_of_queues(number_of_receive_queues)))),
				}
			}
		};
		
		Some(key)
	}
}
