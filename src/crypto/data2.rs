///Register `DATA2` reader
pub type R = crate::R<DATA2rs>;
///Register `DATA2` writer
pub type W = crate::W<DATA2rs>;
///Field `DATA2` reader - Data 2 Access
pub type Data2R = crate::FieldReader<u32>;
///Field `DATA2` writer - Data 2 Access
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data 2 Access
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Data 2 Access
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> Data2W<DATA2rs> {
        Data2W::new(self, 0)
    }
}
///DATA2 Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DATA2rs;
impl crate::RegisterSpec for DATA2rs {
    type Ux = u32;
}
///`read()` method returns [`data2::R`](R) reader structure
impl crate::Readable for DATA2rs {}
///`write(|w| ..)` method takes [`data2::W`](W) writer structure
impl crate::Writable for DATA2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA2 to value 0
impl crate::Resettable for DATA2rs {
    const RESET_VALUE: u32 = 0;
}
