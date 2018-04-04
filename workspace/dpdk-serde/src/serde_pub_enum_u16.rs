// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_export]
macro_rules! serde_pub_enum_u16
{
	($name:ident { $($variant:ident = $value:expr, )* }) =>
	{
		#[repr(u16)]
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub enum $name
		{
			$($variant = $value,)*
		}

		impl ::serde::Serialize for $name
		{
			fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
			{
				serializer.serialize_u16(*self as u16)
			}
		}

		impl ::serde::Deserialize for $name
		{
			fn deserialize<D: ::serde::Deserializer>(deserializer: D) -> Result<Self, D::Error>
			{
				struct Visitor;

				impl ::serde::de::Visitor for Visitor
				{
					type Value = $name;

					fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
					{
						formatter.write_str("positive integer")
					}

					fn visit_u16<E: ::serde::de::Error>(self, value: u16) -> Result<$name, E>
					{
						// Rust does not come with a simple way of converting a number to an enum, so use a big `match`.
						match value
						{
							$( $value => Ok($name::$variant), )*
							_ => Err(E::custom(format!("unknown {} value: {}", stringify!($name), value))),
						}
					}
				}

				deserializer.deserialize_u16(Visitor)
			}
		}
	}
}
