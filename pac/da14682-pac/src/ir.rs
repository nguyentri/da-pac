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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"IR registers"]
unsafe impl ::core::marker::Send for super::Ir {}
unsafe impl ::core::marker::Sync for super::Ir {}
impl super::Ir {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ir_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_freq_carrier_off_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrFreqCarrierOffReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrFreqCarrierOffReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_freq_carrier_on_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrFreqCarrierOnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrFreqCarrierOnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_logic_one_time_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrLogicOneTimeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrLogicOneTimeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_logic_zero_time_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrLogicZeroTimeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrLogicZeroTimeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_main_fifo_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrMainFifoReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrMainFifoReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_repeat_fifo_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrRepeatFifoReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrRepeatFifoReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_repeat_time_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrRepeatTimeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrRepeatTimeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ir_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::IrStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrCtrlReg_SPEC;
impl crate::sealed::RegSpec for IrCtrlReg_SPEC {
    type DataType = u16;
}

pub type IrCtrlReg = crate::RegValueT<IrCtrlReg_SPEC>;

impl IrCtrlReg {
    #[inline(always)]
    pub fn ir_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, IrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,IrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_logic_one_format(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,IrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_logic_zero_format(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,IrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_invert_output(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,IrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_repeat_type(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,IrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_tx_start(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,IrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,IrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_rep_fifo_reset(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IrCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, IrCtrlReg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ir_code_fifo_reset(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IrCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, IrCtrlReg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for IrCtrlReg {
    #[inline(always)]
    fn default() -> IrCtrlReg {
        <crate::RegValueT<IrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrFreqCarrierOffReg_SPEC;
impl crate::sealed::RegSpec for IrFreqCarrierOffReg_SPEC {
    type DataType = u16;
}

pub type IrFreqCarrierOffReg = crate::RegValueT<IrFreqCarrierOffReg_SPEC>;

impl IrFreqCarrierOffReg {
    #[inline(always)]
    pub fn ir_freq_carrier_off(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        IrFreqCarrierOffReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            IrFreqCarrierOffReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IrFreqCarrierOffReg {
    #[inline(always)]
    fn default() -> IrFreqCarrierOffReg {
        <crate::RegValueT<IrFreqCarrierOffReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrFreqCarrierOnReg_SPEC;
impl crate::sealed::RegSpec for IrFreqCarrierOnReg_SPEC {
    type DataType = u16;
}

pub type IrFreqCarrierOnReg = crate::RegValueT<IrFreqCarrierOnReg_SPEC>;

impl IrFreqCarrierOnReg {
    #[inline(always)]
    pub fn ir_freq_carrier_on(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        IrFreqCarrierOnReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            IrFreqCarrierOnReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IrFreqCarrierOnReg {
    #[inline(always)]
    fn default() -> IrFreqCarrierOnReg {
        <crate::RegValueT<IrFreqCarrierOnReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for IrIrqStatusReg_SPEC {
    type DataType = u16;
}

pub type IrIrqStatusReg = crate::RegValueT<IrIrqStatusReg_SPEC>;

impl IrIrqStatusReg {
    #[inline(always)]
    pub fn ir_irq_ack(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IrIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,IrIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for IrIrqStatusReg {
    #[inline(always)]
    fn default() -> IrIrqStatusReg {
        <crate::RegValueT<IrIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrLogicOneTimeReg_SPEC;
impl crate::sealed::RegSpec for IrLogicOneTimeReg_SPEC {
    type DataType = u16;
}

pub type IrLogicOneTimeReg = crate::RegValueT<IrLogicOneTimeReg_SPEC>;

impl IrLogicOneTimeReg {
    #[inline(always)]
    pub fn ir_logic_one_mark(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        IrLogicOneTimeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            IrLogicOneTimeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ir_logic_one_space(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        IrLogicOneTimeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            IrLogicOneTimeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IrLogicOneTimeReg {
    #[inline(always)]
    fn default() -> IrLogicOneTimeReg {
        <crate::RegValueT<IrLogicOneTimeReg_SPEC> as RegisterValue<_>>::new(257)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrLogicZeroTimeReg_SPEC;
impl crate::sealed::RegSpec for IrLogicZeroTimeReg_SPEC {
    type DataType = u16;
}

pub type IrLogicZeroTimeReg = crate::RegValueT<IrLogicZeroTimeReg_SPEC>;

impl IrLogicZeroTimeReg {
    #[inline(always)]
    pub fn ir_logic_zero_mark(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        IrLogicZeroTimeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            IrLogicZeroTimeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ir_logic_zero_space(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        IrLogicZeroTimeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            IrLogicZeroTimeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IrLogicZeroTimeReg {
    #[inline(always)]
    fn default() -> IrLogicZeroTimeReg {
        <crate::RegValueT<IrLogicZeroTimeReg_SPEC> as RegisterValue<_>>::new(257)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrMainFifoReg_SPEC;
impl crate::sealed::RegSpec for IrMainFifoReg_SPEC {
    type DataType = u16;
}

pub type IrMainFifoReg = crate::RegValueT<IrMainFifoReg_SPEC>;

impl IrMainFifoReg {
    #[inline(always)]
    pub fn ir_code_fifo_data(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, IrMainFifoReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            IrMainFifoReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IrMainFifoReg {
    #[inline(always)]
    fn default() -> IrMainFifoReg {
        <crate::RegValueT<IrMainFifoReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrRepeatFifoReg_SPEC;
impl crate::sealed::RegSpec for IrRepeatFifoReg_SPEC {
    type DataType = u16;
}

pub type IrRepeatFifoReg = crate::RegValueT<IrRepeatFifoReg_SPEC>;

impl IrRepeatFifoReg {
    #[inline(always)]
    pub fn ir_repeat_fifo_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        IrRepeatFifoReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            IrRepeatFifoReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IrRepeatFifoReg {
    #[inline(always)]
    fn default() -> IrRepeatFifoReg {
        <crate::RegValueT<IrRepeatFifoReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrRepeatTimeReg_SPEC;
impl crate::sealed::RegSpec for IrRepeatTimeReg_SPEC {
    type DataType = u16;
}

pub type IrRepeatTimeReg = crate::RegValueT<IrRepeatTimeReg_SPEC>;

impl IrRepeatTimeReg {
    #[inline(always)]
    pub fn ir_repeat_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        IrRepeatTimeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            IrRepeatTimeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IrRepeatTimeReg {
    #[inline(always)]
    fn default() -> IrRepeatTimeReg {
        <crate::RegValueT<IrRepeatTimeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrStatusReg_SPEC;
impl crate::sealed::RegSpec for IrStatusReg_SPEC {
    type DataType = u16;
}

pub type IrStatusReg = crate::RegValueT<IrStatusReg_SPEC>;

impl IrStatusReg {
    #[inline(always)]
    pub fn ir_busy(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,IrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_rep_fifo_wrds(
        self,
    ) -> crate::common::RegisterField<6, 0xf, 1, 0, u8, u8, IrStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0xf,1,0,u8,u8,IrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ir_code_fifo_wrds(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, IrStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,IrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for IrStatusReg {
    #[inline(always)]
    fn default() -> IrStatusReg {
        <crate::RegValueT<IrStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
