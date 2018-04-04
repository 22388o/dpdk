// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveVmdQConfiguration
{
}

impl ReceiveVmdQConfiguration
{
	fn as_rte_eth_vmdq_rx_conf(&self) -> rte_eth_vmdq_rx_conf
	{
		rte_eth_vmdq_rx_conf::default()
	}
}

impl Default for ReceiveVmdQConfiguration
{
	fn default() -> Self
	{
		ReceiveVmdQConfiguration
		{
		}
	}
}
