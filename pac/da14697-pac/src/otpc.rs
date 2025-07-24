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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:38 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"OTPC registers"]
unsafe impl ::core::marker::Send for super::Otpc {}
unsafe impl ::core::marker::Sync for super::Otpc {}
impl super::Otpc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Mode register"]
    #[inline(always)]
    pub const fn otpc_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "The address of the word that will be programmed, when the PROG mode is used."]
    #[inline(always)]
    pub const fn otpc_paddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "The 32-bit word that will be programmed, when the PROG mode is used."]
    #[inline(always)]
    pub const fn otpc_pword_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPwordReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPwordReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Status register"]
    #[inline(always)]
    pub const fn otpc_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Various timing parameters of the OTP cell."]
    #[inline(always)]
    pub const fn otpc_tim1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcTim1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcTim1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Various timing parameters of the OTP cell."]
    #[inline(always)]
    pub const fn otpc_tim2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcTim2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcTim2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcModeReg_SPEC;
impl crate::sealed::RegSpec for OtpcModeReg_SPEC {
    type DataType = u32;
}

#[doc = "Mode register"]
pub type OtpcModeReg = crate::RegValueT<OtpcModeReg_SPEC>;

impl OtpcModeReg {
    #[doc = "Defines the part of the OTP cell that is programmed by the controller during the PROG mode, for each program request that is applied.\n0x0 : Both normal and redundancy arrays are programmed. This is the normal way of programming.\n0x1 : Only the normal array is programmed.\n0x2 : Only the redundancy array is programmed.\n0x3 : Reserved\nThe value of this configuration field can be modified only when the controller is in an inactive mode (PDOWN, DSTBY, STBY). The setting will take effect when will be enabled again the PROG mode."]
    #[inline(always)]
    pub fn otpc_mode_prg_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, OtpcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the temperature condition under which is performed a margin read. It affects only the initial margin read (RINI mode) and the programming verification margin read (PVFY).\n0 : Regular temperature condition (less than 85°C)\n1 : High temperature condition (85°C or more)\nThe value of this configuration field can be modified only when the controller is in an inactive mode (PDOWN, DSTBY, STBY). The selection will take effect at the next PVFY or RINI mode that will be enabled. The READ mode is not affected by the setting of this configuration bit."]
    #[inline(always)]
    pub fn otpc_mode_ht_marg_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the memory area of the OTP cell that will be used.\n0 - Uses the main memory area of the OTP cell\n1 - Uses the test row of the OTP cell\nThe value of this configuration field can be modified only when the controller is in an inactive mode (PDOWN, DSTBY, STBY). The selection will take effect at the next programming or reading mode that will be enabled."]
    #[inline(always)]
    pub fn otpc_mode_use_tst_row(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the mode of operation of the OTPC controller. The encoding of the modes is as follows:\n0x0: PDOWN. The power supply of the OTP memory is OFF.\n0x1: DSTBY. The OTP memory is in deep standby mode (power supply ON and internal LDO OFF).\n0x2: STBY. The OTP memory is powered (power supply ON and internal LDO ON, but is not selected.\n0x3: READ. The OTP memory is in the normal read mode.\n0x4: PROG. The OTP memory is in programming mode.\n0x5: PVFY. The OTP memory is in programming verification mode (margin read after programming).\n0x6: RINI. The OTP memory is in initial read mode (initial margin read).\n0x7: Reserved.\n\nWhenever the OTPC_MODE_REG\\[MODE\\] is changing, the status bit OTPC_STAT_REG\\[OTPC_STAT_MRDY\\] gets the value zero. The new mode will be ready for use when the OTPC_STAT_MRDY become again 1. During the mode transition the OTPC_MODE_REG\\[MODE\\] become read only. Do not try to use or change any function of the controller until the OTPC_STAT_MRDY bit to become equal to 1."]
    #[inline(always)]
    pub fn otpc_mode_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, OtpcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcModeReg {
    #[inline(always)]
    fn default() -> OtpcModeReg {
        <crate::RegValueT<OtpcModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPaddrReg_SPEC;
impl crate::sealed::RegSpec for OtpcPaddrReg_SPEC {
    type DataType = u32;
}

#[doc = "The address of the word that will be programmed, when the PROG mode is used."]
pub type OtpcPaddrReg = crate::RegValueT<OtpcPaddrReg_SPEC>;

impl OtpcPaddrReg {
    #[doc = "The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory.\nThe OTPC_PADDR_REG refers to a word address. The OTPC_PADDR_REG has to be writen after the OTP_PWORD_REG and only if the OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0). A writting to the OTPC_PADDR_REG triggers the controller to start the programming procedure (only if the PROG mode is active)."]
    #[inline(always)]
    pub fn otpc_paddr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OtpcPaddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OtpcPaddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcPaddrReg {
    #[inline(always)]
    fn default() -> OtpcPaddrReg {
        <crate::RegValueT<OtpcPaddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPwordReg_SPEC;
impl crate::sealed::RegSpec for OtpcPwordReg_SPEC {
    type DataType = u32;
}

#[doc = "The 32-bit word that will be programmed, when the PROG mode is used."]
pub type OtpcPwordReg = crate::RegValueT<OtpcPwordReg_SPEC>;

impl OtpcPwordReg {
    #[doc = "The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP memory, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory.\nThe OTP_PWORD_REG must be written before the OTPC_PADDR_REG and only if OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\] = 1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0)."]
    #[inline(always)]
    pub fn otpc_pword(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcPwordReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcPwordReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcPwordReg {
    #[inline(always)]
    fn default() -> OtpcPwordReg {
        <crate::RegValueT<OtpcPwordReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcStatReg_SPEC;
impl crate::sealed::RegSpec for OtpcStatReg_SPEC {
    type DataType = u32;
}

#[doc = "Status register"]
pub type OtpcStatReg = crate::RegValueT<OtpcStatReg_SPEC>;

impl OtpcStatReg {
    #[doc = "Indicates the progress of the transition from a mode of operation to a new mode of operation.\n0 : There is a transition in progress in a new mode of operation . Wait until the transition to be completed.\n1 : The transition to the new mode of operation has been completed. The function that has been enabled by the new mode can be used. A new mode can be applied.\nThis status bit gets the value zero every time where the OTPC_MODE_REG\\[MODE\\] is changing. Do not try to use or change any function of the controller until this status bit to becomes equal to 1."]
    #[inline(always)]
    pub fn otpc_stat_mrdy(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates the status of the programming buffer (PBUF).\n0 : The PBUF contains the address and the data of a programming request. The OTPC_PADDR_REG and the OTPC_PWORD_REG should not be written as long as this status bit is zero.\n1 : The PBUF is empty and a new programming request can be registered in the PBUF by using the OTPC_PADDR_REG and the OTPC_PWORD_REG registers.\nThis status bit gets the value zero every time where a progrmaming is triggered by the OTPC_PADDR_REG (only if the PROG mode is active)."]
    #[inline(always)]
    pub fn otpc_stat_pbuf_empty(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates the state of the programming process.\n0: The controller is busy. A programming is in progress.\n1: The logic which performs programming is idle."]
    #[inline(always)]
    pub fn otpc_stat_prdy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcStatReg {
    #[inline(always)]
    fn default() -> OtpcStatReg {
        <crate::RegValueT<OtpcStatReg_SPEC> as RegisterValue<_>>::new(7)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcTim1Reg_SPEC;
impl crate::sealed::RegSpec for OtpcTim1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Various timing parameters of the OTP cell."]
pub type OtpcTim1Reg = crate::RegValueT<OtpcTim1Reg_SPEC>;

impl OtpcTim1Reg {
    #[doc = "The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for programming. It must be :\n- at least 10us\n- no more than 100us"]
    #[inline(always)]
    pub fn otpc_tim1_us_t_csp(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7f,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for any kind of read. It must be at least 10us."]
    #[inline(always)]
    pub fn otpc_tim1_us_t_cs(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) that are required until to be enabled the LDO of the OTP. It must be at least 10us."]
    #[inline(always)]
    pub fn otpc_tim1_us_t_pl(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval at least higher than 60ns. This timing parameter refers to the access time of the OTP memory."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_rd(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval that is at least higher than 20 ns."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_20ns(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval equal to 1us. This setting affects all the timing parameters that refer to microseconds, due to that defines the correspondence of a microsecond to a number of hclk_c clock cycles."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_1us(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcTim1Reg {
    #[inline(always)]
    fn default() -> OtpcTim1Reg {
        <crate::RegValueT<OtpcTim1Reg_SPEC> as RegisterValue<_>>::new(161026079)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcTim2Reg_SPEC;
impl crate::sealed::RegSpec for OtpcTim2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Various timing parameters of the OTP cell."]
pub type OtpcTim2Reg = crate::RegValueT<OtpcTim2Reg_SPEC>;

impl OtpcTim2Reg {
    #[doc = "Adds an additional hclk_c clock cycle at all the time intervals that count in microseconds.\n0 : The extra hclk_c clock cycle is not applied\n1 : The extra hclk_c clock cycle is applied"]
    #[inline(always)]
    pub fn otpc_tim2_us_add_cc_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, OtpcTim2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) that are required after the exit from the deep sleep standby mode and before to become ready to enter in an active mode (reading or programming). It must be at least 2us."]
    #[inline(always)]
    pub fn otpc_tim2_us_t_sas(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x3,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) that are required after the last programming pulse and before to be disabled the programming mode in the OTP memory. It must be:\n- at least 5us\n- no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pph(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) that are required after the enabling of the power supply of the OTP memory and before to become ready for the enabling of the internal LDO. It must be at least 1us."]
    #[inline(always)]
    pub fn otpc_tim2_us_t_vds(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x7,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) that are required after the enabling of the programming in the OTP memory and before to be applied the first programming pulse. It must be :\n- at least 5us\n- no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pps(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) for recovery after a programming sequence. It must be :\n- at least 5us\n- no more than 100us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_ppr(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) between two consecutive programming pulses. It must be :\n- at least 1us\n- no more than 5us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pwi(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of microseconds (minus one) that lasts the programming of each bit. It must be :\n- at least 10us\n- no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pw(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcTim2Reg {
    #[inline(always)]
    fn default() -> OtpcTim2Reg {
        <crate::RegValueT<OtpcTim2Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}
