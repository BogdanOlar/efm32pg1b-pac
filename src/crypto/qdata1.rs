///Register `QDATA1` reader
pub type R = crate::R<QDATA1rs>;
///Register `QDATA1` writer
pub type W = crate::W<QDATA1rs>;
///Field `QDATA1` reader - Quad Data 1 Access
pub type Qdata1R = crate::FieldReader<u32>;
///Field `QDATA1` writer - Quad Data 1 Access
pub type Qdata1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Quad Data 1 Access
    #[inline(always)]
    pub fn qdata1(&self) -> Qdata1R {
        Qdata1R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<QDATA1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Quad Data 1 Access
    #[inline(always)]
    #[must_use]
    pub fn qdata1(&mut self) -> Qdata1W<QDATA1rs> {
        Qdata1W::new(self, 0)
    }
}
///QDATA1 Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`qdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct QDATA1rs;
impl crate::RegisterSpec for QDATA1rs {
    type Ux = u32;
}
///`read()` method returns [`qdata1::R`](R) reader structure
impl crate::Readable for QDATA1rs {}
///`write(|w| ..)` method takes [`qdata1::W`](W) writer structure
impl crate::Writable for QDATA1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QDATA1 to value 0
impl crate::Resettable for QDATA1rs {
    const RESET_VALUE: u32 = 0;
}
