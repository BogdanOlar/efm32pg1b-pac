///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `EM2BLOCK` reader - Energy Mode 2 Block
pub type Em2blockR = crate::BitReader;
///Field `EM2BLOCK` writer - Energy Mode 2 Block
pub type Em2blockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Energy Mode 2 Block
    #[inline(always)]
    pub fn em2block(&self) -> Em2blockR {
        Em2blockR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("em2block", &self.em2block())
            .finish()
    }
}
impl W {
    ///Bit 1 - Energy Mode 2 Block
    #[inline(always)]
    #[must_use]
    pub fn em2block(&mut self) -> Em2blockW<CTRLrs> {
        Em2blockW::new(self, 1)
    }
}
///Control Register
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
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
