// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use super::devices::DeviceName;
use super::devices::virtual_devices::net_virtual_devices::NetVirtualDeviceDriverName;
use super::devices::virtual_devices::net_virtual_devices::NetVirtualDeviceName;
use super::logicalCores::*;
use super::logicalCores::discovery::*;
use super::logicalCores::receiveTransmitQueuePair::*;
use super::memory::DpdkAllocatedMemory;
use super::packetBuffers::PacketBufferPool;
use super::bus::pci::DpdkPciDeviceAddress;


/// Burst packet support.
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
