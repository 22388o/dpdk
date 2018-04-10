// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Allocate.
#[inline(always)]
#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8
{
	let alignment = if _align < MinimumAlignment
	{
		DefaultAlignment
	}
	else
	{
		_align as c_uint
	};
	
	unsafe { rte_malloc(null(), size, alignment) as *mut u8	}
}
