// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Iterator over all logical cores.
///
/// Create using `AllLogicalCoreIterator::default()`.
#[derive(Debug)]
pub struct AllLogicalCoreIterator
{
	index: usize,
}

impl Default for AllLogicalCoreIterator
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			index: 0,
		}
	}
}

impl Iterator for AllLogicalCoreIterator
{
	type Item = LogicalCore;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		while self.index < LogicalCore::Maximum
		{
			if LogicalCore::is_invalid(self.index as u16)
			{
				self.index += 1;
				continue
			}
			
			return Some(LogicalCore(self.index as u16))
		}
		None
	}
}
