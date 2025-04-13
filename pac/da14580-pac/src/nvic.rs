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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:24:23 +0000

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
    #[doc = "Interrupt set-enable register"]
    #[inline(always)]
    pub const fn iser(&self) -> &'static crate::common::Reg<self::Iser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt clear-enable register"]
    #[inline(always)]
    pub const fn icer(&self) -> &'static crate::common::Reg<self::Icer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Interrupt set-pending register"]
    #[inline(always)]
    pub const fn ispr(&self) -> &'static crate::common::Reg<self::Ispr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ispr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Interrupt clear-pending register"]
    #[inline(always)]
    pub const fn icpr(&self) -> &'static crate::common::Reg<self::Icpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "Interrupt priority register 0"]
    #[inline(always)]
    pub const fn ipr0(&self) -> &'static crate::common::Reg<self::Ipr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "Interrupt priority register 1"]
    #[inline(always)]
    pub const fn ipr1(&self) -> &'static crate::common::Reg<self::Ipr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "Interrupt priority register 2"]
    #[inline(always)]
    pub const fn ipr2(&self) -> &'static crate::common::Reg<self::Ipr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(776usize),
            )
        }
    }

    #[doc = "Interrupt priority register 3"]
    #[inline(always)]
    pub const fn ipr3(&self) -> &'static crate::common::Reg<self::Ipr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(780usize),
            )
        }
    }

    #[doc = "Interrupt priority register 4"]
    #[inline(always)]
    pub const fn ipr4(&self) -> &'static crate::common::Reg<self::Ipr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }

    #[doc = "Interrupt priority register 5"]
    #[inline(always)]
    pub const fn ipr5(&self) -> &'static crate::common::Reg<self::Ipr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(788usize),
            )
        }
    }

    #[doc = "Interrupt priority register 6"]
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
#[doc = "Interrupt set-enable register"]
pub type Iser = crate::RegValueT<Iser_SPEC>;

