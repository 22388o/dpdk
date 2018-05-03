// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait to unify ArrayVec and Vec when dealing with NonNull pointers.
pub trait NonNullUnifiedArrayVecAndVec<T> : UnifiedArrayVecAndVec<NonNull<T>>
{
	#[inline(always)]
	fn to_ffi_data(&mut self, start_from_index: usize) -> (*mut *mut T, usize)
	{
		let length = self.length();
		debug_assert!(length >= start_from_index, "length '{}' is less than start_from_index '{}'", length, start_from_index);
		
		let number_of_potential_packets = length - start_from_index;
		debug_assert_ne!(number_of_potential_packets, 0, "number_of_potential_packets is zero");
		
		let pointer: *mut *mut T = unsafe { transmute(self.mutable_pointer_at_length(start_from_index) as *mut NonNull<T>) };
		
		(pointer, number_of_potential_packets)
	}
	
	#[inline(always)]
	fn to_ffi_data_u16(&mut self, start_from_index: usize) -> (*mut *mut T, u16)
	{
		let (pointer, number_of_potential_packets) = self.to_ffi_data(start_from_index);
		debug_assert!(number_of_potential_packets <= ::std::u16::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u16::MAX", number_of_potential_packets, ::std::u16::MAX);
		(pointer, number_of_potential_packets as u16)
	}
	
	#[inline(always)]
	fn to_ffi_data_u32(&mut self, start_from_index: usize) -> (*mut *mut T, u32)
	{
		let (pointer, number_of_potential_packets) = self.to_ffi_data(start_from_index);
		debug_assert!(number_of_potential_packets <= ::std::u32::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u32::MAX", number_of_potential_packets, ::std::u32::MAX);
		(pointer, number_of_potential_packets as u32)
	}
	
	#[inline(always)]
	fn from_ffi_data(&mut self) -> (*mut *mut T, usize)
	{
		let length = self.length();
		let maximum_capacity = self.maximum_capacity();
		debug_assert!(maximum_capacity >= length, "maximum_capacity '{}' is less than length '{}'", maximum_capacity, length);
		
		let number_of_potential_packets = maximum_capacity - length;
		debug_assert_ne!(number_of_potential_packets, 0, "number_of_potential_packets is zero");
		
		let pointer: *mut *mut T = unsafe { transmute(self.mutable_pointer_at_length(length) as *mut NonNull<T>) };
		
		(pointer, number_of_potential_packets)
	}
	
	#[inline(always)]
	fn from_ffi_data_u16(&mut self) -> (*mut *mut T, u16)
	{
		let (pointer, number_of_potential_packets) = self.from_ffi_data();
		debug_assert!(number_of_potential_packets <= ::std::u16::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u16::MAX", number_of_potential_packets, ::std::u16::MAX);
		(pointer, number_of_potential_packets as u16)
	}
	
	#[inline(always)]
	fn from_ffi_data_u32(&mut self) -> (*mut *mut T, u32)
	{
		let (pointer, number_of_potential_packets) = self.from_ffi_data();
		debug_assert!(number_of_potential_packets <= ::std::u32::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u32::MAX", number_of_potential_packets, ::std::u32::MAX);
		(pointer, number_of_potential_packets as u32)
	}
}

impl NonNullUnifiedArrayVecAndVec<T> for Vec<T>
{
}

/// A trait to unify ArrayVec and Vec.
pub trait UnifiedArrayVecAndVec<T>
{
	/// Obtain maximum capacity.
	#[inline(always)]
	fn maximum_capacity(&self) -> usize;
	
	/// Obtain length.
	#[inline(always)]
	fn length(&self) -> usize;
	
	/// Should return a pointer to just after the last element.
	#[inline(always)]
	fn mutable_pointer_at_length(&mut self, length: usize) -> *mut T;
	
	/// Set length.
	#[inline(always)]
	fn set_length(&mut self, length: usize);
	
	/// Truncates without dropping any members (sets length to zero).
	#[inline(always)]
	fn truncate_without_drop(&mut self)
	{
		self.set_length(0)
	}
}

impl<T> UnifiedArrayVecAndVec<T> for Vec<T>
{
	#[inline(always)]
	fn maximum_capacity(&self) -> usize
	{
		self.capacity()
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.len()
	}
	
	#[inline(always)]
	fn mutable_pointer_at_length(&mut self, length: usize) -> *mut T
	{
		unsafe { self.get_unchecked_mut(length) as *mut T }
	}
	
	#[inline(always)]
	fn set_length(&mut self, length: usize)
	{
		unsafe { self.set_len(length) };
	}
}

macro_rules! implement_for_array_vec
{
	($len: expr) =>
	{
		impl<T> UnifiedArrayVecAndVec<T> for ArrayVec<[T; $len]>
		{
			#[inline(always)]
			fn maximum_capacity(&self) -> usize
			{
				$len
			}
			
			#[inline(always)]
			fn length(&self) -> usize
			{
				self.len()
			}
			
			#[inline(always)]
			fn mutable_pointer_at_length(&mut self, length: usize) -> *mut T
			{
				unsafe { self.get_unchecked_mut(length) as *mut T }
			}
			
			#[inline(always)]
			fn set_length(&mut self, length: usize)
			{
				unsafe { self.set_len(length) };
			}
		}
		
		impl NonNullUnifiedArrayVecAndVec<T> for ArrayVec<[T; $len]>
		{
		}
	}
}
implement_for_array_vec!(0);
implement_for_array_vec!(1);
implement_for_array_vec!(2);
implement_for_array_vec!(3);
implement_for_array_vec!(4);
implement_for_array_vec!(5);
implement_for_array_vec!(6);
implement_for_array_vec!(7);
implement_for_array_vec!(8);
implement_for_array_vec!(9);
implement_for_array_vec!(10);
implement_for_array_vec!(11);
implement_for_array_vec!(12);
implement_for_array_vec!(13);
implement_for_array_vec!(14);
implement_for_array_vec!(15);
implement_for_array_vec!(16);
implement_for_array_vec!(17);
implement_for_array_vec!(18);
implement_for_array_vec!(19);
implement_for_array_vec!(20);
implement_for_array_vec!(21);
implement_for_array_vec!(22);
implement_for_array_vec!(23);
implement_for_array_vec!(24);
implement_for_array_vec!(25);
implement_for_array_vec!(26);
implement_for_array_vec!(27);
implement_for_array_vec!(28);
implement_for_array_vec!(29);
implement_for_array_vec!(30);
implement_for_array_vec!(31);
implement_for_array_vec!(32);
implement_for_array_vec!(40);
implement_for_array_vec!(48);
implement_for_array_vec!(50);
implement_for_array_vec!(56);
implement_for_array_vec!(64);
implement_for_array_vec!(72);
implement_for_array_vec!(96);
implement_for_array_vec!(100);
implement_for_array_vec!(128);
implement_for_array_vec!(160);
implement_for_array_vec!(192);
implement_for_array_vec!(200);
implement_for_array_vec!(224);
implement_for_array_vec!(256);
implement_for_array_vec!(512);
implement_for_array_vec!(1024);
