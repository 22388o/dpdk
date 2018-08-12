// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EthernetPortIdentifier(u16);

impl TryFrom<u16> for EthernetPortIdentifier
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value >= Self::Maximum as u16
		{
			Err(())
		}
		else
		{
			Ok(EthernetPortIdentifier(value))
		}
	}
}

impl TryFrom<usize> for EthernetPortIdentifier
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if value >= Self::Maximum as usize
		{
			Err(())
		}
		else
		{
			Ok(EthernetPortIdentifier(value as u16))
		}
	}
}

impl Into<u16> for EthernetPortIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<usize> for EthernetPortIdentifier
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Step for EthernetPortIdentifier
{
	#[inline(always)]
	fn steps_between(start: &Self, end: &Self) -> Option<usize>
	{
		u16::steps_between(&start.0, &end.0)
	}
	
	#[inline(always)]
	fn replace_one(&mut self) -> Self
	{
		replace(self, EthernetPortIdentifier(1))
	}
	
	#[inline(always)]
	fn replace_zero(&mut self) -> Self
	{
		replace(self, EthernetPortIdentifier(0))
	}
	
	#[inline(always)]
	fn add_one(&self) -> Self
	{
		EthernetPortIdentifier(self.0.add_one())
	}
	
	#[inline(always)]
	fn sub_one(&self) -> Self
	{
		EthernetPortIdentifier(self.0.sub_one())
	}
	
	#[inline(always)]
	fn add_usize(&self, n: usize) -> Option<Self>
	{
		self.0.add_usize(n).map(|value| EthernetPortIdentifier(value))
	}
}

impl Add<u16> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u16) -> Self::Output
	{
		EthernetPortIdentifier(min(self.0.saturating_add(rhs), Self::Maximum as u16))
	}
}

impl Add<usize> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: usize) -> Self::Output
	{
		EthernetPortIdentifier(min(self.0.saturating_add(rhs as u16), Self::Maximum as u16))
	}
}

impl AddAssign<u16> for EthernetPortIdentifier
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u16)
	{
		*self = (*self).add(rhs)
	}
}

impl AddAssign<usize> for EthernetPortIdentifier
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: usize)
	{
		*self = (*self).add(rhs)
	}
}

impl Sub<u16> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: u16) -> Self::Output
	{
		EthernetPortIdentifier(self.0.saturating_sub(rhs))
	}
}

impl Sub<usize> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: usize) -> Self::Output
	{
		EthernetPortIdentifier(self.0.saturating_sub(rhs as u16))
	}
}

impl SubAssign<u16> for EthernetPortIdentifier
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: u16)
	{
		self.0 = self.0.saturating_sub(rhs)
	}
}

impl SubAssign<usize> for EthernetPortIdentifier
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: usize)
	{
		self.0 = self.0.saturating_sub(rhs as u16)
	}
}

impl EthernetPortIdentifier
{
	/// Maximum.
	pub const Maximum: usize = RTE_MAX_ETHPORTS;
}
