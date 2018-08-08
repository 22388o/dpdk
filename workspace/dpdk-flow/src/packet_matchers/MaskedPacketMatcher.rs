// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A pattern matcher which uses a mask.
pub trait MaskedPacketMatcher: PacketMatcher
{
	/// DPDK type.
	type DpdkType;
	
	/// Default mask.
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType;
}
