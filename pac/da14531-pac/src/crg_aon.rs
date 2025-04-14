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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:19 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRG_AON registers"]
unsafe impl ::core::marker::Send for super::CrgAon {}
unsafe impl ::core::marker::Sync for super::CrgAon {}
impl super::CrgAon {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gp_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hibern_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::HibernCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::HibernCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::HwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::HwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn power_aon_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PowerAonCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PowerAonCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ram_lpmx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RamLpmxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RamLpmxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn test_vdd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TestVddReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TestVddReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpDataReg_SPEC;
impl crate::sealed::RegSpec for GpDataReg_SPEC {
    type DataType = u16;
}

pub type GpDataReg = crate::RegValueT<GpDataReg_SPEC>;

impl GpDataReg {
    #[inline(always)]
    pub fn ana_spare(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, GpDataReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,GpDataReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disable_clamp_overrule(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, GpDataReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,GpDataReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_gp_data(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, GpDataReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,GpDataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpDataReg {
    #[inline(always)]
    fn default() -> GpDataReg {
        <crate::RegValueT<GpDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HibernCtrlReg_SPEC;
impl crate::sealed::RegSpec for HibernCtrlReg_SPEC {
    type DataType = u16;
}

pub type HibernCtrlReg = crate::RegValueT<HibernCtrlReg_SPEC>;

impl HibernCtrlReg {
    #[inline(always)]
    pub fn hibern_wkup_mask(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, HibernCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,HibernCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hibern_wkup_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HibernCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,HibernCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hibernation_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HibernCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,HibernCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for HibernCtrlReg {
    #[inline(always)]
    fn default() -> HibernCtrlReg {
        <crate::RegValueT<HibernCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for HwrCtrlReg_SPEC {
    type DataType = u16;
}

pub type HwrCtrlReg = crate::RegValueT<HwrCtrlReg_SPEC>;

impl HwrCtrlReg {
    #[inline(always)]
    pub fn disable_hwr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,HwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for HwrCtrlReg {
    #[inline(always)]
    fn default() -> HwrCtrlReg {
        <crate::RegValueT<HwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadLatchReg_SPEC;
impl crate::sealed::RegSpec for PadLatchReg_SPEC {
    type DataType = u16;
}

pub type PadLatchReg = crate::RegValueT<PadLatchReg_SPEC>;

impl PadLatchReg {
    #[inline(always)]
    pub fn pad_latch_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PadLatchReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PadLatchReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PadLatchReg {
    #[inline(always)]
    fn default() -> PadLatchReg {
        <crate::RegValueT<PadLatchReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowerAonCtrlReg_SPEC;
impl crate::sealed::RegSpec for PowerAonCtrlReg_SPEC {
    type DataType = u16;
}

pub type PowerAonCtrlReg = crate::RegValueT<PowerAonCtrlReg_SPEC>;

impl PowerAonCtrlReg {
    #[inline(always)]
    pub fn force_running_comp_dis(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PowerAonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_ret_trim(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, PowerAonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmp_vcont_slp_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PowerAonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_mode_force(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, PowerAonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charge_vbat_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PowerAonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32k_low_speed_force(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PowerAonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32k_high_speed_force(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PowerAonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_high_rst_mask(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PowerAonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_low_rst_mask(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PowerAonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbat_hl_connect_res_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, PowerAonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,PowerAonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PowerAonCtrlReg {
    #[inline(always)]
    fn default() -> PowerAonCtrlReg {
        <crate::RegValueT<PowerAonCtrlReg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamLpmxReg_SPEC;
impl crate::sealed::RegSpec for RamLpmxReg_SPEC {
    type DataType = u16;
}

pub type RamLpmxReg = crate::RegValueT<RamLpmxReg_SPEC>;

impl RamLpmxReg {
    #[inline(always)]
    pub fn ramx_lpmx(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RamLpmxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RamLpmxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RamLpmxReg {
    #[inline(always)]
    fn default() -> RamLpmxReg {
        <crate::RegValueT<RamLpmxReg_SPEC> as RegisterValue<_>>::new(7)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetStatReg_SPEC;
impl crate::sealed::RegSpec for ResetStatReg_SPEC {
    type DataType = u16;
}

pub type ResetStatReg = crate::RegValueT<ResetStatReg_SPEC>;

impl ResetStatReg {
    #[inline(always)]
    pub fn wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn poreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetStatReg {
    #[inline(always)]
    fn default() -> ResetStatReg {
        <crate::RegValueT<ResetStatReg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TestVddReg_SPEC;
impl crate::sealed::RegSpec for TestVddReg_SPEC {
    type DataType = u16;
}

pub type TestVddReg = crate::RegValueT<TestVddReg_SPEC>;

impl TestVddReg {
    #[inline(always)]
    pub fn ldos_disable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TestVddReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,TestVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn test_vdd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TestVddReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TestVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TestVddReg {
    #[inline(always)]
    fn default() -> TestVddReg {
        <crate::RegValueT<TestVddReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
