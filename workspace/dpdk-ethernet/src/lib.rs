// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(renamed_and_removed_lints)]
#![cfg_attr(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")), feature(const_fn, core_intrinsics, step_trait, try_from))]
#![deny(missing_docs)]


//! #dpdk-global-allocator
//! A wrapper around DPDK's ethernet device driver functionality.
//!
//! Whilst DPDK is supported for Linux on AArch64, ARM v7, PowerPC 64-bit (recent) and x86-64, and FreeBSD on x86-64, only Linux x86-64 will compile.
//!
//! Some work has been done to try to maintain compatibility with FreeBSD and other architectures but this is not maintained yet.


#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] include!("lib.cfg.rs");
