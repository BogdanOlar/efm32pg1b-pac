///Register `QDATA1BYTE` reader
pub type R = crate::R<QDATA1BYTErs>;
///Register `QDATA1BYTE` writer
pub type W = crate::W<QDATA1BYTErs>;
///Field `QDATA1BYTE` reader - Qdata 1 Byte Access
pub type Qdata1byteR = crate::FieldReader;
///Field `QDATA1BYTE` writer - Qdata 1 Byte Access
pub type Qdata1byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Qdata 1 Byte Access
    #[inline(always)]
    pub fn qdata1byte(&self) -> Qdata1byteR {
        Qdata1byteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<QDATA1BYTErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Qdata 1 Byte Access
    #[inline(always)]
    #[must_use]
    pub fn qdata1byte(&mut self) -> Qdata1byteW<QDATA1BYTErs> {
        Qdata1byteW::new(self, 0)
    }
}
///QDATA1 Register Byte Access
///
///You can [`read`](crate::Reg::read) this register and get [`qdata1byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct QDATA1BYTErs;
impl crate::RegisterSpec for QDATA1BYTErs {
    type Ux = u32;
}
///`read()` method returns [`qdata1byte::R`](R) reader structure
impl crate::Readable for QDATA1BYTErs {}
///`write(|w| ..)` method takes [`qdata1byte::W`](W) writer structure
impl crate::Writable for QDATA1BYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QDATA1BYTE to value 0
impl crate::Resettable for QDATA1BYTErs {
    const RESET_VALUE: u32 = 0;
}
