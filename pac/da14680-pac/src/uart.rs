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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:50 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"UART registers"]
unsafe impl ::core::marker::Send for super::Uart {}
unsafe impl ::core::marker::Sync for super::Uart {}
impl super::Uart {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn uart_cpr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartCprReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartCprReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_ctr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartCtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartCtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_dlf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartDlfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartDlfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_dmasa_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartDmasaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartDmasaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_ier_dlh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartIerDlhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartIerDlhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_iir_fcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartIirFcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartIirFcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_lcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartLcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartLcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_lsr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartLsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartLsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_mcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartMcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartMcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_rbr_thr_dll_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartRbrThrDllReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartRbrThrDllReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_sbcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSbcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSbcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_scr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartScrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartScrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_ucv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartUcvReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartUcvReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_usr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartUsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartUsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartCprReg_SPEC;
impl crate::sealed::RegSpec for UartCprReg_SPEC {
    type DataType = u16;
}

pub type UartCprReg = crate::RegValueT<UartCprReg_SPEC>;

impl UartCprReg {
    #[inline(always)]
    pub fn cpr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, UartCprReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,UartCprReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartCprReg {
    #[inline(always)]
    fn default() -> UartCprReg {
        <crate::RegValueT<UartCprReg_SPEC> as RegisterValue<_>>::new(14657)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartCtrReg_SPEC;
impl crate::sealed::RegSpec for UartCtrReg_SPEC {
    type DataType = u32;
}

pub type UartCtrReg = crate::RegValueT<UartCtrReg_SPEC>;

impl UartCtrReg {
    #[inline(always)]
    pub fn ctr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        UartCtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            UartCtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartCtrReg {
    #[inline(always)]
    fn default() -> UartCtrReg {
        <crate::RegValueT<UartCtrReg_SPEC> as RegisterValue<_>>::new(272)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartDlfReg_SPEC;
impl crate::sealed::RegSpec for UartDlfReg_SPEC {
    type DataType = u16;
}

pub type UartDlfReg = crate::RegValueT<UartDlfReg_SPEC>;

impl UartDlfReg {
    #[inline(always)]
    pub fn uart_dlf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UartDlfReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UartDlfReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartDlfReg {
    #[inline(always)]
    fn default() -> UartDlfReg {
        <crate::RegValueT<UartDlfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartDmasaReg_SPEC;
impl crate::sealed::RegSpec for UartDmasaReg_SPEC {
    type DataType = u16;
}

pub type UartDmasaReg = crate::RegValueT<UartDmasaReg_SPEC>;

impl UartDmasaReg {
    #[inline(always)]
    pub fn dmasa(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartDmasaReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,UartDmasaReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UartDmasaReg {
    #[inline(always)]
    fn default() -> UartDmasaReg {
        <crate::RegValueT<UartDmasaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartIerDlhReg_SPEC;
impl crate::sealed::RegSpec for UartIerDlhReg_SPEC {
    type DataType = u16;
}

pub type UartIerDlhReg = crate::RegValueT<UartIerDlhReg_SPEC>;

impl UartIerDlhReg {
    #[inline(always)]
    pub fn ptime_dlh7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dlh6_4(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UartIerDlhReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edssi_dlh3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn elsi_dhl2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn etbei_dlh1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn erbfi_dlh0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartIerDlhReg {
    #[inline(always)]
    fn default() -> UartIerDlhReg {
        <crate::RegValueT<UartIerDlhReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartIirFcrReg_SPEC;
impl crate::sealed::RegSpec for UartIirFcrReg_SPEC {
    type DataType = u16;
}

pub type UartIirFcrReg = crate::RegValueT<UartIirFcrReg_SPEC>;

impl UartIirFcrReg {
    #[inline(always)]
    pub fn iir_fcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, UartIirFcrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            UartIirFcrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartIirFcrReg {
    #[inline(always)]
    fn default() -> UartIirFcrReg {
        <crate::RegValueT<UartIirFcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartLcrReg_SPEC;
impl crate::sealed::RegSpec for UartLcrReg_SPEC {
    type DataType = u16;
}

pub type UartLcrReg = crate::RegValueT<UartLcrReg_SPEC>;

impl UartLcrReg {
    #[inline(always)]
    pub fn uart_dlab(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_bc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_eps(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_stop(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dls(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, UartLcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartLcrReg {
    #[inline(always)]
    fn default() -> UartLcrReg {
        <crate::RegValueT<UartLcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartLsrReg_SPEC;
impl crate::sealed::RegSpec for UartLsrReg_SPEC {
    type DataType = u16;
}

pub type UartLsrReg = crate::RegValueT<UartLsrReg_SPEC>;

impl UartLsrReg {
    #[inline(always)]
    pub fn uart_temt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_thre(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_bi(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_fe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_oe(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartLsrReg {
    #[inline(always)]
    fn default() -> UartLsrReg {
        <crate::RegValueT<UartLsrReg_SPEC> as RegisterValue<_>>::new(96)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartMcrReg_SPEC;
impl crate::sealed::RegSpec for UartMcrReg_SPEC {
    type DataType = u16;
}

pub type UartMcrReg = crate::RegValueT<UartMcrReg_SPEC>;

impl UartMcrReg {
    #[inline(always)]
    pub fn uart_sire(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_lb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_out2(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_out1(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartMcrReg {
    #[inline(always)]
    fn default() -> UartMcrReg {
        <crate::RegValueT<UartMcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartRbrThrDllReg_SPEC;
impl crate::sealed::RegSpec for UartRbrThrDllReg_SPEC {
    type DataType = u16;
}

pub type UartRbrThrDllReg = crate::RegValueT<UartRbrThrDllReg_SPEC>;

impl UartRbrThrDllReg {
    #[inline(always)]
    pub fn rbr_thr_dll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartRbrThrDllReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartRbrThrDllReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartRbrThrDllReg {
    #[inline(always)]
    fn default() -> UartRbrThrDllReg {
        <crate::RegValueT<UartRbrThrDllReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSbcrReg_SPEC;
impl crate::sealed::RegSpec for UartSbcrReg_SPEC {
    type DataType = u16;
}

pub type UartSbcrReg = crate::RegValueT<UartSbcrReg_SPEC>;

impl UartSbcrReg {
    #[inline(always)]
    pub fn uart_shadow_break_control(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSbcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartSbcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSbcrReg {
    #[inline(always)]
    fn default() -> UartSbcrReg {
        <crate::RegValueT<UartSbcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartScrReg_SPEC;
impl crate::sealed::RegSpec for UartScrReg_SPEC {
    type DataType = u16;
}

pub type UartScrReg = crate::RegValueT<UartScrReg_SPEC>;

impl UartScrReg {
    #[inline(always)]
    pub fn uart_scratch_pad(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartScrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UartScrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartScrReg {
    #[inline(always)]
    fn default() -> UartScrReg {
        <crate::RegValueT<UartScrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrrReg_SPEC;
impl crate::sealed::RegSpec for UartSrrReg_SPEC {
    type DataType = u16;
}

pub type UartSrrReg = crate::RegValueT<UartSrrReg_SPEC>;

impl UartSrrReg {
    #[inline(always)]
    pub fn uart_ur(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,UartSrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSrrReg {
    #[inline(always)]
    fn default() -> UartSrrReg {
        <crate::RegValueT<UartSrrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartUcvReg_SPEC;
impl crate::sealed::RegSpec for UartUcvReg_SPEC {
    type DataType = u32;
}

pub type UartUcvReg = crate::RegValueT<UartUcvReg_SPEC>;

impl UartUcvReg {
    #[inline(always)]
    pub fn ucv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        UartUcvReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            UartUcvReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartUcvReg {
    #[inline(always)]
    fn default() -> UartUcvReg {
        <crate::RegValueT<UartUcvReg_SPEC> as RegisterValue<_>>::new(13610)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartUsrReg_SPEC;
impl crate::sealed::RegSpec for UartUsrReg_SPEC {
    type DataType = u16;
}

pub type UartUsrReg = crate::RegValueT<UartUsrReg_SPEC>;

impl UartUsrReg {
    #[inline(always)]
    pub fn uart_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartUsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UartUsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartUsrReg {
    #[inline(always)]
    fn default() -> UartUsrReg {
        <crate::RegValueT<UartUsrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