impl Iser {
    #[doc = "BLE_WAKEUP_LP_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE_GEN_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART2_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "I2C_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SPI_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ADC_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "KEYBRD_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE baseband or Radio diagnostic (Interrupt set-enable bit))"]
    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RFCAL_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO0 interrupt through debounce (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO1 interrupt through debounce (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO2 interrupt through debounce (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "IGPIO3 interrupt through debounce (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO4 interrupt through debounce (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software timer Interrupt (set-enable bit)"]
    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Combined Wake up Capture Timer, GPIO and QuadDecoder interrupt (set-enable bit)"]
    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PCM Interrupt (set-enable bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter input Interrupt (set-enable bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter output Interrupt (set-enable bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Interrupt (set-enable bit)"]
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
#[doc = "Interrupt clear-enable register"]
pub type Icer = crate::RegValueT<Icer_SPEC>;

impl Icer {
    #[doc = "BLE_WAKEUP_LP_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE_GEN_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART2_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "I2C_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SPI_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ADC_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "KEYBRD_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE baseband or Radio diagnostic (Interrupt clear-enable bit))"]
    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RFCAL_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO0 interrupt through debounce (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO1 interrupt through debounce (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO2 interrupt through debounce (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "IGPIO3 interrupt through debounce (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO4 interrupt through debounce (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software timer Interrupt (clear-enable bit)"]
    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Combined Wake up Capture Timer, GPIO and QuadDecoder interrupt (clear-enable bit)"]
    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PCM Interrupt (clear-enable bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter input Interrupt (clear-enable bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter output Interrupt (clear-enable bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Interrupt (clear-enable bit)"]
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
#[doc = "Interrupt set-pending register"]
pub type Ispr = crate::RegValueT<Ispr_SPEC>;

impl Ispr {
    #[doc = "BLE_WAKEUP_LP_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE_GEN_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART2_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "I2C_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SPI_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ADC_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "KEYBRD_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE baseband or Radio diagnostic (Interrupt set-pending bit))"]
    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RFCAL_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO0 interrupt through debounce (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO1 interrupt through debounce (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO2 interrupt through debounce (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "IGPIO3 interrupt through debounce (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO4 interrupt through debounce (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software timer Interrupt (set-pending bit)"]
    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Combined Wake up Capture Timer, GPIO and QuadDecoder interrupt (set-pending bit)"]
    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PCM Interrupt (set-pending bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter input Interrupt (set-pending bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter output Interrupt (set-pending bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Interrupt (set-pending bit)"]
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
#[doc = "Interrupt clear-pending register"]
pub type Icpr = crate::RegValueT<Icpr_SPEC>;

impl Icpr {
    #[doc = "BLE_WAKEUP_LP_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE_GEN_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn ble_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART2_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "I2C_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SPI_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ADC_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "KEYBRD_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BLE baseband or Radio diagnostic (Interrupt clear-pending bit))"]
    #[inline(always)]
    pub fn ble_rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RFCAL_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO0 interrupt through debounce (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn gpio0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO1 interrupt through debounce (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn gpio1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO2 interrupt through debounce (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn gpio2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "IGPIO3 interrupt through debounce (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn gpio3_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPIO4 interrupt through debounce (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn gpio4_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software timer Interrupt (clear-pending bit)"]
    #[inline(always)]
    pub fn swtim_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Combined Wake up Capture Timer, GPIO and QuadDecoder interrupt (clear-pending bit)"]
    #[inline(always)]
    pub fn wkup_quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PCM Interrupt (clear-pending bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter input Interrupt (clear-pending bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sample rate converter output Interrupt (clear-pending bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Interrupt (clear-pending bit)"]
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
#[doc = "Interrupt priority register 0"]
pub type Ipr0 = crate::RegValueT<Ipr0_SPEC>;

impl Ipr0 {
    #[doc = "BLE_WAKEUP_LP_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn ble_wakeup_lp_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BLE_GEN_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn ble_gen_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UART_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn uart_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UART2_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn uart2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Ipr0_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Interrupt priority register 1"]
pub type Ipr1 = crate::RegValueT<Ipr1_SPEC>;

impl Ipr1 {
    #[doc = "I2C_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn i2c_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SPI_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn spi_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn adc_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "KEYBRD_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn keybrd_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Ipr1_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Interrupt priority register 2"]
pub type Ipr2 = crate::RegValueT<Ipr2_SPEC>;

impl Ipr2 {
    #[doc = "BLE_RF_DIAG_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn ble_rf_diag_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RF_CAL_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn rf_cal_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GPIO0_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn gpio0_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GPIO1_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn gpio1_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Ipr2_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Interrupt priority register 3"]
pub type Ipr3 = crate::RegValueT<Ipr3_SPEC>;

impl Ipr3 {
    #[doc = "SPI_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn spi_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SPI2_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn spi2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn adc_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "KEYBRD_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn keybrd_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Ipr3_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Interrupt priority register 4"]
pub type Ipr4 = crate::RegValueT<Ipr4_SPEC>;

impl Ipr4 {
    #[doc = "GPIO2_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn gpio2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GPIO3_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn gpio3_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GPIO4_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn gpio4_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SWTIM_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn swtim_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Ipr4_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Interrupt priority register 5"]
pub type Ipr5 = crate::RegValueT<Ipr5_SPEC>;

impl Ipr5 {
    #[doc = "WKUP_QUADEC_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn wkup_quadec_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PCM_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn pcm_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRC_IN_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn src_in_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRC_OUT_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn src_out_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Ipr5_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Interrupt priority register 6"]
pub type Ipr6 = crate::RegValueT<Ipr6_SPEC>;

impl Ipr6 {
    #[doc = "DMA_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn dma_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ipr6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ipr6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr6 {
    #[inline(always)]
    fn default() -> Ipr6 {
        <crate::RegValueT<Ipr6_SPEC> as RegisterValue<_>>::new(0)
    }
}
