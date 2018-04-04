// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::rust_extra::unlikely;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::serde::de;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::ser::Serializer;
use ::std::fmt;
use ::std::fmt::Formatter;


include!("Layer4Port.rs");
include!("Layer4Protocol.rs");
