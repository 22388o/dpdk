// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_avp_req_id
{
	RTE_AVP_REQ_UNKNOWN = 0,
	RTE_AVP_REQ_CHANGE_MTU = 1,
	RTE_AVP_REQ_CFG_NETWORK_IF = 2,
	RTE_AVP_REQ_CFG_DEVICE = 3,
	RTE_AVP_REQ_SHUTDOWN_DEVICE = 4,
	RTE_AVP_REQ_MAX = 5,
}
