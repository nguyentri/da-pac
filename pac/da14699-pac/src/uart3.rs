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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:54:18 +0000

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
    #[doc = "ISO7816 Config Register"]
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

    #[doc = "ISO7816 Control Register"]
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

    #[doc = "Component Type Register"]
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

    #[doc = "Divisor Latch Fraction Register"]
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

    #[doc = "DMA Software Acknowledge"]
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

    #[doc = "ISO7816 Error Signal Control Register"]
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

    #[doc = "Halt TX"]
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

    #[doc = "Interrupt Enable Register"]
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

    #[doc = "Interrupt Identification Register/FIFO Control Register"]
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

    #[doc = "ISO7816 Interrupt Status Register"]
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

    #[doc = "Line Extended Control Register"]
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

    #[doc = "Line Control Register"]
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

    #[doc = "Line Status Register"]
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

    #[doc = "Modem Control Register"]
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

    #[doc = "Modem Status Register"]
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

    #[doc = "Receive Address Register"]
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

    #[doc = "Receive Buffer Register"]
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

    #[doc = "Receive FIFO Level."]
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

    #[doc = "Shadow Break Control Register"]
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

    #[doc = "Shadow DMA Mode"]
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

    #[doc = "Shadow FIFO Enable"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Shadow Receive/Transmit Buffer Register"]
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

    #[doc = "Software Reset Register."]
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

    #[doc = "Shadow Request to Send"]
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

    #[doc = "Shadow RCVR Trigger"]
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

    #[doc = "Shadow TX Empty Trigger"]
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

    #[doc = "Transmit Address Register"]
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

    #[doc = "Transmit FIFO Level"]
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

    #[doc = "ISO7816 Timer Register"]
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

    #[doc = "Component Version"]
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

    #[doc = "UART Status register."]
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
#[doc = "ISO7816 Config Register"]
pub type Uart3ConfigReg = crate::RegValueT<Uart3ConfigReg_SPEC>;

