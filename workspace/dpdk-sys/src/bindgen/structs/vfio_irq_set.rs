// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct vfio_irq_set
{
	pub argsz: __u32,
	pub flags: __u32,
	pub index: __u32,
	pub start: __u32,
	pub count: __u32,
	pub data: IncompleteArrayField<__u8>,
}

impl Default for vfio_irq_set
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for vfio_irq_set
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "vfio_irq_set {{ data: {:?} }}", self.data)
	}
}
