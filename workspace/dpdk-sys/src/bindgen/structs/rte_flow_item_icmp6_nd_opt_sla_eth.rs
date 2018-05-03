// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_item_icmp6_nd_opt_sla_eth
{
	pub type_: u8,
	pub length: u8,
	pub sla: ether_addr,
}

impl Default for rte_flow_item_icmp6_nd_opt_sla_eth
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_item_icmp6_nd_opt_sla_eth
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_item_icmp6_nd_opt_sla_eth {{ sla: {:?} }}", self.sla)
	}
}
