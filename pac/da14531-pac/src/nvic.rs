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

    #[doc = "Interrupt priority register 7"]
    #[inline(always)]
    pub const fn ipr7(&self) -> &'static crate::common::Reg<self::Ipr7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipr7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(796usize),
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

    #[doc = "FTDF_WAKEUP_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn ftdf_wakeup_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FTDF_GEN_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn ftdf_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RFCAL_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "COEX_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn coex_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CRYPTO_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn crypto_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MRM_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn mrm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART2_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C2_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn i2c2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI2_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn spi2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ADC_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "KEYBRD_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IRGEN_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn irgen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "WKUP_GPIO_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn wkup_gpio_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM0_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn swtim0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM1_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn swtim1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "QUADEC_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn usb_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_IN_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_OUT_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VBUS_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn vbus_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RF_DIAG_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TRNG_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn trng_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDC_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn dcdc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "XTAL16RDY_IRQn (Interrupt set-enable bit)"]
    #[inline(always)]
    pub fn xtal16rdy_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Rsvd__irq__n (Reserved)"]
    #[inline(always)]
    pub fn rsvd__irq__n(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Iser_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Iser_SPEC, crate::common::RW>::from_register(
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

    #[doc = "FTDF_WAKEUP_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn ftdf_wakeup_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FTDF_GEN_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn ftdf_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RFCAL_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "COEX_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn coex_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CRYPTO_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn crypto_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MRM_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn mrm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART2_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C2_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn i2c2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI2_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn spi2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ADC_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "KEYBRD_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IRGEN_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn irgen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "WKUP_GPIO_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn wkup_gpio_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM0_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn swtim0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM1_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn swtim1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "QUADEC_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn usb_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_IN_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_OUT_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VBUS_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn vbus_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RF_DIAG_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TRNG_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn trng_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDC_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn dcdc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "XTAL16RDY_IRQn (Interrupt clear-enable bit)"]
    #[inline(always)]
    pub fn xtal16rdy_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Rsvd__irq__n (Reserved)"]
    #[inline(always)]
    pub fn rsvd__irq__n(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Icer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Icer_SPEC, crate::common::RW>::from_register(
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

    #[doc = "FTDF_WAKEUP_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn ftdf_wakeup_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FTDF_GEN_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn ftdf_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RFCAL_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "COEX_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn coex_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CRYPTO_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn crypto_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MRM_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn mrm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART2_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C2_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn i2c2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI2_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn spi2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ADC_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "KEYBRD_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IRGEN_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn irgen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "WKUP_GPIO_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn wkup_gpio_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM0_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn swtim0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM1_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn swtim1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "QUADEC_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn usb_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_IN_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_OUT_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VBUS_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn vbus_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RF_DIAG_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TRNG_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn trng_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDC_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn dcdc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "XTAL16RDY_IRQn (Interrupt set-pending bit)"]
    #[inline(always)]
    pub fn xtal16rdy_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Rsvd__irq__n (Reserved)"]
    #[inline(always)]
    pub fn rsvd__irq__n(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ispr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ispr_SPEC, crate::common::RW>::from_register(
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

    #[doc = "FTDF_WAKEUP_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn ftdf_wakeup_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FTDF_GEN_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn ftdf_gen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RFCAL_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn rfcal_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "COEX_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn coex_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CRYPTO_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn crypto_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MRM_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn mrm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn uart_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "UART2_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn uart2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn i2c_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C2_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn i2c2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn spi_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPI2_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn spi2_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ADC_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn adc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "KEYBRD_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn keybrd_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IRGEN_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn irgen_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "WKUP_GPIO_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn wkup_gpio_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM0_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn swtim0_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SWTIM1_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn swtim1_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "QUADEC_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn quadec_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn usb_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn pcm_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_IN_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn src_in_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SRC_OUT_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn src_out_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VBUS_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn vbus_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn dma_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RF_DIAG_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn rf_diag_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TRNG_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn trng_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDC_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn dcdc_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "XTAL16RDY_IRQn (Interrupt clear-pending bit)"]
    #[inline(always)]
    pub fn xtal16rdy_irqn(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Rsvd__irq__n (Reserved)"]
    #[inline(always)]
    pub fn rsvd__irq__n(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Icpr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Icpr_SPEC, crate::common::RW>::from_register(
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
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "BLE_GEN_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn ble_gen_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FTDF_WAKEUP_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn ftdf_wakeup_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FTDF_GEN_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn ftdf_gen_irqn_prio(
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

#[doc = "Interrupt priority register 1"]
pub type Ipr1 = crate::RegValueT<Ipr1_SPEC>;

impl Ipr1 {
    #[doc = "RFCAL_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn rfcal_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "COEX_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn coex_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CRYPTO_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn crypto_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MRM_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn mrm_irqn_prio(
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

#[doc = "Interrupt priority register 2"]
pub type Ipr2 = crate::RegValueT<Ipr2_SPEC>;

impl Ipr2 {
    #[doc = "UART_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn uart_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "UART2_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn uart2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "I2C_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn i2c_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "I2C2_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn i2c2_irqn_prio(
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

#[doc = "Interrupt priority register 3"]
pub type Ipr3 = crate::RegValueT<Ipr3_SPEC>;

impl Ipr3 {
    #[doc = "SPI_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn spi_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SPI2_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn spi2_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ADC_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn adc_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "KEYBRD_IRQn\\[7:0\\] bits (Interrupt priority)"]
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

#[doc = "Interrupt priority register 4"]
pub type Ipr4 = crate::RegValueT<Ipr4_SPEC>;

impl Ipr4 {
    #[doc = "IRGEN_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn irgen_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "WKUP_GPIO_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn wkup_gpio_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SWTIM0_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn swtim0_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SWTIM1_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn swtim1_irqn_prio(
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

#[doc = "Interrupt priority register 5"]
pub type Ipr5 = crate::RegValueT<Ipr5_SPEC>;

impl Ipr5 {
    #[doc = "QUADEC_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn quadec_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "USB_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn usb_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PCM_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn pcm_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC_IN_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn src_in_irqn_prio(
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

#[doc = "Interrupt priority register 6"]
pub type Ipr6 = crate::RegValueT<Ipr6_SPEC>;

impl Ipr6 {
    #[doc = "SRC_OUT_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn src_out_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VBUS_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn vbus_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn dma_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RF_DIAG_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn rf_diag_irqn_prio(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr6 {
    #[inline(always)]
    fn default() -> Ipr6 {
        <crate::RegValueT<Ipr6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr7_SPEC;
impl crate::sealed::RegSpec for Ipr7_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt priority register 7"]
pub type Ipr7 = crate::RegValueT<Ipr7_SPEC>;

impl Ipr7 {
    #[doc = "TRNG_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn trng_irqn_prio(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ipr7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ipr7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DCDC_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn dcdc_irqn_prio(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ipr7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ipr7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "XTAL16RDY_IRQn\\[7:0\\] bits (Interrupt priority)"]
    #[inline(always)]
    pub fn xtal16rdy_irqn_prio(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ipr7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ipr7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RESERVED31_IRQn\\[7:0\\] bits (Reserved)"]
    #[inline(always)]
    pub fn reserved31_irqn_dont_use(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ipr7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ipr7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipr7 {
    #[inline(always)]
    fn default() -> Ipr7 {
        <crate::RegValueT<Ipr7_SPEC> as RegisterValue<_>>::new(0)
    }
}
