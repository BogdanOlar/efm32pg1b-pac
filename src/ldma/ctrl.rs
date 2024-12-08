///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `SYNCPRSSETEN` reader - Synchronization PRS Set Enable
pub type SyncprssetenR = crate::FieldReader;
///Field `SYNCPRSSETEN` writer - Synchronization PRS Set Enable
pub type SyncprssetenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SYNCPRSCLREN` reader - Synchronization PRS Clear Enable
pub type SyncprsclrenR = crate::FieldReader;
///Field `SYNCPRSCLREN` writer - Synchronization PRS Clear Enable
pub type SyncprsclrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NUMFIXED` reader - Number of Fixed Priority Channels
pub type NumfixedR = crate::FieldReader;
///Field `NUMFIXED` writer - Number of Fixed Priority Channels
pub type NumfixedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:7 - Synchronization PRS Set Enable
    #[inline(always)]
    pub fn syncprsseten(&self) -> SyncprssetenR {
        SyncprssetenR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Synchronization PRS Clear Enable
    #[inline(always)]
    pub fn syncprsclren(&self) -> SyncprsclrenR {
        SyncprsclrenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:26 - Number of Fixed Priority Channels
    #[inline(always)]
    pub fn numfixed(&self) -> NumfixedR {
        NumfixedR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("syncprsseten", &self.syncprsseten())
            .field("syncprsclren", &self.syncprsclren())
            .field("numfixed", &self.numfixed())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Synchronization PRS Set Enable
    #[inline(always)]
    #[must_use]
    pub fn syncprsseten(&mut self) -> SyncprssetenW<CTRLrs> {
        SyncprssetenW::new(self, 0)
    }
    ///Bits 8:15 - Synchronization PRS Clear Enable
    #[inline(always)]
    #[must_use]
    pub fn syncprsclren(&mut self) -> SyncprsclrenW<CTRLrs> {
        SyncprsclrenW::new(self, 8)
    }
    ///Bits 24:26 - Number of Fixed Priority Channels
    #[inline(always)]
    #[must_use]
    pub fn numfixed(&mut self) -> NumfixedW<CTRLrs> {
        NumfixedW::new(self, 24)
    }
}
///DMA Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0x0700_0000
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x0700_0000;
}
