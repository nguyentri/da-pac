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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:52 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DCDC_BOOST registers"]
unsafe impl ::core::marker::Send for super::DcdcBoost {}
unsafe impl ::core::marker::Sync for super::DcdcBoost {}
impl super::DcdcBoost {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn boost_ctrl_reg0(
        &self,
    ) -> &'static crate::common::Reg<self::BoostCtrlReg0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostCtrlReg0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn boost_ctrl_reg1(
        &self,
    ) -> &'static crate::common::Reg<self::BoostCtrlReg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostCtrlReg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn boost_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BoostStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn boost_test_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BoostTestCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostTestCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vled_pwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VledPwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VledPwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vled_pwr_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VledPwrStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VledPwrStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostCtrlReg0_SPEC;
impl crate::sealed::RegSpec for BoostCtrlReg0_SPEC {
    type DataType = u32;
}

pub type BoostCtrlReg0 = crate::RegValueT<BoostCtrlReg0_SPEC>;

impl BoostCtrlReg0 {
    #[doc = "Delay before generating next comparator clock after a timeout event on the P switch, allows inductor current to drop to zero\n0x0: Disabled\n0x1: 250 ns\n0x2: 500 ns\n0x3: 750 ns\n0x4: 1000 ns (default)\n0x5: 1250 ns\n0x6: 1500 ns\n0x7: 1750 ns\n0x8: 2000 ns\n0x9: 2250 ns\n0xA: 2500 ns\n0xB: 2750 ns\n0xC: 3000 ns\n0xD: 3250 ns\n0xE: 3500 ns\n0xF: 3750 ns"]
    #[inline(always)]
    pub fn boost_timeout_trig_delay(
        self,
    ) -> crate::common::RegisterField<17, 0xf, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0xf,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "P switch timeout, if switch is closed longer than this a timeout is generated and the FSM is forced to the next state\n0x0: Disabled\n0x1: 250 ns\n0x2: 500 ns\n0x3: 750 ns\n0x4: 1000 ns\n0x5: 1250 ns\n0x6: 1500 ns\n0x7: 1750 ns\n0x8: 2000 ns (default)\n0x9: 2250 ns\n0xA: 2500 ns\n0xB: 2750 ns\n0xC: 3000 ns\n0xD: 3250 ns\n0xE: 3500 ns\n0xF: 3750 ns"]
    #[inline(always)]
    pub fn boost_psw_timeout(
        self,
    ) -> crate::common::RegisterField<13, 0xf, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0xf,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "N switch timeout, if switch is closed longer than this a timeout is generated and the FSM is forced to the next state\n0x0: Disabled\n0x1: 125 ns\n0x2: 250 ns\n0x3: 375 ns\n0x4: 500 ns\n0x5: 625 ns\n0x6: 750 ns (default)\n0x7: 875 ns\n0x8: 1000 ns\n0x9: 1125 ns\n0xA: 1250 ns\n0xB: 1375 ns\n0xC: 1500 ns\n0xD: 1625 ns\n0xE: 1750 ns\n0xF: 1875 ms"]
    #[inline(always)]
    pub fn boost_nsw_timeout(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xf,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Number of subsequent V_NOK events before BOOST_VLED_OK is reset\n0x0: 2\n0x1: 4\n0x2: 8 (default)\n0x3: 15"]
    #[inline(always)]
    pub fn boost_ok_clr_count(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines times between comparator samples when converter is idle\n0x0 = 250 ns\n0x1 = 500 ns (default)\n0x2 = 1000 ns\n0x3 = 2000 ns"]
    #[inline(always)]
    pub fn boost_idle_clk_div(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim setting for boost converter, sets deviation from nominal output voltage (4 V)\n0x0: -75 mV\n0x1: -50 mV\n0x2: -25 mV\n0x3: 0 mV (default)\n0x4: 25 mV\n0x5: 50 mV\n0x6: 75 mV\n0x7: 100 mV"]
    #[inline(always)]
    pub fn boost_vled_trim(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage selection for boost converter, sets nominal output voltage\n0x0: 4.50V (default)\n0x1: 4.75V\n0x2: 5.00V\n0x3: 5.00V"]
    #[inline(always)]
    pub fn boost_vled_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BoostCtrlReg0 {
    #[inline(always)]
    fn default() -> BoostCtrlReg0 {
        <crate::RegValueT<BoostCtrlReg0_SPEC> as RegisterValue<_>>::new(593196)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostCtrlReg1_SPEC;
impl crate::sealed::RegSpec for BoostCtrlReg1_SPEC {
    type DataType = u32;
}

pub type BoostCtrlReg1 = crate::RegValueT<BoostCtrlReg1_SPEC>;

impl BoostCtrlReg1 {
    #[doc = "Enable fixed current iso dynamic current in sleep mode\n0x0: Use dynamic current control\n0x1: Use fixed current as defined in BOOST_CUR_LIM_SLEEP (default)"]
    #[inline(always)]
    pub fn boost_cur_lim_sleep_fixed(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BoostCtrlReg1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Fixed inductor peak current limit in sleep mode\nI = 30 mA * (1 + N), default 960 mA"]
    #[inline(always)]
    pub fn boost_cur_lim_sleep(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Step size taken by automatic inductor peak current limit control\n0x0: 0 (disabled)\n0x1: 1\n0x2: 2 (default)\n0x3: 3"]
    #[inline(always)]
    pub fn boost_cur_lim_step(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maximum inductor peak current limit\nI = 30 mA * (1 + N), default 960 mA"]
    #[inline(always)]
    pub fn boost_cur_lim_max(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Minimum inductor peak current limit\nI = 30 mA * (1 + N), default 150 mA"]
    #[inline(always)]
    pub fn boost_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BoostCtrlReg1 {
    #[inline(always)]
    fn default() -> BoostCtrlReg1 {
        <crate::RegValueT<BoostCtrlReg1_SPEC> as RegisterValue<_>>::new(261092)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostStatusReg_SPEC;
impl crate::sealed::RegSpec for BoostStatusReg_SPEC {
    type DataType = u32;
}

pub type BoostStatusReg = crate::RegValueT<BoostStatusReg_SPEC>;

impl BoostStatusReg {
    #[doc = "Actual P side comparator trim value"]
    #[inline(always)]
    pub fn boost_comp_trim(
        self,
    ) -> crate::common::RegisterField<18, 0x3f, 1, 0, u8, u8, BoostStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x3f,1,0,u8,u8,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Converter idle"]
    #[inline(always)]
    pub fn boost_idle(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Actual inductor peak current limit"]
    #[inline(always)]
    pub fn boost_cur_lim(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, BoostStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "P output of P side dynamic comparator"]
    #[inline(always)]
    pub fn boost_comp_p_dyn_p(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "N output of P side dynamic comparator"]
    #[inline(always)]
    pub fn boost_comp_p_dyn_n(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Output of P side continuous time comparator"]
    #[inline(always)]
    pub fn boost_comp_p_cont(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Output of N side continuous time comparator"]
    #[inline(always)]
    pub fn boost_comp_n_cont(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Timeout on P switch occurred"]
    #[inline(always)]
    pub fn boost_timeout_psw(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Timeout on N switch occurred"]
    #[inline(always)]
    pub fn boost_timeout_nsw(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "NOK output of output voltage comparator"]
    #[inline(always)]
    pub fn boost_vout_nok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "OK output of output voltage comparator"]
    #[inline(always)]
    pub fn boost_vout_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "State of boost converter switches\n0x0: Both off\n0x1: P switch on\n0x2: N switch on\n0x3: Undefined"]
    #[inline(always)]
    pub fn boost_sw_state(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, BoostStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates if the converter is enabled and the startup counter has expired (internal biasing settled)"]
    #[inline(always)]
    pub fn boost_startup_complete(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that V_LED is above its threshold, reset after too many subsequent V_NOK events"]
    #[inline(always)]
    pub fn boost_vled_ok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BoostStatusReg {
    #[inline(always)]
    fn default() -> BoostStatusReg {
        <crate::RegValueT<BoostStatusReg_SPEC> as RegisterValue<_>>::new(147456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostTestCtrlReg_SPEC;
impl crate::sealed::RegSpec for BoostTestCtrlReg_SPEC {
    type DataType = u32;
}

pub type BoostTestCtrlReg = crate::RegValueT<BoostTestCtrlReg_SPEC>;

impl BoostTestCtrlReg {
    #[doc = "Test mode control for 20mA test-mux\n0x0: All switches open (Default)\n0x1: Sink 20mA from LX for testing PMOS\n0x2: Sink 20mA from V_LED\n0x3: Source 20mA into LX for testing NMOS"]
    #[inline(always)]
    pub fn boost_iload_sel_test(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, u8, BoostTestCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            27,
            0x3,
            1,
            0,
            u8,
            u8,
            BoostTestCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trim low side supply voltage\nV = 2 V + 300 mV * N, default 2.9 V"]
    #[inline(always)]
    pub fn boost_lssup_trim(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, BoostTestCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            u8,
            u8,
            BoostTestCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trim high side ground\nV = VBAT - (2 V + 400 mV * N), default VBAT - 3.2 V"]
    #[inline(always)]
    pub fn boost_hsgnd_trim(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, u8, BoostTestCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            u8,
            u8,
            BoostTestCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BoostTestCtrlReg {
    #[inline(always)]
    fn default() -> BoostTestCtrlReg {
        <crate::RegValueT<BoostTestCtrlReg_SPEC> as RegisterValue<_>>::new(15729152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VledPwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for VledPwrCtrlReg_SPEC {
    type DataType = u32;
}

pub type VledPwrCtrlReg = crate::RegValueT<VledPwrCtrlReg_SPEC>;

impl VledPwrCtrlReg {
    #[doc = "Manual selection of VLED supply source, requires that VLED_PWR_MANUAL = 0x1\n0x0: VLED not powered\n0x1: VLED powered by VSYS\n0x2: VLED powered by boost converter\n0x3: N.A."]
    #[inline(always)]
    pub fn vled_pwr_force(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, VledPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0x0: VLED supply source automatically selected\n0x1: VLED supply source manually selected"]
    #[inline(always)]
    pub fn vled_pwr_manual(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, VledPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the condition for powering VLED from VSYS\n0x0: VLED always powered from VSYS\n0x1: VLED powered form VSYS if VSYS is near VLED, depending on vsys-comparator (Default)"]
    #[inline(always)]
    pub fn vled_pwr_use_vsys_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, VledPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0x0: VLED power controller disabled\n0x1: VLED power controller enabled"]
    #[inline(always)]
    pub fn vled_pwr_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, VledPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets debounce time on VSYS comparator in steps of 1.024 ms\nNote: actual delay can be up to one period of 1.024 ms clock shorter than programmed depending on alignment of comparator trip event and clock edge"]
    #[inline(always)]
    pub fn vsys_ok_debounce(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, VledPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VledPwrCtrlReg {
    #[inline(always)]
    fn default() -> VledPwrCtrlReg {
        <crate::RegValueT<VledPwrCtrlReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VledPwrStatusReg_SPEC;
impl crate::sealed::RegSpec for VledPwrStatusReg_SPEC {
    type DataType = u32;
}

pub type VledPwrStatusReg = crate::RegValueT<VledPwrStatusReg_SPEC>;

impl VledPwrStatusReg {
    #[doc = "Indicates whether boost converter is blocked or not"]
    #[inline(always)]
    pub fn vled_pwr_allow_boost(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that VSYS switch is closed"]
    #[inline(always)]
    pub fn vled_pwr_vsys_connected(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "State of the VLED power control FSM\n0x0: Disabled\n0x1: VSYS\n0x2: Boost\n0x3: N.A."]
    #[inline(always)]
    pub fn vled_pwr_switch_ctrl_state(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, VledPwrStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Output of VSYS OK debounce logic"]
    #[inline(always)]
    pub fn vsys_ok_debounced(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Output of VSYS OK logic"]
    #[inline(always)]
    pub fn vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for VledPwrStatusReg {
    #[inline(always)]
    fn default() -> VledPwrStatusReg {
        <crate::RegValueT<VledPwrStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
