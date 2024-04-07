#[doc = "Register `HFXOCTRL1` reader"]
pub type R = crate::R<HFXOCTRL1rs>;
#[doc = "Register `HFXOCTRL1` writer"]
pub type W = crate::W<HFXOCTRL1rs>;
#[doc = "Field `PEAKDETTHR` reader - Sets the Peak Detector amplitude detection threshold levels"]
pub type PeakdetthrR = crate::FieldReader;
#[doc = "Field `PEAKDETTHR` writer - Sets the Peak Detector amplitude detection threshold levels"]
pub type PeakdetthrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REGLVL` reader - Reserved for internal use. Do not change."]
pub type ReglvlR = crate::FieldReader;
#[doc = "Field `REGLVL` writer - Reserved for internal use. Do not change."]
pub type ReglvlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTIBIASEN` reader - Reserved for internal use. Do not change."]
pub type XtibiasenR = crate::BitReader;
#[doc = "Field `XTIBIASEN` writer - Reserved for internal use. Do not change."]
pub type XtibiasenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    pub fn peakdetthr(&self) -> PeakdetthrR {
        PeakdetthrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn reglvl(&self) -> ReglvlR {
        ReglvlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn xtibiasen(&self) -> XtibiasenR {
        XtibiasenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    #[must_use]
    pub fn peakdetthr(&mut self) -> PeakdetthrW<HFXOCTRL1rs> {
        PeakdetthrW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn reglvl(&mut self) -> ReglvlW<HFXOCTRL1rs> {
        ReglvlW::new(self, 4)
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn xtibiasen(&mut self) -> XtibiasenW<HFXOCTRL1rs> {
        XtibiasenW::new(self, 9)
    }
}
#[doc = "HFXO Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOCTRL1rs;
impl crate::RegisterSpec for HFXOCTRL1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxoctrl1::R`](R) reader structure"]
impl crate::Readable for HFXOCTRL1rs {}
#[doc = "`write(|w| ..)` method takes [`hfxoctrl1::W`](W) writer structure"]
impl crate::Writable for HFXOCTRL1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOCTRL1 to value 0x0240"]
impl crate::Resettable for HFXOCTRL1rs {
    const RESET_VALUE: u32 = 0x0240;
}
