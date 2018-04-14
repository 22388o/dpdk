// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use super::super::*;
use super::super::super::packetBuffers::rte_mbufEx;
use super::super::super::tldk::devices::Device;
use super::super::super::tldk::devices::TcpDevice;
use super::super::super::tldk::devices::UdpDevice;


include!("TransmitBurst.rs");
include!("TransmitBurstFunction.rs");
include!("TransmitBurstFunctionData.rs");
include!("TransmitBurstQueue.rs");
