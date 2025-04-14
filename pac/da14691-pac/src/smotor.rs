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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:15 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SMOTOR registers"]
unsafe impl ::core::marker::Send for super::Smotor {}
unsafe impl ::core::marker::Sync for super::Smotor {}
impl super::Smotor {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn cmd_table_base(
        &self,
    ) -> &'static crate::common::Reg<self::CmdTableBase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmdTableBase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pg0_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pg0CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pg0CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pg1_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pg1CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pg1CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pg2_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pg2CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pg2CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pg3_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pg3CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pg3CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pg4_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pg4CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pg4CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smotor_cmd_fifo_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SmotorCmdFifoReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmotorCmdFifoReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smotor_cmd_read_ptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SmotorCmdReadPtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmotorCmdReadPtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smotor_cmd_write_ptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SmotorCmdWritePtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmotorCmdWritePtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smotor_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SmotorCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmotorCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smotor_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SmotorIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmotorIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smotor_trigger_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SmotorTriggerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmotorTriggerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wavetable_base(
        &self,
    ) -> &'static crate::common::Reg<self::WavetableBase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WavetableBase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdTableBase_SPEC;
impl crate::sealed::RegSpec for CmdTableBase_SPEC {
    type DataType = u32;
}

pub type CmdTableBase = crate::RegValueT<CmdTableBase_SPEC>;

