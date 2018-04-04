// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::arrayvec::ArrayVec;
use ::dpdk_sys::*;
use ::devices::DeviceName;
use ::devices::virtualDevices::netVirtualDevices::NetVirtualDeviceDriverName;
use ::devices::virtualDevices::netVirtualDevices::NetVirtualDeviceName;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::strnlen;
use ::libc::timespec;
use ::libc::uint16_t;
use ::libc::uint32_t;
use ::libc::uint8_t;
use ::libc_extra::ffi::isFalse;
use ::libc_extra::ffi::isTrue;
use ::logicalCores::*;
use ::logicalCores::discovery::*;
use ::logicalCores::receiveTransmitQueuePair::*;
use ::memory::DpdkAllocatedMemory;
use ::packetBuffers::PacketBufferPool;
use ::pci::DeviceAddress;
use ::rust_extra::arrays::Array40;
use ::rust_extra::arrays::Array52;
use ::rust_extra::arrays::Array64;
use ::rust_extra::likely;
use ::rust_extra::powersOfTwo::PowerOfTwoSixteenBit;
use ::rust_extra::u31;
use ::rust_extra::u5;
use ::rust_extra::u6;
use ::rust_extra::unlikely;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::de::Error;
use ::serde::ser::Serialize;
use ::serde::ser::Serializer;
use ::std::cmp::min;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt;
use ::std::iter::FromIterator;
use ::std::mem::forget;
use ::std::mem::size_of_val;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ptr::copy;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::slice::from_raw_parts_mut;
use ::std::string::FromUtf8Error;
use ::std::str::SplitN;
use ::std::sync::Arc;
use ::std::sync::Mutex;
use ::syscall_alt::constants::NegativeE;


pub mod burst;


include!("BalanceBondingModeTransmitPolicy.rs");
include!("BondedEthernetPort.rs");
include!("BondedEthernetPortConfiguration.rs");
include!("BondingMode.rs");
include!("BondingSlave.rs");
include!("BypassEvent.rs");
include!("BypassFirmwareVersion.rs");
include!("BypassState.rs");
include!("BypassWatchdogTimeout.rs");
include!("CouldNotDetachError.rs");
include!("CouldNotSetMaximumTransmissionUnitError.rs");
include!("DataCentreBridgingCapability.rs");
include!("DataCentreBridgingPriorityFlowControl.rs");
include!("DeviceReceiveOffloadCapabilities.rs");
include!("DeviceTransmitOffloadCapabilities.rs");
include!("EepromInformation.rs");
include!("EPollInterruptEvent.rs");
include!("EthernetPortInformation.rs");
include!("EthernetPort.rs");
include!("EthernetPortConfiguration.rs");
include!("EthernetPortConfigurationFailureKind.rs");
include!("EthernetPortConfigurationFailures.rs");
include!("EthernetPortConfigurationResult.rs");
include!("EthernetPortEventCallback.rs");
include!("EthernetPortIdentifier.rs");
include!("EthernetPortReceiveModeConfiguration.rs");
include!("EthernetPortTransmitModeConfiguration.rs");
include!("ExtendedStatisticNames.rs");
include!("FilterInputSetField.rs");
include!("FilterInputSetOperation.rs");
include!("FilterOperation.rs");
include!("FilterType.rs");
include!("FlowControl.rs");
include!("FlowDirectorConfiguration.rs");
include!("HashFilter.rs");
include!("HashFilterFunction.rs");
include!("LinkSpeed.rs");
include!("LinkSpeeds.rs");
include!("LinkStatus.rs");
include!("MaximumTransmissionUnitSizeInBytes.rs");
include!("MediaAccessControlAddress.rs");
include!("MultiQueuePacketReceiveMode.rs");
include!("MultiQueuePacketTransmitMode.rs");
include!("NoTransmitQueueBufferErrorCallback.rs");
include!("OrganizationallyUniqueIdentifier.rs");
include!("PacketType.rs");
include!("PacketTypeMask.rs");
include!("QueueIdentifier.rs");
include!("QueueMemoryConfiguration.rs");
include!("ReceiveDataCentreBridgingConfiguration.rs");
include!("ReceiveQueue.rs");
include!("ReceiveQueueConfiguration.rs");
include!("ReceiveQueueDeviceConfiguration.rs");
include!("ReceiveSideScalingHashFunctionConfiguration.rs");
include!("ReceiveSideScalingHashFunctionKeyData.rs");
include!("ReceiveSideScalingHashKeySize.rs");
include!("ReceiveSideScalingOffloadFlowType.rs");
include!("ReceiveSideScalingOffloadFlowTypeSet.rs");
include!("ReceiveSideScalingRetaIndirectionTable.rs");
include!("ReceiveVmdQConfiguration.rs");
include!("ReceiveVmdQDataCentreBridgingConfiguration.rs");
include!("RemoveMediaAccessControlAddressError.rs");
include!("TrafficMirroringRule.rs");
include!("TrafficMirroringRuleNumber.rs");
include!("TransmitAdvancedConfiguration.rs");
include!("TransmitQueue.rs");
include!("TransmitQueueBuffer.rs");
include!("TransmitQueueBufferErrorCallback.rs");
include!("TransmitQueueConfiguration.rs");
include!("TransmitQueueDeviceConfiguration.rs");
include!("TransmitQueueFlags.rs");
include!("UdpTunnelConfiguration.rs");
include!("UdpTunnelType.rs");
include!("UsefulBondingMode.rs");
include!("UnsupportedByHardwareError.rs");
include!("UnsupportedOrFullError.rs");
include!("VirtualLanOffloadFeatures.rs");
