// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_action_rss
{
	pub func: rte_eth_hash_function,
	pub level: u32,
	pub types: u64,
	pub key_len: u32,
	pub queue_num: u32,
	pub key: *const u8,
	pub queue: *const u16,
}

impl Default for rte_flow_action_rss
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_action_rss
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_action_rss {{ func: {:?}, key: {:?}, queue: {:?} }}", self.func, self.key, self.queue)
	}
}
