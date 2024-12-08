///Register `DATA1BYTE` reader
pub type R = crate::R<DATA1BYTErs>;
///Register `DATA1BYTE` writer
pub type W = crate::W<DATA1BYTErs>;
///Field `DATA1BYTE` reader - Data 1 Byte Access
pub type Data1byteR = crate::FieldReader;
///Field `DATA1BYTE` writer - Data 1 Byte Access
pub type Data1byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 1 Byte Access
    #[inline(always)]
    pub fn data1byte(&self) -> Data1byteR {
        Data1byteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA1BYTErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Data 1 Byte Access
    #[inline(always)]
    #[must_use]
    pub fn data1byte(&mut self) -> Data1byteW<DATA1BYTErs> {
        Data1byteW::new(self, 0)
    }
}
///DATA1 Register Byte Access
///
///You can [`read`](crate::Reg::read) this register and get [`data1byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DATA1BYTErs;
impl crate::RegisterSpec for DATA1BYTErs {
    type Ux = u32;
}
///`read()` method returns [`data1byte::R`](R) reader structure
impl crate::Readable for DATA1BYTErs {}
///`write(|w| ..)` method takes [`data1byte::W`](W) writer structure
impl crate::Writable for DATA1BYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA1BYTE to value 0
impl crate::Resettable for DATA1BYTErs {
    const RESET_VALUE: u32 = 0;
}