impl Uart3ConfigReg {
    #[doc = "This register is for programmers to use as a temporary storage space. It has no defined purpose in the UART Ctrl."]
    #[inline(always)]
    pub fn iso7816_scratch_pad(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Uart3ConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Uart3ConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : Normal Uart\n1 : ISO7816 Enabled"]
    #[inline(always)]
    pub fn iso7816_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3ConfigReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3ConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : Error Signal feature disabled\n1 : Error Signal feature enabled"]
    #[inline(always)]
    pub fn iso7816_err_sig_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3ConfigReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3ConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : Direct convention\n1 : Inverse convention"]
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
#[doc = "ISO7816 Control Register"]
pub type Uart3CtrlReg = crate::RegValueT<Uart3CtrlReg_SPEC>;

impl Uart3CtrlReg {
    #[doc = "0 : uart sends when tx data is available\n1 : uart sends new character after guard time"]
    #[inline(always)]
    pub fn iso7816_auto_gt(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : ERR_TX_VALUE IRQ is masked\n1 : ERR_TX_VALUE IRQ is enabled"]
    #[inline(always)]
    pub fn iso7816_err_tx_value_irqmask(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : ERR_TX_TIME IRQ is masked\n1 : ERR_TX_TIME IRQ is enabled"]
    #[inline(always)]
    pub fn iso7816_err_tx_time_irqmask(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : timer expired IRQ is masked\n1 : timer expired IRQ is enabled"]
    #[inline(always)]
    pub fn iso7816_tim_expired_irqmask(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : iso7816 clock is stopped\n1 : iso7816 clock is running"]
    #[inline(always)]
    pub fn iso7816_clk_status(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 : iso7816 clock level low when stopped\n1 : iso7816 clock level high when stopped"]
    #[inline(always)]
    pub fn iso7816_clk_level(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : iso7816 clock disabled\n1 : iso7816 clock enabled"]
    #[inline(always)]
    pub fn iso7816_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ISO7816 clk freq = sclk/(2*(ISO7816_CLK_DIV+1)"]
    #[inline(always)]
    pub fn iso7816_clk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Uart3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Uart3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Component Type Register"]
pub type Uart3CtrReg = crate::RegValueT<Uart3CtrReg_SPEC>;

impl Uart3CtrReg {
    #[doc = "Component Type Register"]
    #[inline(always)]
    pub fn uart_ctr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Uart3CtrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Uart3CtrReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Divisor Latch Fraction Register"]
pub type Uart3DlfReg = crate::RegValueT<Uart3DlfReg_SPEC>;

impl Uart3DlfReg {
    #[doc = "The fractional value is added to integer value set by DLH, DLL. Fractional value is equal UART_DLF/16"]
    #[inline(always)]
    pub fn uart_dlf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Uart3DlfReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Uart3DlfReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "DMA Software Acknowledge"]
pub type Uart3DmasaReg = crate::RegValueT<Uart3DmasaReg_SPEC>;

impl Uart3DmasaReg {
    #[doc = "This register is use to perform DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This will cause the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is \'self-clearing\' and it is not necessary to clear this bit."]
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
#[doc = "ISO7816 Error Signal Control Register"]
pub type Uart3ErrCtrlReg = crate::RegValueT<Uart3ErrCtrlReg_SPEC>;

impl Uart3ErrCtrlReg {
    #[doc = "When Error Signal feature is enable and receive mode, it gives the width of the error signal in 1/16etu"]
    #[inline(always)]
    pub fn iso7816_err_pulse_width(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Uart3ErrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Uart3ErrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When Error Signal feature is enable and receive mode, it gives the offset of the error signal in 1/16etu from the 9.6etu"]
    #[inline(always)]
    pub fn iso7816_err_pulse_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Uart3ErrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, Uart3ErrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Halt TX"]
pub type Uart3HtxReg = crate::RegValueT<Uart3HtxReg_SPEC>;

impl Uart3HtxReg {
    #[doc = "This register is use to halt transmissions, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled.\n0 = Halt TX disabled\n1 = Halt TX enabled\nNote, if FIFOs are not enabled, the setting of the halt TX register has no effect on operation."]
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
#[doc = "Interrupt Enable Register"]
pub type Uart3IerDlhReg = crate::RegValueT<Uart3IerDlhReg_SPEC>;

impl Uart3IerDlhReg {
    #[doc = "Interrupt Enable Register: PTIME, Programmable THRE Interrupt Mode Enable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled \nDivisor Latch (High): Bit\\[7\\] of the 8 bit DLH register."]
    #[inline(always)]
    pub fn ptime_dlh7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divisor Latch (High): Bit\\[6:5\\] of the 8 bit DLH register"]
    #[inline(always)]
    pub fn dlh6_5(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Uart3IerDlhReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8, Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: ELCOLR (read only), this bit controls the method for clearing the status in the LSR register. This is applicable only for Overrun Error, Parity Error, Framing Error, and Break Interrupt status bits.\n0 = LSR status bits are cleared either on reading Rx FIFO (RBR Read) or On reading LSR register.\nDivisor Latch (High): Bit\\[4\\] of the 8 bit DLH register"]
    #[inline(always)]
    pub fn elcolr_dlh4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: EDSSI, Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled\nDivisor Latch (High): Bit\\[3\\] of the 8 bit DLH register"]
    #[inline(always)]
    pub fn edssi_dlh3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: ELSI, Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled\nDivisor Latch (High): Bit\\[2\\] of the 8 bit DLH register."]
    #[inline(always)]
    pub fn elsi_dlh2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: ETBEI, Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled \nDivisor Latch (High): Bit\\[1\\] of the 8 bit DLH register."]
    #[inline(always)]
    pub fn etbei_dlh1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: ERBFI, Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFO\'s enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled\nDivisor Latch (High): Bit\\[0\\] of the 8 bit DLH register."]
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
#[doc = "Interrupt Identification Register/FIFO Control Register"]
pub type Uart3IirFcrReg = crate::RegValueT<Uart3IirFcrReg_SPEC>;

impl Uart3IirFcrReg {
    #[doc = "On Read Interrupt Identification Register :\nBits\\[7:6\\], FIFO\'s Enabled (or FIFOSE): This is used to indicate whether the FIFO\'s are enabled or disabled. 00 = disabled. 11 = enabled.\nBits\\[5:4\\],Reserved\nBits\\[3:0\\], Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types:0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout.\nOn Write FIFO Control Register\nBits\\[7:6\\], RCVR Trigger (or RT):. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt will be generated. In auto flow control mode it is used to determine when the rts_n signal will be de-asserted. It also determines when the dma_rx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than full\nBits\\[5:4\\], TX Empty Trigger (or TET): This is used to select the empty threshold level at which the THRE Interrupts will be generated when the mode is active. It also determines when the dma_tx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO 1/4 full 11 = FIFO 1/2 full\nBit\\[3\\], DMA Mode (or DMAM): This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals. 0 = mode 0 1 = mode 1\nBit\\[2\\], XMIT FIFO Reset (or XFIFOR): This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is \'self-clearing\' and it is not necessary to clear this bit.\nBit\\[1\\], RCVR FIFO Reset (or RFIFOR): This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is \'self-clearing\' and it is not necessary to clear this bit.\nBit\\[0\\], FIFO Enable (or FIFOE): This enables/disables the transmit (XMIT) and receive (RCVR) FIFO\'s. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFO\'s will be reset."]
    #[inline(always)]
    pub fn iir_fcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3IirFcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3IirFcrReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "ISO7816 Interrupt Status Register"]
pub type Uart3IrqStatusReg = crate::RegValueT<Uart3IrqStatusReg_SPEC>;

impl Uart3IrqStatusReg {
    #[doc = "On read\n1 : : If error signal is enabled and in transmit mode, module generates IRQ when receiver does not receive correctly the character\nOn Write\n1 : Clear IRQ"]
    #[inline(always)]
    pub fn iso7816_err_tx_value_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3IrqStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3IrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "On read\n1 : If error signal is enabled and in transmit mode, module generates IRQ when it checks the error signal\nOn Write\n1 : Clear IRQ"]
    #[inline(always)]
    pub fn iso7816_err_tx_time_irq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3IrqStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3IrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "On read\n1 : when Timer is expired. Timer has to be disabled to clear the IRQ. When sclk is lower than pclk then this bit has to be checked if it\'s cleared before return form the IRQ Handler"]
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
#[doc = "Line Extended Control Register"]
pub type Uart3LcrExt = crate::RegValueT<Uart3LcrExt_SPEC>;

impl Uart3LcrExt {
    #[doc = "Transmit mode control bit. This bit is used to control the type of transmit mode during 9-bit data transfers.\n1 = In this mode of operation, Transmit Holding Register (THR) and Shadow Transmit Holding Register (STHR) are 9-bit wide. The user needs to ensure that the THR/STHR register is written correctly for address/data.\nAddress: 9th bit is set to 1,\nData : 9th bit is set to 0.\nNote: Transmit address register (TAR) is not applicable in this mode of operation.\n0 = In this mode of operation, Transmit Holding Register (THR) and Shadow Transmit Holding register (STHR) are 8-bit wide. The user needs to program the address into Transmit Address Register (TAR) and data into the THR/STHR register. SEND_ADDR bit is used as a control knob to indicate the uart on when to send the address."]
    #[inline(always)]
    pub fn uart_transmit_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3LcrExt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3LcrExt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Send address control bit. This bit is used as a control knob for the user to determine when to send the address during transmit mode.\n1 = 9-bit character will be transmitted with 9-th bit set to 1 and the remaining 8-bits will match to what is being programmed in \'Transmit Address Register\'.\n0 = 9-bit character will be transmitted with 9-th bit set to 0 and the remaining 8-bits will be taken from the TXFIFO which is programmed through 8-bit wide THR/STHR register.\nNote:\n1. This bit is auto-cleared by the hardware, after sending out the address character. User is not expected to program this bit to 0.\n2. This field is applicable only when DLS_E bit is set to 1 and TRANSMIT_MODE is set to 0."]
    #[inline(always)]
    pub fn uart_send_addr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3LcrExt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3LcrExt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Address Match Mode.This bit is used to enable the address match feature during receive.\n1 = Address match mode; uart will wait until the incoming character with 9-th bit set to 1. And further checks to see if the address matches with what is programmed in \'Receive Address Match Register\'. If match is found, then sub-sequent characters will be treated as valid data and DW_apb_uart starts receiving data.\n0 = Normal mode; DW_apb_uart will start to receive the data and 9-bit character will be formed and written into the receive RXFIFO. User is responsible to read the data and differentiate b/n address and data.\nNote: This field is applicable only when DLS_E is set to 1."]
    #[inline(always)]
    pub fn uart_addr_match(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3LcrExt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3LcrExt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extension for DLS. This bit is used to enable 9-bit data for transmit and receive transfers."]
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
#[doc = "Line Control Register"]
pub type Uart3LcrReg = crate::RegValueT<Uart3LcrReg_SPEC>;

impl Uart3LcrReg {
    #[doc = "Divisor Latch Access Bit.\nThis bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART.\nThis bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    pub fn uart_dlab(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Control Bit.\nThis is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared."]
    #[inline(always)]
    pub fn uart_bc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stick Parity. (writeable only when UART is not busy USR\\[0\\] is 0); otherwise always writable and always readable. This bit is used to force parity value. When PEN, EPS and Stick Parity are set to 1, the parity bit is transmitted and checked as logic 0. If PEN and Stick Parity are set to 1 and EPS is a logic 0, then parity bit is transmitted and checked as a logic 1. If this bit is set to 0, Stick Parity is disabled."]
    #[inline(always)]
    pub fn uart_sp(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Even Parity Select. Writeable only when UART is not busy (USR\\[0\\] is zero).\nThis is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    pub fn uart_eps(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Enable. Writeable only when UART is not busy (USR\\[0\\] is zero)\nThis bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively.\n0 = parity disabled\n1 = parity enabled"]
    #[inline(always)]
    pub fn uart_pen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of stop bits.\nThis is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data.\nIf set to one and the data bits are set to 5 (LCR\\[1:0\\] set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit.\n0 = 1 stop bit\n1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
    #[inline(always)]
    pub fn uart_stop(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Length Select.\nThis is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows:\n00 = 5 bits\n01 = 6 bits\n10 = 7 bits\n11 = 8 bits"]
    #[inline(always)]
    pub fn uart_dls(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Uart3LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Uart3LcrReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Line Status Register"]
pub type Uart3LsrReg = crate::RegValueT<Uart3LsrReg_SPEC>;

impl Uart3LsrReg {
    #[doc = "Address Received Bit.\nIf 9Bit data mode (LCR_EXT\\[0\\]=1) is enabled, this bit is used to indicate the 9th bit of the receive data is set to 1. This bit can also be used to indicate whether the incoming character is address or data.\n1 = Indicates the character is address.\n0 = Indicates the character is data.\nIn the FIFO mode, since the 9th bit is associated with a character received, it is revealed when the character with the 9th bit set to 1 is at the top of the FIFO.\nReading the LSR clears the 9BIT.\nNote: User needs to ensure that interrupt gets cleared (reading LSR register) before the next address byte arrives. If there is a delay in clearing the interrupt, then Software will not be able to distinguish between multiple address related interrupt."]
    #[inline(always)]
    pub fn uart_addr_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receiver FIFO Error bit.\nThis bit is only relevant when FIFOs are enabled (FCR\\[0\\] set to one). This is used to indicate if there is at least one parity error, framing error, or break indication in the FIFO.\n0 = no error in RX FIFO\n1 = error in RX FIFO\nThis bit is cleared when the LSR is read and the character with the error is at the top of the receiver FIFO and there are no subsequent errors in the FIFO."]
    #[inline(always)]
    pub fn uart_rfe(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmitter Empty bit.\nIf FIFOs enabled (FCR\\[0\\] set to one), this bit is set whenever the Transmitter Shift Register and the FIFO are both empty. If FIFOs are disabled, this bit is set whenever the Transmitter Holding Register and the Transmitter Shift Register are both empty."]
    #[inline(always)]
    pub fn uart_temt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Holding Register Empty bit.\nIf THRE mode is disabled (IER\\[7\\] set to zero) and regardless of FIFO\'s being implemented/enabled or not, this bit indicates that the THR or TX FIFO is empty.\nThis bit is set whenever data is transferred from the THR or TX FIFO to the transmitter shift register and no new data has been written to the THR or TX FIFO. This also causes a THRE Interrupt to occur, if the THRE Interrupt is enabled. If both modes are active (IER\\[7\\] set to one and FCR\\[0\\] set to one respectively), the functionality is switched to indicate the transmitter FIFO is full, and no longer controls THRE interrupts, which are then controlled by the FCR\\[5:4\\] threshold setting."]
    #[inline(always)]
    pub fn uart_thre(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Break Interrupt bit.\nThis is used to indicate the detection of a break sequence on the serial input data.\nIf in UART mode (SIR_MODE == Disabled), it is set whenever the serial input, sin, is held in a logic \'0\' state for longer than the sum of start time + data bits + parity + stop bits.\nIn the FIFO mode, the character associated with the break condition is carried through the FIFO and is revealed when the character is at the top of the FIFO.\nReading the LSR clears the BI bit. In the non-FIFO mode, the BI indication occurs immediately and persists until the LSR is read."]
    #[inline(always)]
    pub fn uart_bi(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Framing Error bit.\nThis is used to indicate the occurrence of a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data.\nIn the FIFO mode, since the framing error is associated with a character received, it is revealed when the character with the framing error is at the top of the FIFO.\nWhen a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues receiving the other bit i.e. data, and/or parity and stop. It should be noted that the Framing Error (FE) bit (LSR\\[3\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]).\n0 = no framing error\n1 = framing error\nReading the LSR clears the FE bit."]
    #[inline(always)]
    pub fn uart_fe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Parity Error bit.\nThis is used to indicate the occurrence of a parity error in the receiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set.\nIn the FIFO mode, since the parity error is associated with a character received, it is revealed when the character with the parity error arrives at the top of the FIFO.\nIt should be noted that the Parity Error (PE) bit (LSR\\[2\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]).\n0 = no parity error\n1 = parity error\nReading the LSR clears the PE bit."]
    #[inline(always)]
    pub fn uart_pe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Overrun error bit.\nThis is used to indicate the occurrence of an overrun error.\nThis occurs if a new data character was received before the previous data was read.\nIn the non-FIFO mode, the OE bit is set when a new character arrives in the receiver before the previous character was read from the RBR. When this happens, the data in the RBR is overwritten. In the FIFO mode, an overrun error occurs when the FIFO is full and a new character arrives at the receiver. The data in the FIFO is retained and the data in the receive shift register is lost.\n0 = no overrun error\n1 = overrun error\nReading the LSR clears the OE bit."]
    #[inline(always)]
    pub fn uart_oe(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Ready bit.\nThis is used to indicate that the receiver contains at least one character in the RBR or the receiver FIFO.\n0 = no data ready\n1 = data ready\nThis bit is cleared when the RBR is read in non-FIFO mode, or when the receiver FIFO is empty, in FIFO mode."]
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
#[doc = "Modem Control Register"]
pub type Uart3McrReg = crate::RegValueT<Uart3McrReg_SPEC>;

impl Uart3McrReg {
    #[doc = "Auto Flow Control Enable.\nWhen FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled as described in \"Auto Flow Control\".\n0 = Auto Flow Control Mode disabled\n1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    pub fn uart_afce(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart3McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart3McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LoopBack Bit.\nThis is used to put the UART into a diagnostic mode for test purposes.\nData on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n) are looped back to the inputs, internally."]
    #[inline(always)]
    pub fn uart_lb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Request to Send.\nThis is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data.\nWhen Auto RTS Flow Control is not enabled (MCR\\[5\\] set to zero), the rts_n signal is set low by programming MCR\\[1\\] (RTS) to a high.In Auto Flow Control, active (MCR\\[5\\] set to one) and FIFOs enable (FCR\\[0\\] set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\] is set low.\nNote that in Loopback mode (MCR\\[4\\] set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
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
#[doc = "Modem Status Register"]
pub type Uart3MsrReg = crate::RegValueT<Uart3MsrReg_SPEC>;

impl Uart3MsrReg {
    #[doc = "Clear to Send.\nThis is used to indicate the current state of the modem control line cts_n. This bit is the complement of cts_n. When the Clear to Send input (cts_n) is asserted it is an indication that the modem or data set is ready to exchange data with the UART Ctrl.\n0 = cts_n input is de-asserted (logic 1)\n1 = cts_n input is asserted (logic 0)\nIn Loopback Mode (MCR\\[4\\] = 1), CTS is the same as MCR\\[1\\] (RTS)."]
    #[inline(always)]
    pub fn uart_cts(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Delta Clear to Send.\nThis is used to indicate that the modem control line cts_n has changed since the last time the MSR was read.\n0 = no change on cts_n since last read of MSR\n1 = change on cts_n since last read of MSR\nReading the MSR clears the DCTS bit. In Loopback Mode (MCR\\[4\\] = 1), DCTS reflects changes on MCR\\[1\\] (RTS).\nNote, if the DCTS bit is not set and the cts_n signal is asserted (low) and a reset occurs (software or otherwise), then the DCTS bit is set when the reset is removed if the cts_n signal remains asserted."]
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
#[doc = "Receive Address Register"]
pub type Uart3RarReg = crate::RegValueT<Uart3RarReg_SPEC>;

impl Uart3RarReg {
    #[doc = "This is an address matching register during receive mode. If the 9-th bit is set in the incoming character then the remaining 8-bits will be checked against this register value. If the match happens then sub-sequent characters with 9-th bit set to 0 will be treated as data byte until the next address byte is received.\nNote:\n- This register is applicable only when \'ADDR_MATCH\'(LCR_EXT\\[1\\] and \'DLS_E\' (LCR_EXT\\[0\\]) bits are set to 1.\nRAR should be programmed only when UART is not busy."]
    #[inline(always)]
    pub fn uart_rar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3RarReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3RarReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Receive Buffer Register"]
pub type Uart3RbrThrDllReg = crate::RegValueT<Uart3RbrThrDllReg_SPEC>;

impl Uart3RbrThrDllReg {
    #[doc = "When 9BIT_DATA_EN, On read :Receive Buffer bit 8 - On write Transmit Buffer bit 8 when LCR_EXT\\[3\\]=1"]
    #[inline(always)]
    pub fn rbr_thr_9bit(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Uart3RbrThrDllReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Uart3RbrThrDllReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Buffer Register: (RBR).\nThis register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur.\nTransmit Holding Register: (THR)\nThis register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost.\nDivisor Latch (Low): (DLL)\nThis register makes up the lower 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows:\nbaud rate = (serial clock freq) / (16 * divisor)\nNote that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications will occur. Also, once the DLL is set, at least 8 clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data.\nDivisor Latch (High): (DLH) (Note: This register is placed in UART_IER_DLH_REG with offset 0x4)\nUpper 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock frequency divided by sixteen times the value of the baud rate divisor, as follows:\nbaud rate = (serial clock freq) / (16 * divisor).\nNote that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications occur. Also, once the DLH is set, at least 8 clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data."]
    #[inline(always)]
    pub fn rbr_thr_dll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3RbrThrDllReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3RbrThrDllReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Receive FIFO Level."]
pub type Uart3RflReg = crate::RegValueT<Uart3RflReg_SPEC>;

impl Uart3RflReg {
    #[doc = "Receive FIFO Level.\nThis is indicates the number of data entries in the receive FIFO."]
    #[inline(always)]
    pub fn uart_receive_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Uart3RflReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Uart3RflReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Shadow Break Control Register"]
pub type Uart3SbcrReg = crate::RegValueT<Uart3SbcrReg_SPEC>;

impl Uart3SbcrReg {
    #[doc = "Shadow Break Control Bit.\nThis is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device.\nIf set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared."]
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
#[doc = "Shadow DMA Mode"]
pub type Uart3SdmamReg = crate::RegValueT<Uart3SdmamReg_SPEC>;

impl Uart3SdmamReg {
    #[doc = "Shadow DMA Mode.\nThis is a shadow register for the DMA mode bit (FCR\\[3\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the DMA Mode bit gets updated. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals.\n0 = mode 0\n1 = mode 1"]
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
#[doc = "Shadow FIFO Enable"]
pub type Uart3SfeReg = crate::RegValueT<Uart3SfeReg_SPEC>;

impl Uart3SfeReg {
    #[doc = "Shadow FIFO Enable.\nThis is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr0Reg = crate::RegValueT<Uart3SrbrSthr0Reg_SPEC>;

impl Uart3SrbrSthr0Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr0Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr10Reg = crate::RegValueT<Uart3SrbrSthr10Reg_SPEC>;

impl Uart3SrbrSthr10Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr10Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr11Reg = crate::RegValueT<Uart3SrbrSthr11Reg_SPEC>;

impl Uart3SrbrSthr11Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr11Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr12Reg = crate::RegValueT<Uart3SrbrSthr12Reg_SPEC>;

impl Uart3SrbrSthr12Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr12Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr12Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr13Reg = crate::RegValueT<Uart3SrbrSthr13Reg_SPEC>;

impl Uart3SrbrSthr13Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr13Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr13Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr14Reg = crate::RegValueT<Uart3SrbrSthr14Reg_SPEC>;

impl Uart3SrbrSthr14Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr14Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr15Reg = crate::RegValueT<Uart3SrbrSthr15Reg_SPEC>;

impl Uart3SrbrSthr15Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr15Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr15Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr1Reg = crate::RegValueT<Uart3SrbrSthr1Reg_SPEC>;

impl Uart3SrbrSthr1Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr2Reg = crate::RegValueT<Uart3SrbrSthr2Reg_SPEC>;

impl Uart3SrbrSthr2Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr2Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr3Reg = crate::RegValueT<Uart3SrbrSthr3Reg_SPEC>;

impl Uart3SrbrSthr3Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr3Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr4Reg = crate::RegValueT<Uart3SrbrSthr4Reg_SPEC>;

impl Uart3SrbrSthr4Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr4Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr5Reg = crate::RegValueT<Uart3SrbrSthr5Reg_SPEC>;

impl Uart3SrbrSthr5Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr5Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr6Reg = crate::RegValueT<Uart3SrbrSthr6Reg_SPEC>;

impl Uart3SrbrSthr6Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr6Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr7Reg = crate::RegValueT<Uart3SrbrSthr7Reg_SPEC>;

impl Uart3SrbrSthr7Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr7Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr8Reg = crate::RegValueT<Uart3SrbrSthr8Reg_SPEC>;

impl Uart3SrbrSthr8Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr8Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart3SrbrSthr9Reg = crate::RegValueT<Uart3SrbrSthr9Reg_SPEC>;

impl Uart3SrbrSthr9Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3SrbrSthr9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3SrbrSthr9Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Software Reset Register."]
pub type Uart3SrrReg = crate::RegValueT<Uart3SrrReg_SPEC>;

impl Uart3SrrReg {
    #[doc = "XMIT FIFO Reset.\nThis is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is \'self-clearing\'. It is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_xfr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RCVR FIFO Reset.\nThis is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty.\nNote that this bit is \'self-clearing\'. It is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_rfr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "UART Reset. This asynchronously resets the UART Ctrl and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
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
#[doc = "Shadow Request to Send"]
pub type Uart3SrtsReg = crate::RegValueT<Uart3SrtsReg_SPEC>;

impl Uart3SrtsReg {
    #[doc = "Shadow Request to Send.\nThis is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to\nperforming a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART Ctrl is ready to exchange data.\nWhen Auto RTS Flow Control is not enabled (MCR\\[5\\] = 0), the rts_n signal is set low by programming MCR\\[1\\] (RTS) to a high.\nIn Auto Flow Control, (active MCR\\[5\\] = 1) and FIFOs enable (FCR\\[0\\] = 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold).\nNote that in Loopback mode (MCR\\[4\\] = 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
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
#[doc = "Shadow RCVR Trigger"]
pub type Uart3SrtReg = crate::RegValueT<Uart3SrtReg_SPEC>;

impl Uart3SrtReg {
    #[doc = "Shadow RCVR Trigger.\nThis is a shadow register for the RCVR trigger bits (FCR\\[7:6\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the RCVR trigger bit gets updated.\nThis is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. It also determines when the dma_rx_req_n signal is asserted when DMA Mode (FCR\\[3\\]) = 1. The following trigger levels are supported:\n00 = 1 character in the FIFO\n01 = FIFO ¼ full\n10 = FIFO ½ full\n11 = FIFO 2 less than full"]
    #[inline(always)]
    pub fn uart_shadow_rcvr_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Uart3SrtReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Uart3SrtReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Shadow TX Empty Trigger"]
pub type Uart3StetReg = crate::RegValueT<Uart3StetReg_SPEC>;

impl Uart3StetReg {
    #[doc = "Shadow TX Empty Trigger.\nThis is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated.\nThis is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported:\n00 = FIFO empty\n01 = 2 characters in the FIFO\n10 = FIFO ¼ full\n11 = FIFO ½ full"]
    #[inline(always)]
    pub fn uart_shadow_tx_empty_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Uart3StetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Uart3StetReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit Address Register"]
pub type Uart3TarReg = crate::RegValueT<Uart3TarReg_SPEC>;

impl Uart3TarReg {
    #[doc = "This is an address matching register during transmit mode. If DLS_E (LCR_EXT\\[0\\]) bit is enabled, then uart will send the 9-bit character with 9-th bit set to 1 and remaining 8-bit address will be sent from this register provided \'SEND_ADDR\' (LCR_EXT\\[2\\]) bit is set to 1.\nNote:\n- This register is used only to send the address. The normal data should be sent by programming THR register.\n- Once the address is started to send on the DW_apb_uart serial lane, then \'SEND_ADDR\' bit will be auto-cleared by the hardware."]
    #[inline(always)]
    pub fn uart_tar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart3TarReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart3TarReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit FIFO Level"]
pub type Uart3TflReg = crate::RegValueT<Uart3TflReg_SPEC>;

impl Uart3TflReg {
    #[doc = "Transmit FIFO Level.\nThis is indicates the number of data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn uart_transmit_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Uart3TflReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Uart3TflReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "ISO7816 Timer Register"]
pub type Uart3TimerReg = crate::RegValueT<Uart3TimerReg_SPEC>;

impl Uart3TimerReg {
    #[doc = "0 : Timer will count up to max value then stops. Timer has to be disabled and enabled again to restart. Timer is clocked with the ISO7816 clock\n1 : Timer will count guard time. ISO7816_TIM_MAX has to be 16*GuardTime-1"]
    #[inline(always)]
    pub fn iso7816_tim_mode(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Uart3TimerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Uart3TimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 : Timer is disabled\n1 : Timer is enabled"]
    #[inline(always)]
    pub fn iso7816_tim_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Uart3TimerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Uart3TimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "On write : timer will count from 0 to ISO7816_TIM_MAX\nOn read : gives the current timer value"]
    #[inline(always)]
    pub fn iso7816_tim_max(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Uart3TimerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Uart3TimerReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Component Version"]
pub type Uart3UcvReg = crate::RegValueT<Uart3UcvReg_SPEC>;

impl Uart3UcvReg {
    #[doc = "Component Version"]
    #[inline(always)]
    pub fn uart_ucv(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Uart3UcvReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Uart3UcvReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "UART Status register."]
pub type Uart3UsrReg = crate::RegValueT<Uart3UsrReg_SPEC>;

impl Uart3UsrReg {
    #[doc = "Receive FIFO Full.\nThis is used to indicate that the receive FIFO is completely full.\n0 = Receive FIFO not full\n1 = Receive FIFO Full\nThis bit is cleared when the RX FIFO is no longer full."]
    #[inline(always)]
    pub fn uart_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Not Empty.\nThis is used to indicate that the receive FIFO contains one or more entries.\n0 = Receive FIFO is empty\n1 = Receive FIFO is not empty\nThis bit is cleared when the RX FIFO is empty."]
    #[inline(always)]
    pub fn uart_rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Empty.\nThis is used to indicate that the transmit FIFO is completely empty.\n0 = Transmit FIFO is not empty\n1 = Transmit FIFO is empty\nThis bit is cleared when the TX FIFO is no longer empty."]
    #[inline(always)]
    pub fn uart_tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Not Full.\nThis is used to indicate that the transmit FIFO in not full.\n0 = Transmit FIFO is full\n1 = Transmit FIFO is not full\nThis bit is cleared when the TX FIFO is full."]
    #[inline(always)]
    pub fn uart_tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart3UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart3UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UART Busy. This indicates that a serial transfer is in progress, when cleared indicates that the DW_apb_uart is idle or inactive. 0 - DW_apb_uart is idle or inactive 1 - DW_apb_uart is busy (actively transferring data) Note that it is possible for the UART Busy bit to be cleared even though a new character may have been sent from another device. That is, if the DW_apb_uart has no data in the THR and RBR and there is no transmission in progress and a start bit of a new character has just reached the DW_apb_uart. This is due to the fact that a valid start is not seen until the middle of the bit period and this duration is dependent on the baud divisor that has been programmed. If a second system clock has been implemented (CLOCK_MODE == Enabled) the assertion of this bit will also be delayed by several cycles of the slower clock."]
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
