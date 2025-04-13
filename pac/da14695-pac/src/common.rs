/*
DISCLAIMER
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products.
No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all
applicable laws, including copyright laws.
THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING THIS SOFTWARE, WHETHER EXPRESS, IMPLIED
OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NON-INFRINGEMENT.  ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED.TO THE MAXIMUM EXTENT PERMITTED NOT PROHIBITED BY
LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES SHALL BE LIABLE FOR ANY DIRECT,
INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS SOFTWARE, EVEN IF RENESAS OR
ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability
of this software. By using this software, you agree to the additional terms and conditions found by accessing the
following link:
http://www.renesas.com/disclaimer

*/
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:53:57 +0000

use core::convert::From;
use core::marker::PhantomData;

#[cfg(feature = "tracing")]
use crate::tracing;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

pub(crate) mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr};

    // It would be better with const fn
    // waiting for RFC: const functions in traits #3490
    pub trait CastFrom<A> {
        fn cast_from(val: A) -> Self;
    }

    impl CastFrom<u64> for u8 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u16 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u32 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u64 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    pub trait RegNumberT:
        Copy
        + From<u8>
        + Into<u64>
        + CastFrom<u64>
        + Shr<usize, Output = Self>
        + Shl<usize, Output = Self>
        + BitAndAssign
        + BitAnd<Output = Self>
        + Not<Output = Self>
        + BitOrAssign
    {
    }
    impl RegNumberT for u8 {}
    impl RegNumberT for u16 {}
    impl RegNumberT for u32 {}
    impl RegNumberT for u64 {}

    pub trait RegSpec {
        type DataType: RegNumberT;
    }
}

pub trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T, A: Access> {
    phantom: PhantomData<*mut (T, A)>,
}
unsafe impl<T, A: Access> Send for Reg<T, A> {}
unsafe impl<T, A: Access> Sync for Reg<T, A> {}

use sealed::CastFrom;

use sealed::{RegNumberT, RegSpec};
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct RegValueT<Reg: sealed::RegSpec> {
    pub(crate) data: Reg::DataType,
    pub(crate) mask: Reg::DataType,
}

pub trait RegisterValue<T: RegSpec> {
    /// Create a register value that could be written to a register from raw integer
    ///
    /// ```rust, ignore
    /// // example with generic names
    /// // needs: use test_pac::{timer, RegisterValue, TIMER}
    /// let to_write = timer::BitfieldReg::new(0xdeadbeef);
    /// TIMER.bitfield_reg().write(to_write);
    /// let to_write = to_write.boolw().set(true);
    /// TIMER.bitfield_reg().write(to_write);
    /// ```
    #[must_use]
    fn new(data: T::DataType) -> Self;

    /// Get raw integer from value read from register
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{RegisterValue, TIMER}
    /// let x = TIMER.bitfield_reg().read().get_raw();
    /// ```
    #[must_use]
    fn get_raw(&self) -> T::DataType;

    /// Prepare a register value that could be written to a register with an arbitrary value
    ///
    /// Use this function for setting a register to a custom value, independent
    /// of bitfields, enumerations, etc. No checks are performed on the passed
    /// value. The whole register is updated on write.
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{RegisterValue, TIMER}
    /// TIMER.bitfield_reg().init(|r| r.set_raw(0xdeadbeef))
    /// ```
    #[must_use]
    fn set_raw(self, value: T::DataType) -> Self;
}

impl<T: RegSpec> RegisterValue<T> for RegValueT<T> {
    /// Create a register value that could be written to a register from raw integer
    ///
    /// ```rust, ignore
    /// // example with generic names
    /// // needs: use pac::{timer, RegisterValue, TIMER}
    /// let to_write = timer::BitfieldReg::new(0xdeadbeef);
    /// TIMER.bitfield_reg().write(to_write);
    /// let to_write = to_write.boolw().set(true);
    /// TIMER.bitfield_reg().write(to_write);
    /// ```
    #[inline(always)]
    fn new(data: T::DataType) -> RegValueT<T> {
        Self {
            data,
            mask: 0x0u8.into(),
        }
    }

    /// Get raw integer from value read from register
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{RegisterValue, TIMER}
    /// let x = TIMER.bitfield_reg().read().get_raw();
    /// ```
    #[inline(always)]
    fn get_raw(&self) -> T::DataType {
        self.data
    }

    /// Prepare a register value that could be written to a register with an arbitrary value
    ///
    /// Use this function for setting a register to a custom value, independent
    /// of bitfields, enumerations, etc. No checks are performed on the passed
    /// value.
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{RegisterValue, TIMER}
    /// TIMER.bitfield_reg().init(|r| r.set_raw(0xdeadbeef))
    /// ```
    #[inline(always)]
    fn set_raw(mut self, value: T::DataType) -> Self {
        self.data = value;
        self.mask = !(Into::<T::DataType>::into(0x0u8));
        self
    }
}

