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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Cortex M0 NVIC registers"]
unsafe impl ::core::marker::Send for super::Nvic {}
unsafe impl ::core::marker::Sync for super::Nvic {}
impl super::Nvic {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn iser(&self) -> &'static crate::common::Reg<self::Iser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icer(&self) -> &'static crate::common::Reg<self::Icer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ispr(&self) -> &'static crate::common::Reg<self::Ispr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ispr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icpr(&self) -> &'static crate::common::Reg<self::Icpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipr0(&self) -> &'static crate::common::Reg<self::Ipr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipr1(&self) -> &'static crate::common::Reg<self::Ipr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipr2(&self) -> &'static crate::common::Reg<self::Ipr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(776usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipr3(&self) -> &'static crate::common::Reg<self::Ipr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(780usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipr4(&self) -> &'static crate::common::Reg<self::Ipr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipr5(&self) -> &'static crate::common::Reg<self::Ipr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(788usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipr6(&self) -> &'static crate::common::Reg<self::Ipr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(792usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iser_SPEC;
impl crate::sealed::RegSpec for Iser_SPEC {
    type DataType = u32;
}

pub type Iser = crate::RegValueT<Iser_SPEC>;

impl Iser {
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Iser {
    #[inline(always)]
    fn default() -> Iser {
        <crate::RegValueT<Iser_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icer_SPEC;
impl crate::sealed::RegSpec for Icer_SPEC {
    type DataType = u32;
}

pub type Icer = crate::RegValueT<Icer_SPEC>;

impl Icer {
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Icer {
    #[inline(always)]
    fn default() -> Icer {
        <crate::RegValueT<Icer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ispr_SPEC;
impl crate::sealed::RegSpec for Ispr_SPEC {
    type DataType = u32;
}

pub type Ispr = crate::RegValueT<Ispr_SPEC>;

impl Ispr {
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ispr {
    #[inline(always)]
    fn default() -> Ispr {
        <crate::RegValueT<Ispr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icpr_SPEC;
impl crate::sealed::RegSpec for Icpr_SPEC {
    type DataType = u32;
}

pub type Icpr = crate::RegValueT<Icpr_SPEC>;

impl Icpr {
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Icpr {
    #[inline(always)]
    fn default() -> Icpr {
        <crate::RegValueT<Icpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr0_SPEC;
impl crate::sealed::RegSpec for Ipr0_SPEC {
    type DataType = u32;
}

pub type Ipr0 = crate::RegValueT<Ipr0_SPEC>;

impl Ipr0 {
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_gen_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr0 {
    #[inline(always)]
    fn default() -> Ipr0 {
        <crate::RegValueT<Ipr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr1_SPEC;
impl crate::sealed::RegSpec for Ipr1_SPEC {
    type DataType = u32;
}

pub type Ipr1 = crate::RegValueT<Ipr1_SPEC>;

impl Ipr1 {
    #[inline(always)]
    pub fn i2c_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adc_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn keybrd_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr1 {
    #[inline(always)]
    fn default() -> Ipr1 {
        <crate::RegValueT<Ipr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr2_SPEC;
impl crate::sealed::RegSpec for Ipr2_SPEC {
    type DataType = u32;
}

pub type Ipr2 = crate::RegValueT<Ipr2_SPEC>;

impl Ipr2 {
    #[inline(always)]
    pub fn ble_rf_diag_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_cal_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpio0_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpio1_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr2 {
    #[inline(always)]
    fn default() -> Ipr2 {
        <crate::RegValueT<Ipr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr3_SPEC;
impl crate::sealed::RegSpec for Ipr3_SPEC {
    type DataType = u32;
}

pub type Ipr3 = crate::RegValueT<Ipr3_SPEC>;

impl Ipr3 {
    #[inline(always)]
    pub fn spi_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adc_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn keybrd_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr3 {
    #[inline(always)]
    fn default() -> Ipr3 {
        <crate::RegValueT<Ipr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr4_SPEC;
impl crate::sealed::RegSpec for Ipr4_SPEC {
    type DataType = u32;
}

pub type Ipr4 = crate::RegValueT<Ipr4_SPEC>;

impl Ipr4 {
    #[inline(always)]
    pub fn gpio2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpio3_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpio4_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swtim_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr4 {
    #[inline(always)]
    fn default() -> Ipr4 {
        <crate::RegValueT<Ipr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr5_SPEC;
impl crate::sealed::RegSpec for Ipr5_SPEC {
    type DataType = u32;
}

pub type Ipr5 = crate::RegValueT<Ipr5_SPEC>;

impl Ipr5 {
    #[inline(always)]
    pub fn wkup_quadec_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr5 {
    #[inline(always)]
    fn default() -> Ipr5 {
        <crate::RegValueT<Ipr5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr6_SPEC;
impl crate::sealed::RegSpec for Ipr6_SPEC {
    type DataType = u32;
}

pub type Ipr6 = crate::RegValueT<Ipr6_SPEC>;

impl Ipr6 {
    #[inline(always)]
    pub fn dma_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr6 {
    #[inline(always)]
    fn default() -> Ipr6 {
        <crate::RegValueT<Ipr6_SPEC> as RegisterValue<_>>::new(0)
    }
}
