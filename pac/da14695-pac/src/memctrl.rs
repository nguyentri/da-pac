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

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"MEMCTRL registers"]
unsafe impl ::core::marker::Send for super::Memctrl {}
unsafe impl ::core::marker::Sync for super::Memctrl {}
impl super::Memctrl {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "BSR Reset Register"]
    #[inline(always)]
    pub const fn busy_reset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BusyResetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BusyResetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "BSR Set Register"]
    #[inline(always)]
    pub const fn busy_set_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BusySetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BusySetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "BSR Status Register"]
    #[inline(always)]
    pub const fn busy_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BusyStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BusyStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "CMAC code Base Address Register"]
    #[inline(always)]
    pub const fn cmi_code_base_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmiCodeBaseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmiCodeBaseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "CMAC data Base Address Register"]
    #[inline(always)]
    pub const fn cmi_data_base_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmiDataBaseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmiDataBaseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "CMAC end Address Register"]
    #[inline(always)]
    pub const fn cmi_end_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmiEndReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmiEndReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "CMAC shared data Base Address Register"]
    #[inline(always)]
    pub const fn cmi_shared_base_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmiSharedBaseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmiSharedBaseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Priority Control Register"]
    #[inline(always)]
    pub const fn mem_prio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MemPrioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MemPrioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Maximum Stall cycles Control Register"]
    #[inline(always)]
    pub const fn mem_stall_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MemStallReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MemStallReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "RAM cells Status Register"]
    #[inline(always)]
    pub const fn mem_status2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MemStatus2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MemStatus2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Memory Arbiter Status Register"]
    #[inline(always)]
    pub const fn mem_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MemStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MemStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Sensor Node Controller Base Address Register"]
    #[inline(always)]
    pub const fn snc_base_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncBaseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncBaseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BusyResetReg_SPEC;
impl crate::sealed::RegSpec for BusyResetReg_SPEC {
    type DataType = u32;
}
#[doc = "BSR Reset Register"]
pub type BusyResetReg = crate::RegValueT<BusyResetReg_SPEC>;

