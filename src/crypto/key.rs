///Register `KEY` reader
pub type R = crate::R<KEYrs>;
///Register `KEY` writer
pub type W = crate::W<KEYrs>;
///Field `KEY` reader - Key Access
pub type KeyR = crate::FieldReader<u32>;
///Field `KEY` writer - Key Access
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Key Access
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<KEYrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Key Access
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<KEYrs> {
        KeyW::new(self, 0)
    }
}
///KEY Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct KEYrs;
impl crate::RegisterSpec for KEYrs {
    type Ux = u32;
}
///`read()` method returns [`key::R`](R) reader structure
impl crate::Readable for KEYrs {}
///`write(|w| ..)` method takes [`key::W`](W) writer structure
impl crate::Writable for KEYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets KEY to value 0
impl crate::Resettable for KEYrs {
    const RESET_VALUE: u32 = 0;
}
