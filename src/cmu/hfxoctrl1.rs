///Register `HFXOCTRL1` reader
pub type R = crate::R<HFXOCTRL1rs>;
///Register `HFXOCTRL1` writer
pub type W = crate::W<HFXOCTRL1rs>;
///Field `PEAKDETTHR` reader - Sets the Peak Detector amplitude detection threshold levels
pub type PeakdetthrR = crate::FieldReader;
///Field `PEAKDETTHR` writer - Sets the Peak Detector amplitude detection threshold levels
pub type PeakdetthrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `REGLVL` reader - Reserved for internal use. Do not change.
pub type ReglvlR = crate::FieldReader;
///Field `REGLVL` writer - Reserved for internal use. Do not change.
pub type ReglvlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `XTIBIASEN` reader - Reserved for internal use. Do not change.
pub type XtibiasenR = crate::BitReader;
///Field `XTIBIASEN` writer - Reserved for internal use. Do not change.
pub type XtibiasenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels
    #[inline(always)]
    pub fn peakdetthr(&self) -> PeakdetthrR {
        PeakdetthrR::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Reserved for internal use. Do not change.
    #[inline(always)]
    pub fn reglvl(&self) -> ReglvlR {
        ReglvlR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 9 - Reserved for internal use. Do not change.
    #[inline(always)]
    pub fn xtibiasen(&self) -> XtibiasenR {
        XtibiasenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOCTRL1")
            .field("peakdetthr", &self.peakdetthr())
            .field("reglvl", &self.reglvl())
            .field("xtibiasen", &self.xtibiasen())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels
    #[inline(always)]
    #[must_use]
    pub fn peakdetthr(&mut self) -> PeakdetthrW<HFXOCTRL1rs> {
        PeakdetthrW::new(self, 0)
    }
    ///Bits 4:6 - Reserved for internal use. Do not change.
    #[inline(always)]
    #[must_use]
    pub fn reglvl(&mut self) -> ReglvlW<HFXOCTRL1rs> {
        ReglvlW::new(self, 4)
    }
    ///Bit 9 - Reserved for internal use. Do not change.
    #[inline(always)]
    #[must_use]
    pub fn xtibiasen(&mut self) -> XtibiasenW<HFXOCTRL1rs> {
        XtibiasenW::new(self, 9)
    }
}
///HFXO Control 1
///
///You can [`read`](crate::Reg::read) this register and get [`hfxoctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxoctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFXOCTRL1rs;
impl crate::RegisterSpec for HFXOCTRL1rs {
    type Ux = u32;
}
///`read()` method returns [`hfxoctrl1::R`](R) reader structure
impl crate::Readable for HFXOCTRL1rs {}
///`write(|w| ..)` method takes [`hfxoctrl1::W`](W) writer structure
impl crate::Writable for HFXOCTRL1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFXOCTRL1 to value 0x0240
impl crate::Resettable for HFXOCTRL1rs {
    const RESET_VALUE: u32 = 0x0240;
}
