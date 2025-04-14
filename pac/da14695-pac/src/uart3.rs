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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:21 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"UART3 registers"]
unsafe impl ::core::marker::Send for super::Uart3 {}
unsafe impl ::core::marker::Sync for super::Uart3 {}
impl super::Uart3 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn uart3_config_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3ConfigReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3ConfigReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_ctr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3CtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3CtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_dlf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3DlfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3DlfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_dmasa_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3DmasaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3DmasaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_err_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3ErrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3ErrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_htx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3HtxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3HtxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_ier_dlh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3IerDlhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3IerDlhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_iir_fcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3IirFcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3IirFcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3IrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3IrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_lcr_ext(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3LcrExt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3LcrExt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_lcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3LcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3LcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_lsr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3LsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3LsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_mcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3McrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3McrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_msr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3MsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3MsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_rar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3RarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3RarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_rbr_thr_dll_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3RbrThrDllReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3RbrThrDllReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_rfl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3RflReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3RflReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_sbcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SbcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SbcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_sdmam_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SdmamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SdmamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_sfe_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SfeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SfeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srbr_sthr9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrbrSthr9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrbrSthr9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srts_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrtsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrtsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_srt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3SrtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3SrtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_stet_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3StetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3StetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_tar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3TarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3TarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_tfl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3TflReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3TflReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3TimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3TimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_ucv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3UcvReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3UcvReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart3_usr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart3UsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart3UsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3ConfigReg_SPEC;
impl crate::sealed::RegSpec for Uart3ConfigReg_SPEC {
    type DataType = u32;
}

pub type Uart3ConfigReg = crate::RegValueT<Uart3ConfigReg_SPEC>;

impl Uart3ConfigReg {
    #[inline(always)]
    pub fn iso7816_scratch_pad(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Uart3ConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Uart3ConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3ConfigReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3ConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_err_sig_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3ConfigReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3ConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_convention(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3ConfigReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3ConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3ConfigReg {
    #[inline(always)]
    fn default() -> Uart3ConfigReg {
        <crate::RegValueT<Uart3ConfigReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3CtrlReg_SPEC;
impl crate::sealed::RegSpec for Uart3CtrlReg_SPEC {
    type DataType = u32;
}

pub type Uart3CtrlReg = crate::RegValueT<Uart3CtrlReg_SPEC>;

impl Uart3CtrlReg {
    #[inline(always)]
    pub fn iso7816_auto_gt(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_err_tx_value_irqmask(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_err_tx_time_irqmask(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_tim_expired_irqmask(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_clk_status(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_clk_level(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_clk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Uart3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3CtrlReg {
    #[inline(always)]
    fn default() -> Uart3CtrlReg {
        <crate::RegValueT<Uart3CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3CtrReg_SPEC;
impl crate::sealed::RegSpec for Uart3CtrReg_SPEC {
    type DataType = u32;
}

pub type Uart3CtrReg = crate::RegValueT<Uart3CtrReg_SPEC>;

impl Uart3CtrReg {
    #[inline(always)]
    pub fn uart_ctr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Uart3CtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Uart3CtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3CtrReg {
    #[inline(always)]
    fn default() -> Uart3CtrReg {
        <crate::RegValueT<Uart3CtrReg_SPEC> as RegisterValue<_>>::new(1146552592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3DlfReg_SPEC;
impl crate::sealed::RegSpec for Uart3DlfReg_SPEC {
    type DataType = u32;
}

pub type Uart3DlfReg = crate::RegValueT<Uart3DlfReg_SPEC>;

impl Uart3DlfReg {
    #[inline(always)]
    pub fn uart_dlf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Uart3DlfReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Uart3DlfReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3DlfReg {
    #[inline(always)]
    fn default() -> Uart3DlfReg {
        <crate::RegValueT<Uart3DlfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3DmasaReg_SPEC;
impl crate::sealed::RegSpec for Uart3DmasaReg_SPEC {
    type DataType = u32;
}

pub type Uart3DmasaReg = crate::RegValueT<Uart3DmasaReg_SPEC>;

impl Uart3DmasaReg {
    #[inline(always)]
    pub fn uart_dmasa(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3DmasaReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3DmasaReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3DmasaReg {
    #[inline(always)]
    fn default() -> Uart3DmasaReg {
        <crate::RegValueT<Uart3DmasaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3ErrCtrlReg_SPEC;
impl crate::sealed::RegSpec for Uart3ErrCtrlReg_SPEC {
    type DataType = u32;
}

pub type Uart3ErrCtrlReg = crate::RegValueT<Uart3ErrCtrlReg_SPEC>;

impl Uart3ErrCtrlReg {
    #[inline(always)]
    pub fn iso7816_err_pulse_width(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, u8, Uart3ErrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1f,1,0,u8,u8,Uart3ErrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_err_pulse_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Uart3ErrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Uart3ErrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3ErrCtrlReg {
    #[inline(always)]
    fn default() -> Uart3ErrCtrlReg {
        <crate::RegValueT<Uart3ErrCtrlReg_SPEC> as RegisterValue<_>>::new(270)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3HtxReg_SPEC;
impl crate::sealed::RegSpec for Uart3HtxReg_SPEC {
    type DataType = u32;
}

pub type Uart3HtxReg = crate::RegValueT<Uart3HtxReg_SPEC>;

impl Uart3HtxReg {
    #[inline(always)]
    pub fn uart_halt_tx(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3HtxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3HtxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3HtxReg {
    #[inline(always)]
    fn default() -> Uart3HtxReg {
        <crate::RegValueT<Uart3HtxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3IerDlhReg_SPEC;
impl crate::sealed::RegSpec for Uart3IerDlhReg_SPEC {
    type DataType = u32;
}

pub type Uart3IerDlhReg = crate::RegValueT<Uart3IerDlhReg_SPEC>;

impl Uart3IerDlhReg {
    #[inline(always)]
    pub fn ptime_dlh7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dlh6_5(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Uart3IerDlhReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn elcolr_dlh4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edssi_dlh3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn elsi_dlh2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn etbei_dlh1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn erbfi_dlh0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3IerDlhReg {
    #[inline(always)]
    fn default() -> Uart3IerDlhReg {
        <crate::RegValueT<Uart3IerDlhReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3IirFcrReg_SPEC;
impl crate::sealed::RegSpec for Uart3IirFcrReg_SPEC {
    type DataType = u32;
}

pub type Uart3IirFcrReg = crate::RegValueT<Uart3IirFcrReg_SPEC>;

impl Uart3IirFcrReg {
    #[inline(always)]
    pub fn iir_fcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Uart3IirFcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Uart3IirFcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3IirFcrReg {
    #[inline(always)]
    fn default() -> Uart3IirFcrReg {
        <crate::RegValueT<Uart3IirFcrReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3IrqStatusReg_SPEC;
impl crate::sealed::RegSpec for Uart3IrqStatusReg_SPEC {
    type DataType = u32;
}

pub type Uart3IrqStatusReg = crate::RegValueT<Uart3IrqStatusReg_SPEC>;

impl Uart3IrqStatusReg {
    #[inline(always)]
    pub fn iso7816_err_tx_value_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3IrqStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3IrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_err_tx_time_irq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3IrqStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3IrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_tim_expired_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3IrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3IrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3IrqStatusReg {
    #[inline(always)]
    fn default() -> Uart3IrqStatusReg {
        <crate::RegValueT<Uart3IrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3LcrExt_SPEC;
impl crate::sealed::RegSpec for Uart3LcrExt_SPEC {
    type DataType = u32;
}

pub type Uart3LcrExt = crate::RegValueT<Uart3LcrExt_SPEC>;

impl Uart3LcrExt {
    #[inline(always)]
    pub fn uart_transmit_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3LcrExt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3LcrExt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_send_addr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3LcrExt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3LcrExt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_addr_match(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3LcrExt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3LcrExt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dls_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3LcrExt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3LcrExt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3LcrExt {
    #[inline(always)]
    fn default() -> Uart3LcrExt {
        <crate::RegValueT<Uart3LcrExt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3LcrReg_SPEC;
impl crate::sealed::RegSpec for Uart3LcrReg_SPEC {
    type DataType = u32;
}

pub type Uart3LcrReg = crate::RegValueT<Uart3LcrReg_SPEC>;

impl Uart3LcrReg {
    #[inline(always)]
    pub fn uart_dlab(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_bc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_sp(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_eps(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_stop(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dls(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Uart3LcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3LcrReg {
    #[inline(always)]
    fn default() -> Uart3LcrReg {
        <crate::RegValueT<Uart3LcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3LsrReg_SPEC;
impl crate::sealed::RegSpec for Uart3LsrReg_SPEC {
    type DataType = u32;
}

pub type Uart3LsrReg = crate::RegValueT<Uart3LsrReg_SPEC>;

impl Uart3LsrReg {
    #[inline(always)]
    pub fn uart_addr_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rfe(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_temt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_thre(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_bi(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_fe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_oe(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3LsrReg {
    #[inline(always)]
    fn default() -> Uart3LsrReg {
        <crate::RegValueT<Uart3LsrReg_SPEC> as RegisterValue<_>>::new(96)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3McrReg_SPEC;
impl crate::sealed::RegSpec for Uart3McrReg_SPEC {
    type DataType = u32;
}

pub type Uart3McrReg = crate::RegValueT<Uart3McrReg_SPEC>;

impl Uart3McrReg {
    #[inline(always)]
    pub fn uart_afce(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_lb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rts(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3McrReg {
    #[inline(always)]
    fn default() -> Uart3McrReg {
        <crate::RegValueT<Uart3McrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3MsrReg_SPEC;
impl crate::sealed::RegSpec for Uart3MsrReg_SPEC {
    type DataType = u32;
}

pub type Uart3MsrReg = crate::RegValueT<Uart3MsrReg_SPEC>;

impl Uart3MsrReg {
    #[inline(always)]
    pub fn uart_cts(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dcts(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3MsrReg {
    #[inline(always)]
    fn default() -> Uart3MsrReg {
        <crate::RegValueT<Uart3MsrReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3RarReg_SPEC;
impl crate::sealed::RegSpec for Uart3RarReg_SPEC {
    type DataType = u32;
}

pub type Uart3RarReg = crate::RegValueT<Uart3RarReg_SPEC>;

impl Uart3RarReg {
    #[inline(always)]
    pub fn uart_rar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Uart3RarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Uart3RarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3RarReg {
    #[inline(always)]
    fn default() -> Uart3RarReg {
        <crate::RegValueT<Uart3RarReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3RbrThrDllReg_SPEC;
impl crate::sealed::RegSpec for Uart3RbrThrDllReg_SPEC {
    type DataType = u32;
}

pub type Uart3RbrThrDllReg = crate::RegValueT<Uart3RbrThrDllReg_SPEC>;

impl Uart3RbrThrDllReg {
    #[inline(always)]
    pub fn rbr_thr_9bit(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Uart3RbrThrDllReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Uart3RbrThrDllReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rbr_thr_dll(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3RbrThrDllReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3RbrThrDllReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3RbrThrDllReg {
    #[inline(always)]
    fn default() -> Uart3RbrThrDllReg {
        <crate::RegValueT<Uart3RbrThrDllReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3RflReg_SPEC;
impl crate::sealed::RegSpec for Uart3RflReg_SPEC {
    type DataType = u32;
}

pub type Uart3RflReg = crate::RegValueT<Uart3RflReg_SPEC>;

impl Uart3RflReg {
    #[inline(always)]
    pub fn uart_receive_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Uart3RflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Uart3RflReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3RflReg {
    #[inline(always)]
    fn default() -> Uart3RflReg {
        <crate::RegValueT<Uart3RflReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SbcrReg_SPEC;
impl crate::sealed::RegSpec for Uart3SbcrReg_SPEC {
    type DataType = u32;
}

pub type Uart3SbcrReg = crate::RegValueT<Uart3SbcrReg_SPEC>;

impl Uart3SbcrReg {
    #[inline(always)]
    pub fn uart_shadow_break_control(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3SbcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3SbcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3SbcrReg {
    #[inline(always)]
    fn default() -> Uart3SbcrReg {
        <crate::RegValueT<Uart3SbcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SdmamReg_SPEC;
impl crate::sealed::RegSpec for Uart3SdmamReg_SPEC {
    type DataType = u32;
}

pub type Uart3SdmamReg = crate::RegValueT<Uart3SdmamReg_SPEC>;

impl Uart3SdmamReg {
    #[inline(always)]
    pub fn uart_shadow_dma_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3SdmamReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3SdmamReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3SdmamReg {
    #[inline(always)]
    fn default() -> Uart3SdmamReg {
        <crate::RegValueT<Uart3SdmamReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SfeReg_SPEC;
impl crate::sealed::RegSpec for Uart3SfeReg_SPEC {
    type DataType = u32;
}

pub type Uart3SfeReg = crate::RegValueT<Uart3SfeReg_SPEC>;

impl Uart3SfeReg {
    #[inline(always)]
    pub fn uart_shadow_fifo_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3SfeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3SfeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3SfeReg {
    #[inline(always)]
    fn default() -> Uart3SfeReg {
        <crate::RegValueT<Uart3SfeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr0Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr0Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr0Reg = crate::RegValueT<Uart3SrbrSthr0Reg_SPEC>;

impl Uart3SrbrSthr0Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr0Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr0Reg {
        <crate::RegValueT<Uart3SrbrSthr0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr10Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr10Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr10Reg = crate::RegValueT<Uart3SrbrSthr10Reg_SPEC>;

impl Uart3SrbrSthr10Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr10Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr10Reg {
        <crate::RegValueT<Uart3SrbrSthr10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr11Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr11Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr11Reg = crate::RegValueT<Uart3SrbrSthr11Reg_SPEC>;

impl Uart3SrbrSthr11Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr11Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr11Reg {
        <crate::RegValueT<Uart3SrbrSthr11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr12Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr12Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr12Reg = crate::RegValueT<Uart3SrbrSthr12Reg_SPEC>;

impl Uart3SrbrSthr12Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr12Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr12Reg {
        <crate::RegValueT<Uart3SrbrSthr12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr13Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr13Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr13Reg = crate::RegValueT<Uart3SrbrSthr13Reg_SPEC>;

impl Uart3SrbrSthr13Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr13Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr13Reg {
        <crate::RegValueT<Uart3SrbrSthr13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr14Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr14Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr14Reg = crate::RegValueT<Uart3SrbrSthr14Reg_SPEC>;

impl Uart3SrbrSthr14Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr14Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr14Reg {
        <crate::RegValueT<Uart3SrbrSthr14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr15Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr15Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr15Reg = crate::RegValueT<Uart3SrbrSthr15Reg_SPEC>;

impl Uart3SrbrSthr15Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr15Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr15Reg {
        <crate::RegValueT<Uart3SrbrSthr15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr1Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr1Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr1Reg = crate::RegValueT<Uart3SrbrSthr1Reg_SPEC>;

impl Uart3SrbrSthr1Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr1Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr1Reg {
        <crate::RegValueT<Uart3SrbrSthr1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr2Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr2Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr2Reg = crate::RegValueT<Uart3SrbrSthr2Reg_SPEC>;

impl Uart3SrbrSthr2Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr2Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr2Reg {
        <crate::RegValueT<Uart3SrbrSthr2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr3Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr3Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr3Reg = crate::RegValueT<Uart3SrbrSthr3Reg_SPEC>;

impl Uart3SrbrSthr3Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr3Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr3Reg {
        <crate::RegValueT<Uart3SrbrSthr3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr4Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr4Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr4Reg = crate::RegValueT<Uart3SrbrSthr4Reg_SPEC>;

impl Uart3SrbrSthr4Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr4Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr4Reg {
        <crate::RegValueT<Uart3SrbrSthr4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr5Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr5Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr5Reg = crate::RegValueT<Uart3SrbrSthr5Reg_SPEC>;

impl Uart3SrbrSthr5Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr5Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr5Reg {
        <crate::RegValueT<Uart3SrbrSthr5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr6Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr6Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr6Reg = crate::RegValueT<Uart3SrbrSthr6Reg_SPEC>;

impl Uart3SrbrSthr6Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr6Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr6Reg {
        <crate::RegValueT<Uart3SrbrSthr6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr7Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr7Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr7Reg = crate::RegValueT<Uart3SrbrSthr7Reg_SPEC>;

impl Uart3SrbrSthr7Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr7Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr7Reg {
        <crate::RegValueT<Uart3SrbrSthr7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr8Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr8Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr8Reg = crate::RegValueT<Uart3SrbrSthr8Reg_SPEC>;

impl Uart3SrbrSthr8Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr8Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr8Reg {
        <crate::RegValueT<Uart3SrbrSthr8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrbrSthr9Reg_SPEC;
impl crate::sealed::RegSpec for Uart3SrbrSthr9Reg_SPEC {
    type DataType = u32;
}

pub type Uart3SrbrSthr9Reg = crate::RegValueT<Uart3SrbrSthr9Reg_SPEC>;

impl Uart3SrbrSthr9Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart3SrbrSthr9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart3SrbrSthr9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3SrbrSthr9Reg {
    #[inline(always)]
    fn default() -> Uart3SrbrSthr9Reg {
        <crate::RegValueT<Uart3SrbrSthr9Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrrReg_SPEC;
impl crate::sealed::RegSpec for Uart3SrrReg_SPEC {
    type DataType = u32;
}

pub type Uart3SrrReg = crate::RegValueT<Uart3SrrReg_SPEC>;

impl Uart3SrrReg {
    #[inline(always)]
    pub fn uart_xfr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rfr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_ur(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3SrrReg {
    #[inline(always)]
    fn default() -> Uart3SrrReg {
        <crate::RegValueT<Uart3SrrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrtsReg_SPEC;
impl crate::sealed::RegSpec for Uart3SrtsReg_SPEC {
    type DataType = u32;
}

pub type Uart3SrtsReg = crate::RegValueT<Uart3SrtsReg_SPEC>;

impl Uart3SrtsReg {
    #[inline(always)]
    pub fn uart_shadow_request_to_send(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3SrtsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3SrtsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3SrtsReg {
    #[inline(always)]
    fn default() -> Uart3SrtsReg {
        <crate::RegValueT<Uart3SrtsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3SrtReg_SPEC;
impl crate::sealed::RegSpec for Uart3SrtReg_SPEC {
    type DataType = u32;
}

pub type Uart3SrtReg = crate::RegValueT<Uart3SrtReg_SPEC>;

impl Uart3SrtReg {
    #[inline(always)]
    pub fn uart_shadow_rcvr_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Uart3SrtReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Uart3SrtReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3SrtReg {
    #[inline(always)]
    fn default() -> Uart3SrtReg {
        <crate::RegValueT<Uart3SrtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3StetReg_SPEC;
impl crate::sealed::RegSpec for Uart3StetReg_SPEC {
    type DataType = u32;
}

pub type Uart3StetReg = crate::RegValueT<Uart3StetReg_SPEC>;

impl Uart3StetReg {
    #[inline(always)]
    pub fn uart_shadow_tx_empty_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Uart3StetReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Uart3StetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3StetReg {
    #[inline(always)]
    fn default() -> Uart3StetReg {
        <crate::RegValueT<Uart3StetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3TarReg_SPEC;
impl crate::sealed::RegSpec for Uart3TarReg_SPEC {
    type DataType = u32;
}

pub type Uart3TarReg = crate::RegValueT<Uart3TarReg_SPEC>;

impl Uart3TarReg {
    #[inline(always)]
    pub fn uart_tar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Uart3TarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Uart3TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3TarReg {
    #[inline(always)]
    fn default() -> Uart3TarReg {
        <crate::RegValueT<Uart3TarReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3TflReg_SPEC;
impl crate::sealed::RegSpec for Uart3TflReg_SPEC {
    type DataType = u32;
}

pub type Uart3TflReg = crate::RegValueT<Uart3TflReg_SPEC>;

impl Uart3TflReg {
    #[inline(always)]
    pub fn uart_transmit_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Uart3TflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Uart3TflReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3TflReg {
    #[inline(always)]
    fn default() -> Uart3TflReg {
        <crate::RegValueT<Uart3TflReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3TimerReg_SPEC;
impl crate::sealed::RegSpec for Uart3TimerReg_SPEC {
    type DataType = u32;
}

pub type Uart3TimerReg = crate::RegValueT<Uart3TimerReg_SPEC>;

impl Uart3TimerReg {
    #[inline(always)]
    pub fn iso7816_tim_mode(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Uart3TimerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Uart3TimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_tim_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Uart3TimerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Uart3TimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iso7816_tim_max(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Uart3TimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Uart3TimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3TimerReg {
    #[inline(always)]
    fn default() -> Uart3TimerReg {
        <crate::RegValueT<Uart3TimerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3UcvReg_SPEC;
impl crate::sealed::RegSpec for Uart3UcvReg_SPEC {
    type DataType = u32;
}

pub type Uart3UcvReg = crate::RegValueT<Uart3UcvReg_SPEC>;

impl Uart3UcvReg {
    #[inline(always)]
    pub fn uart_ucv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Uart3UcvReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Uart3UcvReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uart3UcvReg {
    #[inline(always)]
    fn default() -> Uart3UcvReg {
        <crate::RegValueT<Uart3UcvReg_SPEC> as RegisterValue<_>>::new(875573546)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3UsrReg_SPEC;
impl crate::sealed::RegSpec for Uart3UsrReg_SPEC {
    type DataType = u32;
}

pub type Uart3UsrReg = crate::RegValueT<Uart3UsrReg_SPEC>;

impl Uart3UsrReg {
    #[inline(always)]
    pub fn uart_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart3UsrReg {
    #[inline(always)]
    fn default() -> Uart3UsrReg {
        <crate::RegValueT<Uart3UsrReg_SPEC> as RegisterValue<_>>::new(6)
    }
}