impl BusyResetReg {
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_spare(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_motor(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_timer2(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_timer(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_uart3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_gpadc(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_pdm(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_src(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_pcm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_sdadc(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_i2c2(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_i2c(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_spi2(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_spi(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_uart2(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear the BUSY bitfield, by writing the master code which has claimed to this field\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_uart(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, BusyResetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, BusyResetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BusyResetReg {
    #[inline(always)]
    fn default() -> BusyResetReg {
        <crate::RegValueT<BusyResetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BusySetReg_SPEC;
impl crate::sealed::RegSpec for BusySetReg_SPEC {
    type DataType = u32;
}
#[doc = "BSR Set Register"]
pub type BusySetReg = crate::RegValueT<BusySetReg_SPEC>;

impl BusySetReg {
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_spare(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_motor(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_timer2(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_timer(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_uart3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_gpadc(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_pdm(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_src(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_pcm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_sdadc(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_i2c2(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_i2c(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_spi2(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_spi(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_uart2(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing a non-zero value to this field sets the corresponding BUSY bit, but only if it was not claimed (BUSY=0).\nReading returns 0 to allow read/modify/write to the register."]
    #[inline(always)]
    pub fn busy_uart(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, BusySetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, BusySetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BusySetReg {
    #[inline(always)]
    fn default() -> BusySetReg {
        <crate::RegValueT<BusySetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BusyStatReg_SPEC;
impl crate::sealed::RegSpec for BusyStatReg_SPEC {
    type DataType = u32;
}
#[doc = "BSR Status Register"]
pub type BusyStatReg = crate::RegValueT<BusyStatReg_SPEC>;

impl BusyStatReg {
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_spare(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_motor(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_timer2(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_timer(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_uart3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_gpadc(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_pdm(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_src(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_pcm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_sdadc(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_i2c2(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_i2c(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_spi2(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_spi(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_uart2(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "A non-zero value indicates the resource is busy. The value represents which master is using it."]
    #[inline(always)]
    pub fn busy_uart(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, BusyStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, BusyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BusyStatReg {
    #[inline(always)]
    fn default() -> BusyStatReg {
        <crate::RegValueT<BusyStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmiCodeBaseReg_SPEC;
impl crate::sealed::RegSpec for CmiCodeBaseReg_SPEC {
    type DataType = u32;
}
#[doc = "CMAC code Base Address Register"]
pub type CmiCodeBaseReg = crate::RegValueT<CmiCodeBaseReg_SPEC>;

impl CmiCodeBaseReg {
    #[doc = "Base address for CMAC code with steps of 1 kB.\n0x001: 1 kB base address\n0x010: 16 kB base address\n0x100: 256 kB base address"]
    #[inline(always)]
    pub fn cmi_code_base_addr(
        self,
    ) -> crate::common::RegisterField<10, 0x1ff, 1, 0, u16, CmiCodeBaseReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1ff,1,0,u16, CmiCodeBaseReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmiCodeBaseReg {
    #[inline(always)]
    fn default() -> CmiCodeBaseReg {
        <crate::RegValueT<CmiCodeBaseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmiDataBaseReg_SPEC;
impl crate::sealed::RegSpec for CmiDataBaseReg_SPEC {
    type DataType = u32;
}
#[doc = "CMAC data Base Address Register"]
pub type CmiDataBaseReg = crate::RegValueT<CmiDataBaseReg_SPEC>;

impl CmiDataBaseReg {
    #[doc = "Base address for CMAC data with steps of 4 bytes.\n0x00001: 4 byte base address\n0x00010: 64 byte base address\n0x00100: 1 kB base address\n0x01000: 16 kB base address\n0x10000: 256 kB base address"]
    #[inline(always)]
    pub fn cmi_data_base_addr(
        self,
    ) -> crate::common::RegisterField<2, 0x1ffff, 1, 0, u32, CmiDataBaseReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1ffff,1,0,u32, CmiDataBaseReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmiDataBaseReg {
    #[inline(always)]
    fn default() -> CmiDataBaseReg {
        <crate::RegValueT<CmiDataBaseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmiEndReg_SPEC;
impl crate::sealed::RegSpec for CmiEndReg_SPEC {
    type DataType = u32;
}
#[doc = "CMAC end Address Register"]
pub type CmiEndReg = crate::RegValueT<CmiEndReg_SPEC>;

impl CmiEndReg {
    #[doc = "End address for CMAC code and data accesses with steps of 1 kB.\n0x000: accesses up to 1kB are allowed\n0x001: accesses up to 2kB are allowed\n0x01F: accesses up to 32kB are allowed\n0x1FF: accesses up to 512kB are allowed"]
    #[inline(always)]
    pub fn cmi_end_addr(
        self,
    ) -> crate::common::RegisterField<10, 0x1ff, 1, 0, u16, CmiEndReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1ff,1,0,u16, CmiEndReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmiEndReg {
    #[inline(always)]
    fn default() -> CmiEndReg {
        <crate::RegValueT<CmiEndReg_SPEC> as RegisterValue<_>>::new(524287)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmiSharedBaseReg_SPEC;
impl crate::sealed::RegSpec for CmiSharedBaseReg_SPEC {
    type DataType = u32;
}
#[doc = "CMAC shared data Base Address Register"]
pub type CmiSharedBaseReg = crate::RegValueT<CmiSharedBaseReg_SPEC>;

impl CmiSharedBaseReg {
    #[doc = "Base address for CMAC shared data with steps of 1 kB.\n0x001: 1 kB base address\n0x010: 16 kB base address\n0x100: 256 kB base address"]
    #[inline(always)]
    pub fn cmi_shared_base_addr(
        self,
    ) -> crate::common::RegisterField<10, 0x1ff, 1, 0, u16, CmiSharedBaseReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1ff,1,0,u16, CmiSharedBaseReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmiSharedBaseReg {
    #[inline(always)]
    fn default() -> CmiSharedBaseReg {
        <crate::RegValueT<CmiSharedBaseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemPrioReg_SPEC;
impl crate::sealed::RegSpec for MemPrioReg_SPEC {
    type DataType = u32;
}
#[doc = "Priority Control Register"]
pub type MemPrioReg = crate::RegValueT<MemPrioReg_SPEC>;

impl MemPrioReg {
    #[doc = "Priority for the AHB interface.\n00: low priority (default)\n01: mid priority\n1x: high priority"]
    #[inline(always)]
    pub fn ahb_prio(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, MemPrioReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, MemPrioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority for the AHB2 interface.\n00: low priority (default)\n01: mid priority\n1x: high priority"]
    #[inline(always)]
    pub fn ahb2_prio(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, MemPrioReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, MemPrioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority for the SNC interface.\n00: low priority (default)\n01: mid priority\n1x: high priority"]
    #[inline(always)]
    pub fn snc_prio(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, MemPrioReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, MemPrioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MemPrioReg {
    #[inline(always)]
    fn default() -> MemPrioReg {
        <crate::RegValueT<MemPrioReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemStallReg_SPEC;
impl crate::sealed::RegSpec for MemStallReg_SPEC {
    type DataType = u32;
}
#[doc = "Maximum Stall cycles Control Register"]
pub type MemStallReg = crate::RegValueT<MemStallReg_SPEC>;

impl MemStallReg {
    #[doc = "Maximum allowed number of stall cycles for the AHB interface. If exceeded, the interface will get top priority (above high priority). Valid for a single access so the next access (of a burst) might end up in the que for the same number of wait cycles.\n0: don\'t use, not feasible and can block other interfaces\n1: max 1 stall cycle\n15: max 15 stall cycles"]
    #[inline(always)]
    pub fn ahb_max_stall(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, MemStallReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, MemStallReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum allowed number of stall cycles for the AHB2 interface. If exceeded, the interface will get top priority (above high priority). Valid for a single access so the next access (of a burst) might end up in the que for the same number of wait cycles.\n0: don\'t use, not feasible and can block other interfaces\n1: max 1 stall cycle\n15: max 15 stall cycles"]
    #[inline(always)]
    pub fn ahb2_max_stall(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, MemStallReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, MemStallReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum allowed number of stall cycles for the SNC interface. If exceeded, the interface will get top priority (above high priority). Valid for a single access so the next access (of a burst) might end up in the que for the same number of wait cycles.\n0: don\'t use, not feasible and can block other interfaces\n1: max 1 stall cycle\n15: max 15 stall cycles"]
    #[inline(always)]
    pub fn snc_max_stall(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, MemStallReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, MemStallReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MemStallReg {
    #[inline(always)]
    fn default() -> MemStallReg {
        <crate::RegValueT<MemStallReg_SPEC> as RegisterValue<_>>::new(4095)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemStatus2Reg_SPEC;
impl crate::sealed::RegSpec for MemStatus2Reg_SPEC {
    type DataType = u32;
}
#[doc = "RAM cells Status Register"]
pub type MemStatus2Reg = crate::RegValueT<MemStatus2Reg_SPEC>;

impl MemStatus2Reg {
    #[doc = "Reading a \'1\' indicates RAM8 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram8_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reading a \'1\' indicates RAM7 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram7_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reading a \'1\' indicates RAM6 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram6_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reading a \'1\' indicates RAM5 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram5_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reading a \'1\' indicates RAM4 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram4_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reading a \'1\' indicates RAM3 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram3_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reading a \'1\' indicates RAM2 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram2_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reading a \'1\' indicates RAM1 was off but still access was performed.\nWriting a \'1\' will clear the status back to \'0\'."]
    #[inline(always)]
    pub fn ram1_off_but_access(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MemStatus2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,MemStatus2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MemStatus2Reg {
    #[inline(always)]
    fn default() -> MemStatus2Reg {
        <crate::RegValueT<MemStatus2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemStatusReg_SPEC;
impl crate::sealed::RegSpec for MemStatusReg_SPEC {
    type DataType = u32;
}
#[doc = "Memory Arbiter Status Register"]
pub type MemStatusReg = crate::RegValueT<MemStatusReg_SPEC>;

impl MemStatusReg {
    #[doc = "Writing a \'1\' clears CMI_NOT_READY bit."]
    #[inline(always)]
    pub fn cmi_clear_ready(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, MemStatusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13,1,0,MemStatusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "0: Normal operation\n1: CMI access performed which couldn\'t be handled right away (interface doesn\'t allow wait cycles)"]
    #[inline(always)]
    pub fn cmi_not_ready(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, MemStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,MemStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The maximum number of arbiter clock cycles that an AHB2 access has been buffered."]
    #[inline(always)]
    pub fn ahb2_wr_buff_cnt(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, MemStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, MemStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The maximum number of arbiter clock cycles that an AHB access has been buffered."]
    #[inline(always)]
    pub fn ahb_wr_buff_cnt(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, MemStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8, MemStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Writing a \'1\' clears AHB2_WR_BUFF_CNT."]
    #[inline(always)]
    pub fn ahb2_clr_wr_buff(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, MemStatusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,MemStatusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a \'1\' clears AHB_WR_BUFF_CNT."]
    #[inline(always)]
    pub fn ahb_clr_wr_buff(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, MemStatusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,MemStatusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "0: No AHB2 write access is buffered.\n1: Currently a single AHB2 write access is buffered in the arbiter."]
    #[inline(always)]
    pub fn ahb2_write_buff(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, MemStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,MemStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: No AHB write access is buffered.\n1: Currently a single AHB write access is buffered in the arbiter."]
    #[inline(always)]
    pub fn ahb_write_buff(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MemStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MemStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MemStatusReg {
    #[inline(always)]
    fn default() -> MemStatusReg {
        <crate::RegValueT<MemStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncBaseReg_SPEC;
impl crate::sealed::RegSpec for SncBaseReg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node Controller Base Address Register"]
pub type SncBaseReg = crate::RegValueT<SncBaseReg_SPEC>;

impl SncBaseReg {
    #[doc = "Base address for SNC interface with steps of 4 bytes.\n0x00001: 4 byte base address\n0x00010: 64 byte base address\n0x00100: 1 kB base address\n0x01000: 16 kB base address\n0x10000: 256 kB base address"]
    #[inline(always)]
    pub fn snc_base_address(
        self,
    ) -> crate::common::RegisterField<2, 0x1ffff, 1, 0, u32, SncBaseReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1ffff,1,0,u32, SncBaseReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SncBaseReg {
    #[inline(always)]
    fn default() -> SncBaseReg {
        <crate::RegValueT<SncBaseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
