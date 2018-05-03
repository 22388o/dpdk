// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_security_ipsec_xform
{
	pub spi: u32,
	pub salt: u32,
	pub options: rte_security_ipsec_sa_options,
	pub direction: rte_security_ipsec_sa_direction,
	pub proto: rte_security_ipsec_sa_protocol,
	pub mode: rte_security_ipsec_sa_mode,
	pub tunnel: rte_security_ipsec_tunnel_param,
	pub esn_soft_limit: u64,
}

impl Default for rte_security_ipsec_xform
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_security_ipsec_xform
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_security_ipsec_xform {{ options: {:?}, direction: {:?}, proto: {:?}, mode: {:?}, tunnel: {:?} }}", self.options, self.direction, self.proto, self.mode, self.tunnel)
	}
}
