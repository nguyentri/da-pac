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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:45 +0000

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

    #[doc = "Base address of the command table"]
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

    #[doc = "Pattern generator 0 control register"]
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

    #[doc = "Pattern generator 1 control register"]
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

    #[doc = "Pattern generator 2 control register"]
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

    #[doc = "Pattern generator 3 control register"]
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

    #[doc = "Pattern generator 4 control register"]
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

    #[doc = "Motor control command FIFO register"]
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

    #[doc = "Command read pointer register"]
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

    #[doc = "Command write pointer register"]
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

    #[doc = "Motor control register"]
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

    #[doc = "Motor control IRQ clear register"]
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

    #[doc = "Motor controller trigger register"]
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

    #[doc = "Base address of the wavetable"]
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

#[doc = "Base address of the command table"]
pub type CmdTableBase = crate::RegValueT<CmdTableBase_SPEC>;

impl CmdTableBase {
    #[doc = "Dummy field for register test generation."]
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

#[doc = "Pattern generator 0 control register"]
pub type Pg0CtrlReg = crate::RegValueT<Pg0CtrlReg_SPEC>;

impl Pg0CtrlReg {
    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it is done generating a pattern. It is only valid if SMOTOR_GENEND_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it starts generating a pattern. It is only valid if SMOTOR_GENSTART_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Auto start mode: pattern generator will start whenever all enabled signals have received a command\n1 = Manual start mode: pattern generator will only start if it has been given a PG_START, and all enabled signals have received a command"]
    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Flex mode\n1 = Pair mode"]
    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
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

#[doc = "Pattern generator 1 control register"]
pub type Pg1CtrlReg = crate::RegValueT<Pg1CtrlReg_SPEC>;

impl Pg1CtrlReg {
    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it is done generating a pattern. It is only valid if SMOTOR_GENEND_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it starts generating a pattern. It is only valid if SMOTOR_GENSTART_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Auto start mode: pattern generator will start whenever all enabled signals have received a command\n1 = Manual start mode: pattern generator will only start if it has been given a PG_START, and all enabled signals have received a command"]
    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Flex mode\n1 = Pair mode"]
    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
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

#[doc = "Pattern generator 2 control register"]
pub type Pg2CtrlReg = crate::RegValueT<Pg2CtrlReg_SPEC>;

impl Pg2CtrlReg {
    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it is done generating a pattern. It is only valid if SMOTOR_GENEND_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it starts generating a pattern. It is only valid if SMOTOR_GENSTART_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Auto start mode: pattern generator will start whenever all enabled signals have received a command\n1 = Manual start mode: pattern generator will only start if it has been given a PG_START, and all enabled signals have received a command"]
    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Flex mode\n1 = Pair mode"]
    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
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

#[doc = "Pattern generator 3 control register"]
pub type Pg3CtrlReg = crate::RegValueT<Pg3CtrlReg_SPEC>;

impl Pg3CtrlReg {
    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it is done generating a pattern. It is only valid if SMOTOR_GENEND_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it starts generating a pattern. It is only valid if SMOTOR_GENSTART_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Auto start mode: pattern generator will start whenever all enabled signals have received a command\n1 = Manual start mode: pattern generator will only start if it has been given a PG_START, and all enabled signals have received a command"]
    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Flex mode\n1 = Pair mode"]
    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
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

#[doc = "Pattern generator 4 control register"]
pub type Pg4CtrlReg = crate::RegValueT<Pg4CtrlReg_SPEC>;

impl Pg4CtrlReg {
    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it is done generating a pattern. It is only valid if SMOTOR_GENEND_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines if the corresponding pattern generator will contribute to the generation of the IRQ when it starts generating a pattern. It is only valid if SMOTOR_GENSTART_IRQ_EN is enabled:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Auto start mode: pattern generator will start whenever all enabled signals have received a command\n1 = Manual start mode: pattern generator will only start if it has been given a PG_START, and all enabled signals have received a command"]
    #[inline(always)]
    pub fn pg_start_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Flex mode\n1 = Pair mode"]
    #[inline(always)]
    pub fn pg_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig3_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig2_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig1_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Signal disabled\n1 = Signal enabled"]
    #[inline(always)]
    pub fn sig0_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pg4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out3_sig(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Pg4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out2_sig(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Pg4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
    #[inline(always)]
    pub fn out1_sig(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Pg4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Pg4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects which signal is routed to the output."]
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

#[doc = "Motor control command FIFO register"]
pub type SmotorCmdFifoReg = crate::RegValueT<SmotorCmdFifoReg_SPEC>;

impl SmotorCmdFifoReg {
    #[doc = "Writing to this address will push a command into the command FIFO."]
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

#[doc = "Command read pointer register"]
pub type SmotorCmdReadPtrReg = crate::RegValueT<SmotorCmdReadPtrReg_SPEC>;

impl SmotorCmdReadPtrReg {
    #[doc = "Pointer to the next command to be popped from the FIFO. The command at SMOTOR_CMD_READ_PTR-1 is the last command that has been popped from the FIFO into its corresponding PG."]
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

#[doc = "Command write pointer register"]
pub type SmotorCmdWritePtrReg = crate::RegValueT<SmotorCmdWritePtrReg_SPEC>;

impl SmotorCmdWritePtrReg {
    #[doc = "Pointer to the location in the FIFO where the next command will be pushed at. The last command pushed to the FIFO is at SMOTOR_CMD_WRITE_PTR - 1. Can only be changed in cyclic mode"]
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

#[doc = "Motor control register"]
pub type SmotorCtrlReg = crate::RegValueT<SmotorCtrlReg_SPEC>;

impl SmotorCtrlReg {
    #[doc = "0 = RTC event does not trigger command pop\n1 = RTC event triggers command pop"]
    #[inline(always)]
    pub fn trig_rtc_event_en(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Divided sleep clock does not trigger command pop\n1 = Divided sleep clock triggers command pop"]
    #[inline(always)]
    pub fn mc_lp_clk_trig_en(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ in the event of the FIFO level (write pointer - read pointer) reaching, or is below the threshold determined by SMOTOR_THRESHOLD.\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn smotor_threshold_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines the FIFO level (write pointer - read pointer) at or below which and IRQ can be triggered using SMOTOR_THRESHOLD_IRQ_EN."]
    #[inline(always)]
    pub fn smotor_threshold(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, SmotorCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ in the event of FIFO underrun:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn smotor_fifo_unr_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ in the event of FIFO overflow:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn smotor_fifo_ovf_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ in the event a pattern generator (configured to do so through its corresponding GENEND_IRQ_EN bit) has ended generating a pattern:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn smotor_genend_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ in the event a pattern generator (configured to do so through its corresponding GENSTART_IRQ_EN bit) has just started generating a pattern:\n0 = Interrupt requests disabled\n1 = Interrupt requests enabled"]
    #[inline(always)]
    pub fn smotor_genstart_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SmotorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Idle time of a PG after generating a waveform. A PG will remain busy for the last signal\'s MOI to finish."]
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

    #[doc = "Depth of the cyclic buffer, only valid if CYCLIC_MODE is 1."]
    #[inline(always)]
    pub fn cyclic_size(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, SmotorCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,SmotorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines operation mode of command FIFO:\n0 = Normal FIFO mode\n1 = Cyclic buffer mode, CYCLIC_SIZE determines buffer depth"]
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

#[doc = "Motor control IRQ clear register"]
pub type SmotorIrqClearReg = crate::RegValueT<SmotorIrqClearReg_SPEC>;

impl SmotorIrqClearReg {
    #[doc = "Clears the THRESHOLD_IRQ_STATUS bit."]
    #[inline(always)]
    pub fn threshold_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Clears the FIFO_UNR_IRQ_STATUS bit."]
    #[inline(always)]
    pub fn fifo_unr_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Clears the FIFO_OVF_IRQ_STATUS bit."]
    #[inline(always)]
    pub fn fifo_ovf_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Clears the GENEND_IRQ_STATUS bit."]
    #[inline(always)]
    pub fn genend_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SmotorIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,SmotorIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Clears the GENSTART_IRQ_STATUS bit."]
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

#[doc = "Motor controller trigger register"]
pub type SmotorTriggerReg = crate::RegValueT<SmotorTriggerReg_SPEC>;

impl SmotorTriggerReg {
    #[doc = "Writing 1 to this bit will start PG4, only effective in manual mode."]
    #[inline(always)]
    pub fn pg4_start(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing 1 to this bit will start PG3, only effective in manual mode."]
    #[inline(always)]
    pub fn pg3_start(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing 1 to this bit will start PG2, only effective in manual mode."]
    #[inline(always)]
    pub fn pg2_start(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing 1 to this bit will start PG1, only effective in manual mode."]
    #[inline(always)]
    pub fn pg1_start(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing 1 to this bit will start PG0, only effective in manual mode."]
    #[inline(always)]
    pub fn pg0_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SmotorTriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,SmotorTriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing 1 will pop one (or more, depending on the N_CMDs field of the first) command(s) from the command buffer into its corresponding pattern generator."]
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

#[doc = "Base address of the wavetable"]
pub type WavetableBase = crate::RegValueT<WavetableBase_SPEC>;

impl WavetableBase {
    #[doc = "Dummy bitfield for register test generation."]
    #[inline(always)]
    pub fn wavetable_base_x_b3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, WavetableBase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,WavetableBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dummy bitfield for register test generation."]
    #[inline(always)]
    pub fn wavetable_base_x_b2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, WavetableBase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,WavetableBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dummy bitfield for register test generation."]
    #[inline(always)]
    pub fn wavetable_base_x_b1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, WavetableBase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,WavetableBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dummy bitfield for register test generation."]
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
