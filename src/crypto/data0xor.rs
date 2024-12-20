///Register `DATA0XOR` reader
pub type R = crate::R<DATA0XORrs>;
///Register `DATA0XOR` writer
pub type W = crate::W<DATA0XORrs>;
///Field `DATA0XOR` reader - XOR Data 0 Access
pub type Data0xorR = crate::FieldReader<u32>;
///Field `DATA0XOR` writer - XOR Data 0 Access
pub type Data0xorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - XOR Data 0 Access
    #[inline(always)]
    pub fn data0xor(&self) -> Data0xorR {
        Data0xorR::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA0XORrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - XOR Data 0 Access
    #[inline(always)]
    #[must_use]
    pub fn data0xor(&mut self) -> Data0xorW<DATA0XORrs> {
        Data0xorW::new(self, 0)
    }
}
///DATA0XOR Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`data0xor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0xor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DATA0XORrs;
impl crate::RegisterSpec for DATA0XORrs {
    type Ux = u32;
}
///`read()` method returns [`data0xor::R`](R) reader structure
impl crate::Readable for DATA0XORrs {}
///`write(|w| ..)` method takes [`data0xor::W`](W) writer structure
impl crate::Writable for DATA0XORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA0XOR to value 0
impl crate::Resettable for DATA0XORrs {
    const RESET_VALUE: u32 = 0;
}
