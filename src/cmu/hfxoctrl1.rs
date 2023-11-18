#[doc = "Register `HFXOCTRL1` reader"]
pub type R = crate::R<HFXOCTRL1_SPEC>;
#[doc = "Register `HFXOCTRL1` writer"]
pub type W = crate::W<HFXOCTRL1_SPEC>;
#[doc = "Field `PEAKDETTHR` reader - Sets the Peak Detector amplitude detection threshold levels"]
pub type PEAKDETTHR_R = crate::FieldReader;
#[doc = "Field `PEAKDETTHR` writer - Sets the Peak Detector amplitude detection threshold levels"]
pub type PEAKDETTHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `REGLVL` reader - Reserved for internal use. Do not change."]
pub type REGLVL_R = crate::FieldReader;
#[doc = "Field `REGLVL` writer - Reserved for internal use. Do not change."]
pub type REGLVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `XTIBIASEN` reader - Reserved for internal use. Do not change."]
pub type XTIBIASEN_R = crate::BitReader;
#[doc = "Field `XTIBIASEN` writer - Reserved for internal use. Do not change."]
pub type XTIBIASEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    pub fn peakdetthr(&self) -> PEAKDETTHR_R {
        PEAKDETTHR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn reglvl(&self) -> REGLVL_R {
        REGLVL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn xtibiasen(&self) -> XTIBIASEN_R {
        XTIBIASEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOCTRL1")
            .field("peakdetthr", &format_args!("{}", self.peakdetthr().bits()))
            .field("reglvl", &format_args!("{}", self.reglvl().bits()))
            .field("xtibiasen", &format_args!("{}", self.xtibiasen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFXOCTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    #[must_use]
    pub fn peakdetthr(&mut self) -> PEAKDETTHR_W<HFXOCTRL1_SPEC, 0> {
        PEAKDETTHR_W::new(self)
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn reglvl(&mut self) -> REGLVL_W<HFXOCTRL1_SPEC, 4> {
        REGLVL_W::new(self)
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn xtibiasen(&mut self) -> XTIBIASEN_W<HFXOCTRL1_SPEC, 9> {
        XTIBIASEN_W::new(self)
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
#[doc = "HFXO Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOCTRL1_SPEC;
impl crate::RegisterSpec for HFXOCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxoctrl1::R`](R) reader structure"]
impl crate::Readable for HFXOCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxoctrl1::W`](W) writer structure"]
impl crate::Writable for HFXOCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFXOCTRL1 to value 0x0240"]
impl crate::Resettable for HFXOCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0240;
}