pub trait NoBitfieldReg<Reg: RegSpec>: RegisterValue<Reg>
where
    Self: Sized,
{
    /// Get value read from register
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{NoBitfieldReg, TIMER}
    /// let x = TIMER.nobitfield_reg().read().get();
    /// ```
    #[inline(always)]
    #[must_use]
    fn get(&self) -> Reg::DataType {
        self.get_raw()
    }

    /// Prepare value to be written to register
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{NoBitfieldReg, TIMER}
    /// TIMER.nobitfield_reg().init(|r| r.set(0xc0ffee));
    /// ```
    #[inline(always)]
    #[must_use]
    fn set(self, value: Reg::DataType) -> Self {
        self.set_raw(value)
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Access,
{
    #[inline(always)]
    #[must_use]
    pub(crate) const fn from_ptr(ptr: *mut u8) -> &'static Self {
        unsafe { &*(ptr as *const Self) }
    }

    #[inline(always)]
    #[must_use]
    pub const fn ptr(&self) -> *mut T::DataType {
        self as *const _ as *mut T::DataType
    }

    /// Returns the address of the register.
    pub fn addr(&self) -> usize {
        unsafe { (self as *const _) as usize }
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Read,
{
    /// Read register and return a register value
    ///
    /// # Safety
    /// Read operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// let reg = unsafe { TIMER.bitfield_reg().read() };
    /// if reg.boolr().get() { /* ... */ }
    /// ```
    #[inline(always)]
    #[must_use]
    pub unsafe fn read(&self) -> RegValueT<T> {
        #[cfg(feature = "tracing")]
        let val = {
            let mut buf: u64 = 0x0;
            tracing::READ_FN.with(|rf| {
                if let Some(rf) = rf.get() {
                    buf = rf(self.addr(), std::mem::size_of::<T::DataType>());
                } else {
                    #[cfg(not(feature = "tracing_dummy"))]
                    panic!(
                        "Please, provide an handler for read with tracing::set_read_fn(callback);"
                    );
                }
            });
            T::DataType::cast_from(buf)
        };
        #[cfg(not(feature = "tracing"))]
        let val = self.ptr().read_volatile();
        RegValueT::<T>::new(val)
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Write,
{
    /// Write register value back to register
    ///
    /// # Arguments
    ///
    /// * `reg_value` - A string slice that holds the name of the person
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developers shall read the device user manual.
    /// Register is Send and Sync to allow complete freedom. Developers are responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// // write with a previously read value
    /// let reg = unsafe { TIMER.bitfield_reg().read() };
    /// // or start with a known value
    /// let reg = timer::BitfieldReg::new(0).bitfieldw().set(0x55);
    /// // or start with the register default
    /// let reg = timer::BitfieldReg::default();
    ///
    /// let reg = reg.bitfieldrw().set(0x77);
    ///
    /// // no change has taken place to the register due to `set` calls - do that now by writing back the result
    /// unsafe { TIMER.bitfield_reg().write(reg) }
    /// ```
    /// See also: [`Reg<T, A>::init`] which provides the default value to a closure
    #[inline(always)]
    pub unsafe fn write(&self, reg_value: RegValueT<T>) {
        #[cfg(feature = "tracing")]
        tracing::WRITE_FN.with(|wf| {
            if let Some(wf) = wf.get() {
                wf(
                    self.addr(),
                    std::mem::size_of::<T::DataType>(),
                    reg_value.data.into(),
                )
            } else {
                #[cfg(not(feature = "tracing_dummy"))]
                panic!("Please, provide an handler for read with tracing::set_read_fn(callback);");
            }
        });
        #[cfg(not(feature = "tracing"))]
        self.ptr().write_volatile(reg_value.data);
    }

    /// Write an arbitrary integer to register
    ///
    /// Use this function when e.g. loading data to be written from a config-page.
    /// For normal use prefer either [`Reg<T, A>::write`] if the value was read before, or [`Reg<T, A>::init`],
    /// both of which provide some restrictions available register fields, enums, etc.
    ///
    /// # Arguments
    ///
    /// * `value` - The unchecked value to be written to the register
    ///
    /// # Safety
    ///
    /// Write operation could cause undefined behavior for some peripheral. Developers shall read the device user manual.
    /// Register is Send and Sync to allow complete freedom. Developers are responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// unsafe { TIMER.bitfield_reg().write_raw(0xdead) }
    /// ```
    /// See also [`Reg<T, A>::init`] and [`Reg<T, A>::write`] both of which are the safe, preferred functions.
    #[inline(always)]
    pub unsafe fn write_raw(&self, value: T::DataType) {
        #[cfg(feature = "tracing")]
        tracing::WRITE_FN.with(|wf| {
            if let Some(wf) = wf.get() {
                wf(
                    self.addr(),
                    std::mem::size_of::<T::DataType>(),
                    value.into(),
                )
            } else {
                #[cfg(not(feature = "tracing_dummy"))]
                panic!("Please, provide an handler for read with tracing::set_read_fn(callback);");
            }
        });
        #[cfg(not(feature = "tracing"))]
        self.ptr().write_volatile(value);
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Write,
    RegValueT<T>: Default,
{
    /// Write register with register value built from default register value
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value initialized with register value at Power On Reset.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// TIMER
    ///     .bitfield_reg()
    ///     .init(|r| r.bitfieldw().set(0b1010).boolw().set(true));
    /// ```
    #[inline(always)]
    /// Write value computed by closure that receive as input the reset value of register
    pub unsafe fn init(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
        let val = RegValueT::<T>::default();
        let res = f(val);
        self.write(res);
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Read + Write,
{
    /// Read/modify/write register
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value read from register. The result of the closure
    ///   is written back to the register.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// TIMER
    ///     .bitfield_reg()
    ///     .modify(|r| r.boolrw().set(!r.boolrw().get()));
    /// ```
    #[inline(always)]
    pub unsafe fn modify(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
        let val = self.read();
        let res = f(val);
        self.write(res);
    }
}

/// Proxy struct for enumerated bitfields
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EnumBitfieldStruct<Q: RegNumberT, T>(pub Q, PhantomData<T>);

impl<Q: RegNumberT, T> EnumBitfieldStruct<Q, T> {
    pub const fn new(value: Q) -> Self {
        Self(value, PhantomData)
    }
}

impl<Q: RegNumberT, T> From<EnumBitfieldStruct<Q, T>> for u64 {
    #[inline(always)]
    fn from(value: EnumBitfieldStruct<Q, T>) -> Self {
        value.0.into()
    }
}
impl<Q: RegNumberT, T> CastFrom<u64> for EnumBitfieldStruct<Q, T> {
    #[inline(always)]
    fn cast_from(val: u64) -> Self {
        Self(Q::cast_from(val), PhantomData)
    }
}

impl<Q: RegNumberT, T> From<Q> for EnumBitfieldStruct<Q, T> {
    #[inline(always)]
    fn from(value: Q) -> Self {
        Self(value, PhantomData)
    }
}

/// Proxy struct for numeric bitfields
pub struct RegisterField<
    const START_OFFSET: usize,
    const MASK: u64,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    ValueType,
    T,
    A,
> where
    T: RegSpec,
    A: Access,
{
    data: RegValueT<T>,
    index: u8,
    marker: PhantomData<(ValueType, A)>,
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegSpec,
    A: Access,
{
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn from_register(data: RegValueT<T>, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }

    /// Get mask for bitfield, the mask is unshifted and at offset 0
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub fn mask(&self) -> T::DataType {
        T::DataType::cast_from(MASK)
    }

    /// Get offset of bitfield in containing register
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub const fn offset(&self) -> usize {
        START_OFFSET + (self.index * DIM_INCREMENT) as usize
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegSpec,
    A: Read,
    ValueType: CastFrom<u64>,
{
    /// Extract bitfield from read register value
    #[inline(always)]
    pub fn get(&self) -> ValueType {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered: T::DataType = (self.data.data >> offset) & T::DataType::cast_from(MASK);
        ValueType::cast_from(filtered.into())
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegSpec,
    A: Write,
    u64: From<ValueType>,
{
    /// Prepare bitfield value that could be written to register
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// // get an instance by reading
    /// let values = TIMER.bitfield_reg().read();
    /// // or by starting with a known value
    /// let value = timer::BitfieldReg::new(0);
    /// // or by starting with the default
    /// let value = timer::BitfieldReg::default();
    ///
    /// // set bitfields
    /// let value = value
    ///     // set numeric bitfield
    ///     .bitfieldw()
    ///     .set(0x55)
    ///     // set enumerated bitfield with enumeration
    ///     .bitfieldenumerated()
    ///     .set(timer::bitfield_reg::BitfieldEnumerated::GPIOA_0)
    ///     // set enumerated bitfield from integer
    ///     .bitfieldenumerated()
    ///     .set(1.into());
    ///
    /// // up until now no hardware change has taken place, do that now by writing
    /// TIMER.bitfield_reg().write(value);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: ValueType) -> RegValueT<T> {
        let mask = T::DataType::cast_from(MASK);
        let value: T::DataType = T::DataType::cast_from(Into::<u64>::into(value)) & mask;
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset: T::DataType = mask << offset;
        self.data.mask |= masked_offset;
        self.data.data &= !masked_offset;
        self.data.data |= value << offset;
        self.data
    }
}

/// Proxy struct for boolean bitfields
pub struct RegisterFieldBool<
    const START_OFFSET: usize,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    T,
    A,
> where
    T: RegSpec,
    A: Access,
{
    data: RegValueT<T>,
    index: u8,
    marker: PhantomData<A>,
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegSpec,
    A: Read,
{
    /// Extract bitfield from read register value
    #[inline(always)]
    pub fn get(&self) -> bool {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered = (self.data.data.into() >> offset) & 1;
        filtered == 1
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegSpec,
    A: Write,
{
    /// Prepare bitfield value to be written to register
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// // get an instance by reading
    /// let values = TIMER.bitfield_reg().read();
    /// // or by starting with a known value
    /// let value = timer::BitfieldReg::new(0);
    /// // or by starting with the default
    /// let value = timer::BitfieldReg::default();
    ///
    /// // set bitfield
    /// let value = value
    ///     .boolrw()
    ///     .set(true);
    ///
    /// // up until now no hardware change has taken place, do that now by writing
    /// TIMER.bitfield_reg().write(value);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: bool) -> RegValueT<T> {
        let value: T::DataType = if value {
            T::DataType::cast_from(1u64)
        } else {
            T::DataType::cast_from(0u64)
        };
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset = T::DataType::cast_from(0x1u64) << offset;
        self.data.mask |= masked_offset;
        self.data.data &= !masked_offset;
        self.data.data |= value << offset;
        self.data
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegSpec,
    A: Access,
{
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn from_register(data: RegValueT<T>, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }

    /// Get mask for bitfield, the mask is unshifted and at offset 0
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub fn mask(&self) -> T::DataType {
        T::DataType::cast_from(1)
    }

    /// Get offset of bitfield in containing register
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub const fn offset(&self) -> usize {
        START_OFFSET + (self.index * DIM_INCREMENT) as usize
    }
}

/// An array of identical register clusters.
pub struct ClusterRegisterArray<T: Sized, const DIM: usize, const DIM_INCREMENT: usize> {
    _t: ::core::marker::PhantomData<T>,
}

impl<T: Sized, const DIM: usize, const DIM_INCREMENT: usize>
    ClusterRegisterArray<T, DIM, DIM_INCREMENT>
{
    /// Returns the number of register blocks in the cluster.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        DIM
    }

    /// Returns whether the cluster is empty (DIM == 0).
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        DIM == 0
    }

    /// Returns an iterator over the elements of this cluster.
    #[inline(always)]
    pub fn iter(&self) -> impl ::core::iter::ExactSizeIterator<Item = &T> {
        self.into_iter()
    }

    /// Returns the cluster element with the specified index.
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub const fn get(&self, index: usize) -> &T {
        assert!(index < DIM);
        unsafe { self.get_unchecked(index) }
    }

    /// Returns the cluster element with the specified index.
    ///
    /// # Safety
    ///
    /// `index` must be less than `DIM`.
    #[inline(always)]
    pub const unsafe fn get_unchecked(&self, index: usize) -> &T {
        &*(self.as_ptr().add(index * DIM_INCREMENT) as *const _)
    }

    #[inline(always)]
    pub(crate) const unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const Self)
    }

    #[inline(always)]
    const fn as_ptr(&self) -> *mut u8 {
        self as *const _ as *mut _
    }
}

impl<T: Sized, const DIM: usize, const DIM_INCREMENT: usize> ::core::ops::Index<usize>
    for ClusterRegisterArray<T, DIM, DIM_INCREMENT>
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

impl<'a, T: Sized, const DIM: usize, const DIM_INCREMENT: usize> IntoIterator
    for &'a ClusterRegisterArray<T, DIM, DIM_INCREMENT>
{
    type Item = &'a T;
    type IntoIter = ClusterRegisterArrayIterator<'a, T, DIM, DIM_INCREMENT>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        ClusterRegisterArrayIterator {
            array: self,
            index: 0,
        }
    }
}

pub struct ClusterRegisterArrayIterator<'a, T: Sized, const DIM: usize, const DIM_INCREMENT: usize>
{
    array: &'a ClusterRegisterArray<T, DIM, DIM_INCREMENT>,
    index: usize,
}

impl<'a, T: Sized, const DIM: usize, const DIM_INCREMENT: usize> Iterator
    for ClusterRegisterArrayIterator<'a, T, DIM, DIM_INCREMENT>
{
    type Item = &'a T;
    #[inline(always)]
    fn next(&mut self) -> Option<&'a T> {
        if self.index < self.array.len() {
            let result = &self.array[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len() - self.index;
        (len, Some(len))
    }
}

impl<T: Sized, const DIM: usize, const DIM_INCREMENT: usize> ExactSizeIterator
    for ClusterRegisterArrayIterator<'_, T, DIM, DIM_INCREMENT>
{
}
