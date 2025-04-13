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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:17 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Cortex M0 SCB registers"]
unsafe impl ::core::marker::Send for super::Scb {}
unsafe impl ::core::marker::Sync for super::Scb {}
impl super::Scb {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "CPUID base register"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &'static crate::common::Reg<self::Cpuid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt control and state register"]
    #[inline(always)]
    pub const fn icsr(&self) -> &'static crate::common::Reg<self::Icsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Application interrupt and reset control register"]
    #[inline(always)]
    pub const fn aircr(&self) -> &'static crate::common::Reg<self::Aircr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Aircr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "System control register"]
    #[inline(always)]
    pub const fn scr(&self) -> &'static crate::common::Reg<self::Scr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Configuration and control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &'static crate::common::Reg<self::Ccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "System handler priority register 2"]
    #[inline(always)]
    pub const fn shpr2(&self) -> &'static crate::common::Reg<self::Shpr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Shpr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "System handler priority register 3"]
    #[inline(always)]
    pub const fn shpr3(&self) -> &'static crate::common::Reg<self::Shpr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Shpr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuid_SPEC;
impl crate::sealed::RegSpec for Cpuid_SPEC {
    type DataType = u32;
}
#[doc = "CPUID base register"]
pub type Cpuid = crate::RegValueT<Cpuid_SPEC>;

impl Cpuid {
    #[doc = "REVISION\\[3:0\\] bits (Revision number)"]
    #[inline(always)]
    pub fn revision(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Cpuid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Cpuid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PARTNO\\[11:0\\] bits (Part number of the processor core)"]
    #[inline(always)]
    pub fn partno(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Cpuid_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Cpuid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CONSTANT\\[3:0\\] bits (Reads as 0xF)"]
    #[inline(always)]
    pub fn constant(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Cpuid_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Cpuid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VARIANT\\[3:0\\] bits (Variant number)"]
    #[inline(always)]
    pub fn variant(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Cpuid_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Cpuid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IMPLEMENTER\\[7:0\\] bits (Implementer code)"]
    #[inline(always)]
    pub fn implementer(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cpuid_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cpuid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpuid {
    #[inline(always)]
    fn default() -> Cpuid {
        <crate::RegValueT<Cpuid_SPEC> as RegisterValue<_>>::new(1091355136)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr_SPEC;
impl crate::sealed::RegSpec for Icsr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt control and state register"]
pub type Icsr = crate::RegValueT<Icsr_SPEC>;

impl Icsr {
    #[doc = "VECTACTIVE\\[5:0\\] bits (Active vector)"]
    #[inline(always)]
    pub fn vectactive(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Icsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VECTPENDING\\[5:0\\] bits (Pending vector)"]
    #[inline(always)]
    pub fn vectpending(
        self,
    ) -> crate::common::RegisterField<12, 0x3f, 1, 0, u8, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3f,1,0,u8, Icsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt pending flag, excluding NMI and Faults"]
    #[inline(always)]
    pub fn isrpending(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Icsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SysTick exception clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Icsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Icsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Icsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Icsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Icsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Icsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Icsr {
    #[inline(always)]
    fn default() -> Icsr {
        <crate::RegValueT<Icsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aircr_SPEC;
impl crate::sealed::RegSpec for Aircr_SPEC {
    type DataType = u32;
}
#[doc = "Application interrupt and reset control register"]
pub type Aircr = crate::RegValueT<Aircr_SPEC>;

impl Aircr {
    #[doc = "Reserved for Debug use"]
    #[inline(always)]
    pub fn vectreset(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Aircr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Aircr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reserved for Debug use"]
    #[inline(always)]
    pub fn vectclractive(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Aircr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Aircr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "System reset request"]
    #[inline(always)]
    pub fn sysresetreq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Aircr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Aircr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Data endianness bit"]
    #[inline(always)]
    pub fn endianess(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Aircr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Aircr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VECTKEY\\[15:0\\] bits (Register key)"]
    #[inline(always)]
    pub fn vectkey(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Aircr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Aircr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Aircr {
    #[inline(always)]
    fn default() -> Aircr {
        <crate::RegValueT<Aircr_SPEC> as RegisterValue<_>>::new(4194631680)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr_SPEC;
impl crate::sealed::RegSpec for Scr_SPEC {
    type DataType = u32;
}
#[doc = "System control register"]
pub type Scr = crate::RegValueT<Scr_SPEC>;

impl Scr {
    #[doc = "Configures sleep-on-exit when returning from Handler mode to Thread mode"]
    #[inline(always)]
    pub fn sleeponexit(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Scr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep"]
    #[inline(always)]
    pub fn sleepdeep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Scr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Send event on pending bit"]
    #[inline(always)]
    pub fn seveonpend(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Scr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        <crate::RegValueT<Scr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr_SPEC;
impl crate::sealed::RegSpec for Ccr_SPEC {
    type DataType = u32;
}
#[doc = "Configuration and control register"]
pub type Ccr = crate::RegValueT<Ccr_SPEC>;

impl Ccr {
    #[doc = "Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ccr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ccr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Configures stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ccr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ccr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        <crate::RegValueT<Ccr_SPEC> as RegisterValue<_>>::new(516)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shpr2_SPEC;
impl crate::sealed::RegSpec for Shpr2_SPEC {
    type DataType = u32;
}
#[doc = "System handler priority register 2"]
pub type Shpr2 = crate::RegValueT<Shpr2_SPEC>;

impl Shpr2 {
    #[doc = "PRI_11\\[7:0\\] bits (Priority of system handler 11, SVCall)"]
    #[inline(always)]
    pub fn pri_11(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Shpr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Shpr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Shpr2 {
    #[inline(always)]
    fn default() -> Shpr2 {
        <crate::RegValueT<Shpr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shpr3_SPEC;
impl crate::sealed::RegSpec for Shpr3_SPEC {
    type DataType = u32;
}
#[doc = "System handler priority register 3"]
pub type Shpr3 = crate::RegValueT<Shpr3_SPEC>;

impl Shpr3 {
    #[doc = "PRI_14\\[7:0\\] bits (Priority of system handler 14, PendSV)"]
    #[inline(always)]
    pub fn pri_14(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Shpr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Shpr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PRI_15\\[7:0\\] bits (Priority of system handler 15, SysTick exception)"]
    #[inline(always)]
    pub fn pri_15(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Shpr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Shpr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Shpr3 {
    #[inline(always)]
    fn default() -> Shpr3 {
        <crate::RegValueT<Shpr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