impl CmdTableBase {
    #[inline(always)]
    pub fn cmd_table_base_x(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CmdTableBase_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CmdTableBase_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CmdTableBase {
    #[inline(always)]
    fn default() -> CmdTableBase {
        <crate::RegValueT<CmdTableBase_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pg0CtrlReg_SPEC;
impl crate::sealed::RegSpec for Pg0CtrlReg_SPEC {
    type DataType = u32;
}

pub type Pg0CtrlReg = crate::RegValueT<Pg0CtrlReg_SPEC>;

impl Pg0CtrlReg {
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out0_sig(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Pg0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pg0CtrlReg {
    #[inline(always)]
    fn default() -> Pg0CtrlReg {
        <crate::RegValueT<Pg0CtrlReg_SPEC> as RegisterValue<_>>::new(4068)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pg1CtrlReg_SPEC;
impl crate::sealed::RegSpec for Pg1CtrlReg_SPEC {
    type DataType = u32;
}

pub type Pg1CtrlReg = crate::RegValueT<Pg1CtrlReg_SPEC>;

impl Pg1CtrlReg {
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out0_sig(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Pg1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pg1CtrlReg {
    #[inline(always)]
    fn default() -> Pg1CtrlReg {
        <crate::RegValueT<Pg1CtrlReg_SPEC> as RegisterValue<_>>::new(4068)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pg2CtrlReg_SPEC;
impl crate::sealed::RegSpec for Pg2CtrlReg_SPEC {
    type DataType = u32;
}

pub type Pg2CtrlReg = crate::RegValueT<Pg2CtrlReg_SPEC>;

impl Pg2CtrlReg {
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out0_sig(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Pg2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pg2CtrlReg {
    #[inline(always)]
    fn default() -> Pg2CtrlReg {
        <crate::RegValueT<Pg2CtrlReg_SPEC> as RegisterValue<_>>::new(4068)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pg3CtrlReg_SPEC;
impl crate::sealed::RegSpec for Pg3CtrlReg_SPEC {
    type DataType = u32;
}

pub type Pg3CtrlReg = crate::RegValueT<Pg3CtrlReg_SPEC>;

impl Pg3CtrlReg {
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out0_sig(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Pg3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pg3CtrlReg {
    #[inline(always)]
    fn default() -> Pg3CtrlReg {
        <crate::RegValueT<Pg3CtrlReg_SPEC> as RegisterValue<_>>::new(4068)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pg4CtrlReg_SPEC;
impl crate::sealed::RegSpec for Pg4CtrlReg_SPEC {
    type DataType = u32;
}

pub type Pg4CtrlReg = crate::RegValueT<Pg4CtrlReg_SPEC>;

impl Pg4CtrlReg {
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out0_sig(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Pg4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pg4CtrlReg {
    #[inline(always)]
    fn default() -> Pg4CtrlReg {
        <crate::RegValueT<Pg4CtrlReg_SPEC> as RegisterValue<_>>::new(4068)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmotorCmdFifoReg_SPEC;
impl crate::sealed::RegSpec for SmotorCmdFifoReg_SPEC {
    type DataType = u32;
}

pub type SmotorCmdFifoReg = crate::RegValueT<SmotorCmdFifoReg_SPEC>;

impl SmotorCmdFifoReg {
    #[inline(always)]
    pub fn smotor_cmd_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SmotorCmdFifoReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SmotorCmdFifoReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SmotorCmdFifoReg {
    #[inline(always)]
    fn default() -> SmotorCmdFifoReg {
        <crate::RegValueT<SmotorCmdFifoReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmotorCmdReadPtrReg_SPEC;
impl crate::sealed::RegSpec for SmotorCmdReadPtrReg_SPEC {
    type DataType = u32;
}

pub type SmotorCmdReadPtrReg = crate::RegValueT<SmotorCmdReadPtrReg_SPEC>;

impl SmotorCmdReadPtrReg {
    #[inline(always)]
    pub fn smotor_cmd_read_ptr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        SmotorCmdReadPtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            SmotorCmdReadPtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SmotorCmdReadPtrReg {
    #[inline(always)]
    fn default() -> SmotorCmdReadPtrReg {
        <crate::RegValueT<SmotorCmdReadPtrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmotorCmdWritePtrReg_SPEC;
impl crate::sealed::RegSpec for SmotorCmdWritePtrReg_SPEC {
    type DataType = u32;
}

pub type SmotorCmdWritePtrReg = crate::RegValueT<SmotorCmdWritePtrReg_SPEC>;

impl SmotorCmdWritePtrReg {
    #[inline(always)]
    pub fn smotor_cmd_write_ptr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        SmotorCmdWritePtrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            SmotorCmdWritePtrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SmotorCmdWritePtrReg {
    #[inline(always)]
    fn default() -> SmotorCmdWritePtrReg {
        <crate::RegValueT<SmotorCmdWritePtrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmotorCtrlReg_SPEC;
impl crate::sealed::RegSpec for SmotorCtrlReg_SPEC {
    type DataType = u32;
}

pub type SmotorCtrlReg = crate::RegValueT<SmotorCtrlReg_SPEC>;

impl SmotorCtrlReg {
    #[inline(always)]
    pub fn trig_rtc_event_en(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mc_lp_clk_trig_en(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smotor_threshold_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smotor_threshold(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, SmotorCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smotor_fifo_unr_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smotor_fifo_ovf_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smotor_genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smotor_genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smotor_moi(
        self,
    ) -> crate::common::RegisterField<7, 0x3ff, 1, 0, u16, u16, SmotorCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x3ff,
            1,
            0,
            u16,
            u16,
            SmotorCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cyclic_size(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, SmotorCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cyclic_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SmotorCtrlReg {
    #[inline(always)]
    fn default() -> SmotorCtrlReg {
        <crate::RegValueT<SmotorCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmotorIrqClearReg_SPEC;
impl crate::sealed::RegSpec for SmotorIrqClearReg_SPEC {
    type DataType = u32;
}

pub type SmotorIrqClearReg = crate::RegValueT<SmotorIrqClearReg_SPEC>;

impl SmotorIrqClearReg {
    #[inline(always)]
    pub fn threshold_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifo_unr_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifo_ovf_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn genend_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn genstart_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for SmotorIrqClearReg {
    #[inline(always)]
    fn default() -> SmotorIrqClearReg {
        <crate::RegValueT<SmotorIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmotorTriggerReg_SPEC;
impl crate::sealed::RegSpec for SmotorTriggerReg_SPEC {
    type DataType = u32;
}

pub type SmotorTriggerReg = crate::RegValueT<SmotorTriggerReg_SPEC>;

impl SmotorTriggerReg {
    #[inline(always)]
    pub fn pg4_start(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg3_start(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg2_start(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg1_start(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pg0_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pop_cmd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for SmotorTriggerReg {
    #[inline(always)]
    fn default() -> SmotorTriggerReg {
        <crate::RegValueT<SmotorTriggerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WavetableBase_SPEC;
impl crate::sealed::RegSpec for WavetableBase_SPEC {
    type DataType = u32;
}

pub type WavetableBase = crate::RegValueT<WavetableBase_SPEC>;

impl WavetableBase {
    #[inline(always)]
    pub fn wavetable_base_x_b3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, WavetableBase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,WavetableBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wavetable_base_x_b2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, WavetableBase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,WavetableBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wavetable_base_x_b1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, WavetableBase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,WavetableBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wavetable_base_x_b0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, WavetableBase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,WavetableBase_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WavetableBase {
    #[inline(always)]
    fn default() -> WavetableBase {
        <crate::RegValueT<WavetableBase_SPEC> as RegisterValue<_>>::new(0)
    }
}
