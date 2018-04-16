// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use self::subclasses::*;
use super::*;
use super::super::devices::DeviceName;
use super::super::domain::NetworkInterfaceName;
use super::super::ethernetPorts::EthernetPort;
use super::super::logicalCores::NumaSocketId;
use super::super::logicalCores::active::Active;
use super::super::logicalCores::active::LogicalCoresActive;


/// PCI device subclasses.
pub mod subclasses;


include!("DpdkPciBus.rs");
include!("DpdkPciDevice.rs");
include!("DpdkPciDeviceAddress.rs");
include!("DpdkPciDeviceAddressStringParseError.rs");
include!("DpdkPciDriver.rs");
include!("DpdkPciDriverFlags.rs");
include!("DpdkPciInputOutputPort.rs");
include!("DpdkPciMappedResource.rs");
include!("IndirectPciDeviceIdentifier.rs");
include!("PciDevice.rs");
include!("PciDeviceClass.rs");
include!("PciDeviceIdentifier.rs");
include!("PciDeviceType.rs");
include!("PciKernelDriver.rs");
include!("PciVendorIdentifier.rs");
include!("SupportedDevice.rs");
include!("SupportedDriverIdentifier.rs");
include!("Unbind.rs");
