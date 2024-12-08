///Register `DATA1` reader
pub type R = crate::R<DATA1rs>;
///Register `DATA1` writer
pub type W = crate::W<DATA1rs>;
///Field `DATA1` reader - Data 1 Access
pub type Data1R = crate::FieldReader<u32>;
///Field `DATA1` writer - Data 1 Access
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data 1 Access
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Data 1 Access
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<DATA1rs> {
        Data1W::new(self, 0)
    }
}
///DATA1 Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DATA1rs;
impl crate::RegisterSpec for DATA1rs {
    type Ux = u32;
}
///`read()` method returns [`data1::R`](R) reader structure
impl crate::Readable for DATA1rs {}
///`write(|w| ..)` method takes [`data1::W`](W) writer structure
impl crate::Writable for DATA1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA1 to value 0
impl crate::Resettable for DATA1rs {
    const RESET_VALUE: u32 = 0;
}
