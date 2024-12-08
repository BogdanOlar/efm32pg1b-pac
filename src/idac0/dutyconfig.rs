///Register `DUTYCONFIG` reader
pub type R = crate::R<DUTYCONFIGrs>;
///Register `DUTYCONFIG` writer
pub type W = crate::W<DUTYCONFIGrs>;
///Field `EM2DUTYCYCLEDIS` reader - Duty Cycle Enable
pub type Em2dutycycledisR = crate::BitReader;
///Field `EM2DUTYCYCLEDIS` writer - Duty Cycle Enable
pub type Em2dutycycledisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Duty Cycle Enable
    #[inline(always)]
    pub fn em2dutycycledis(&self) -> Em2dutycycledisR {
        Em2dutycycledisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DUTYCONFIG")
            .field("em2dutycycledis", &self.em2dutycycledis())
            .finish()
    }
}
impl W {
    ///Bit 1 - Duty Cycle Enable
    #[inline(always)]
    #[must_use]
    pub fn em2dutycycledis(&mut self) -> Em2dutycycledisW<DUTYCONFIGrs> {
        Em2dutycycledisW::new(self, 1)
    }
}
///Duty Cycle Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`dutyconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dutyconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DUTYCONFIGrs;
impl crate::RegisterSpec for DUTYCONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`dutyconfig::R`](R) reader structure
impl crate::Readable for DUTYCONFIGrs {}
///`write(|w| ..)` method takes [`dutyconfig::W`](W) writer structure
impl crate::Writable for DUTYCONFIGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DUTYCONFIG to value 0
impl crate::Resettable for DUTYCONFIGrs {
    const RESET_VALUE: u32 = 0;
}
