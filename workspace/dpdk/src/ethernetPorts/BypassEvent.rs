// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BypassEvent
{
	// power button is pushed
	MainPowerOn = 1,
	
	// power supply is being plugged
	AuxillaryPowerOn = 2,
	
	// system shutdown and power supply is left plugged in
	MainPowerOff = 3,
	
	// power supply is being unplugged
	AuxillaryPowerOff = 4,
	
	DisplayOrSetTheWatchdogTimer = 5,
}
