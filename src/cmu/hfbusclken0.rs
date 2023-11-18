#[doc = "Register `HFBUSCLKEN0` reader"]
pub type R = crate::R<HFBUSCLKEN0_SPEC>;
#[doc = "Register `HFBUSCLKEN0` writer"]
pub type W = crate::W<HFBUSCLKEN0_SPEC>;
#[doc = "Field `LE` reader - Low Energy Peripheral Interface Clock Enable"]
pub type LE_R = crate::BitReader;
#[doc = "Field `LE` writer - Low Energy Peripheral Interface Clock Enable"]
pub type LE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRYPTO` reader - Advanced Encryption Standard Accelerator Clock Enable"]
pub type CRYPTO_R = crate::BitReader;
#[doc = "Field `CRYPTO` writer - Advanced Encryption Standard Accelerator Clock Enable"]
pub type CRYPTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO` reader - General purpose Input/Output Clock Enable"]
pub type GPIO_R = crate::BitReader;
#[doc = "Field `GPIO` writer - General purpose Input/Output Clock Enable"]
pub type GPIO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRS` reader - Peripheral Reflex System Clock Enable"]
pub type PRS_R = crate::BitReader;
#[doc = "Field `PRS` writer - Peripheral Reflex System Clock Enable"]
pub type PRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LDMA` reader - Linked Direct Memory Access Controller Clock Enable"]
pub type LDMA_R = crate::BitReader;
#[doc = "Field `LDMA` writer - Linked Direct Memory Access Controller Clock Enable"]
pub type LDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPCRC` reader - General Purpose CRC Clock Enable"]
pub type GPCRC_R = crate::BitReader;
#[doc = "Field `GPCRC` writer - General Purpose CRC Clock Enable"]
pub type GPCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn crypto(&self) -> CRYPTO_R {
        CRYPTO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFBUSCLKEN0")
            .field("le", &format_args!("{}", self.le().bit()))
            .field("crypto", &format_args!("{}", self.crypto().bit()))
            .field("gpio", &format_args!("{}", self.gpio().bit()))
            .field("prs", &format_args!("{}", self.prs().bit()))
            .field("ldma", &format_args!("{}", self.ldma().bit()))
            .field("gpcrc", &format_args!("{}", self.gpcrc().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFBUSCLKEN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn le(&mut self) -> LE_W<HFBUSCLKEN0_SPEC, 0> {
        LE_W::new(self)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypto(&mut self) -> CRYPTO_W<HFBUSCLKEN0_SPEC, 1> {
        CRYPTO_W::new(self)
    }
    #[doc = "Bit 2 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<HFBUSCLKEN0_SPEC, 2> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 3 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<HFBUSCLKEN0_SPEC, 3> {
        PRS_W::new(self)
    }
    #[doc = "Bit 4 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LDMA_W<HFBUSCLKEN0_SPEC, 4> {
        LDMA_W::new(self)
    }
    #[doc = "Bit 5 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpcrc(&mut self) -> GPCRC_W<HFBUSCLKEN0_SPEC, 5> {
        GPCRC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High Frequency Bus Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfbusclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfbusclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFBUSCLKEN0_SPEC;
impl crate::RegisterSpec for HFBUSCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfbusclken0::R`](R) reader structure"]
impl crate::Readable for HFBUSCLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfbusclken0::W`](W) writer structure"]
impl crate::Writable for HFBUSCLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFBUSCLKEN0 to value 0"]
impl crate::Resettable for HFBUSCLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
