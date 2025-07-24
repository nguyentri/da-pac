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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"QUADEC registers"]
unsafe impl ::core::marker::Send for super::Quadec {}
unsafe impl ::core::marker::Sync for super::Quadec {}
impl super::Quadec {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Clock divider register"]
    #[inline(always)]
    pub const fn qdec_clockdiv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecClockdivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecClockdivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Quad Decoder port selection register"]
    #[inline(always)]
    pub const fn qdec_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Quad Decoder control register"]
    #[inline(always)]
    pub const fn qdec_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Event counter register"]
    #[inline(always)]
    pub const fn qdec_event_cnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecEventCntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecEventCntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Counter value of the X Axis"]
    #[inline(always)]
    pub const fn qdec_xcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecXcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecXcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Counter value of the Y Axis"]
    #[inline(always)]
    pub const fn qdec_ycnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecYcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecYcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Counter value of the Z Axis"]
    #[inline(always)]
    pub const fn qdec_zcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecZcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecZcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecClockdivReg_SPEC;
impl crate::sealed::RegSpec for QdecClockdivReg_SPEC {
    type DataType = u16;
}

#[doc = "Clock divider register"]
pub type QdecClockdivReg = crate::RegValueT<QdecClockdivReg_SPEC>;

impl QdecClockdivReg {
    #[doc = "0 = no prescaler enabled\n1 = in sleep and active mode, quadrature clock is divided by 2"]
    #[inline(always)]
    pub fn qdec_prescaler_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, QdecClockdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,QdecClockdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Contains the number of the input clock cycles minus one, that are required to generate one logic clock cycle.\nClock divider is bypassed when system runs at LP_CLK"]
    #[inline(always)]
    pub fn qdec_clockdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        QdecClockdivReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            QdecClockdivReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QdecClockdivReg {
    #[inline(always)]
    fn default() -> QdecClockdivReg {
        <crate::RegValueT<QdecClockdivReg_SPEC> as RegisterValue<_>>::new(999)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for QdecCtrl2Reg_SPEC {
    type DataType = u16;
}

#[doc = "Quad Decoder port selection register"]
pub type QdecCtrl2Reg = crate::RegValueT<QdecCtrl2Reg_SPEC>;

impl QdecCtrl2Reg {
    #[doc = "0 = Normal quadrature counting\n1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chz_event_mode(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, QdecCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Normal quadrature counting\n1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chy_event_mode(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, QdecCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Normal quadrature counting\n1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chx_event_mode(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, QdecCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines which GPIOs are mapped on Channel Z\n0: none\n1: P0\\[2\\] -> CHZ_A, P0\\[5\\] -> CHZ_B\n2: P0\\[1\\] -> CHZ_A, P0\\[4\\] -> CHZ_B\n3: P0\\[3\\] -> CHZ_A, P0\\[10\\] -> CHZ_B\n4: P0\\[6\\] -> CHZ_A, P0\\[7\\] -> CHZ_B\n5: P0\\[8\\] -> CHZ_A, P0\\[9\\] -> CHZ_B\n6: P0\\[0\\] -> CHZ_A, P0\\[11\\] -> CHZ_B\n7: none"]
    #[inline(always)]
    pub fn qdec_chz_port_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, QdecCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines which GPIOs are mapped on Channel Y\n0: none\n1: P0\\[2\\] -> CHY_A, P0\\[5\\] -> CHY_B\n2: P0\\[1\\] -> CHY_A, P0\\[4\\] -> CHY_B\n3: P0\\[3\\] -> CHY_A, P0\\[10\\] -> CHY_B\n4: P0\\[6\\] -> CHY_A, P0\\[7\\] -> CHY_B\n5: P0\\[8\\] -> CHY_A, P0\\[9\\] -> CHY_B\n6: P0\\[0\\] -> CHY_A, P0\\[11\\] -> CHY_B\n7: none"]
    #[inline(always)]
    pub fn qdec_chy_port_sel(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, QdecCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines which GPIOs are mapped on Channel X\n0: none\n1: P0\\[2\\] -> CHX_A, P0\\[5\\] -> CHX_B\n2: P0\\[1\\] -> CHX_A, P0\\[4\\] -> CHX_B\n3: P0\\[3\\] -> CHX_A, P0\\[10\\] -> CHX_B\n4: P0\\[6\\] -> CHX_A, P0\\[7\\] -> CHX_B\n5: P0\\[8\\] -> CHX_A, P0\\[9\\] -> CHX_B\n6: P0\\[0\\] -> CHX_A, P0\\[11\\] -> CHX_B\n7: none"]
    #[inline(always)]
    pub fn qdec_chx_port_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, QdecCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecCtrl2Reg {
    #[inline(always)]
    fn default() -> QdecCtrl2Reg {
        <crate::RegValueT<QdecCtrl2Reg_SPEC> as RegisterValue<_>>::new(3793)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecCtrlReg_SPEC;
impl crate::sealed::RegSpec for QdecCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Quad Decoder control register"]
pub type QdecCtrlReg = crate::RegValueT<QdecCtrlReg_SPEC>;

impl QdecCtrlReg {
    #[doc = "Defines the number of events on either counter (X or Y or Z) that need to be reached before an interrupt is generated. Events are equal to QDEC_IRQ_THRES+1."]
    #[inline(always)]
    pub fn qdec_irq_thres(
        self,
    ) -> crate::common::RegisterField<3, 0xff, 1, 0, u8, u8, QdecCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xff,1,0,u8,u8,QdecCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1 = Interrupt is occured.\n0 = No interrupt pending\nWrite 1 will clear the pending interrupt"]
    #[inline(always)]
    pub fn qdec_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QdecCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,QdecCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing 1 QDEC_EVENT_CNT_REG is cleared"]
    #[inline(always)]
    pub fn qdec_event_cnt_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, QdecCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,QdecCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = interrupt is masked\n1 = interrupt is enabled"]
    #[inline(always)]
    pub fn qdec_irq_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QdecCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,QdecCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecCtrlReg {
    #[inline(always)]
    fn default() -> QdecCtrlReg {
        <crate::RegValueT<QdecCtrlReg_SPEC> as RegisterValue<_>>::new(17)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecEventCntReg_SPEC;
impl crate::sealed::RegSpec for QdecEventCntReg_SPEC {
    type DataType = u16;
}

#[doc = "Event counter register"]
pub type QdecEventCntReg = crate::RegValueT<QdecEventCntReg_SPEC>;

impl QdecEventCntReg {
    #[doc = "Gives the number of events at all channels."]
    #[inline(always)]
    pub fn qdec_event_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, QdecEventCntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,QdecEventCntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecEventCntReg {
    #[inline(always)]
    fn default() -> QdecEventCntReg {
        <crate::RegValueT<QdecEventCntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecXcntReg_SPEC;
impl crate::sealed::RegSpec for QdecXcntReg_SPEC {
    type DataType = u16;
}

#[doc = "Counter value of the X Axis"]
pub type QdecXcntReg = crate::RegValueT<QdecXcntReg_SPEC>;

impl QdecXcntReg {
    #[doc = "Contains a signed value of the events. Zero when channel is disabled"]
    #[inline(always)]
    pub fn qdec_x_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, QdecXcntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,QdecXcntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecXcntReg {
    #[inline(always)]
    fn default() -> QdecXcntReg {
        <crate::RegValueT<QdecXcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecYcntReg_SPEC;
impl crate::sealed::RegSpec for QdecYcntReg_SPEC {
    type DataType = u16;
}

#[doc = "Counter value of the Y Axis"]
pub type QdecYcntReg = crate::RegValueT<QdecYcntReg_SPEC>;

impl QdecYcntReg {
    #[doc = "Contains a signed value of the events. Zero when channel is disabled"]
    #[inline(always)]
    pub fn qdec_y_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, QdecYcntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,QdecYcntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecYcntReg {
    #[inline(always)]
    fn default() -> QdecYcntReg {
        <crate::RegValueT<QdecYcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecZcntReg_SPEC;
impl crate::sealed::RegSpec for QdecZcntReg_SPEC {
    type DataType = u16;
}

#[doc = "Counter value of the Z Axis"]
pub type QdecZcntReg = crate::RegValueT<QdecZcntReg_SPEC>;

impl QdecZcntReg {
    #[doc = "Contains a signed value of the events. Zero when channel is disabled"]
    #[inline(always)]
    pub fn qdec_z_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, QdecZcntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,QdecZcntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecZcntReg {
    #[inline(always)]
    fn default() -> QdecZcntReg {
        <crate::RegValueT<QdecZcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
