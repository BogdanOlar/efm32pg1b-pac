///Register `TEMPLIMITS` reader
pub type R = crate::R<TEMPLIMITSrs>;
///Register `TEMPLIMITS` writer
pub type W = crate::W<TEMPLIMITSrs>;
///Field `TEMPLOW` reader - Temperature Low Limit
pub type TemplowR = crate::FieldReader;
///Field `TEMPLOW` writer - Temperature Low Limit
pub type TemplowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TEMPHIGH` reader - Temperature High Limit
pub type TemphighR = crate::FieldReader;
///Field `TEMPHIGH` writer - Temperature High Limit
pub type TemphighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EM4WUEN` reader - Enable EM4 Wakeup Due to Low/high Temperature
pub type Em4wuenR = crate::BitReader;
///Field `EM4WUEN` writer - Enable EM4 Wakeup Due to Low/high Temperature
pub type Em4wuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Temperature Low Limit
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Temperature High Limit
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature
    #[inline(always)]
    pub fn em4wuen(&self) -> Em4wuenR {
        Em4wuenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMPLIMITS")
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .field("em4wuen", &self.em4wuen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Temperature Low Limit
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TemplowW<TEMPLIMITSrs> {
        TemplowW::new(self, 0)
    }
    ///Bits 8:15 - Temperature High Limit
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TemphighW<TEMPLIMITSrs> {
        TemphighW::new(self, 8)
    }
    ///Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature
    #[inline(always)]
    #[must_use]
    pub fn em4wuen(&mut self) -> Em4wuenW<TEMPLIMITSrs> {
        Em4wuenW::new(self, 16)
    }
}
///Temperature Limits for Interrupt Generation
///
///You can [`read`](crate::Reg::read) this register and get [`templimits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`templimits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TEMPLIMITSrs;
impl crate::RegisterSpec for TEMPLIMITSrs {
    type Ux = u32;
}
///`read()` method returns [`templimits::R`](R) reader structure
impl crate::Readable for TEMPLIMITSrs {}
///`write(|w| ..)` method takes [`templimits::W`](W) writer structure
impl crate::Writable for TEMPLIMITSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TEMPLIMITS to value 0xff00
impl crate::Resettable for TEMPLIMITSrs {
    const RESET_VALUE: u32 = 0xff00;
}
