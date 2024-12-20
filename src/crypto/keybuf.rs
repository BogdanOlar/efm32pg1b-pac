///Register `KEYBUF` reader
pub type R = crate::R<KEYBUFrs>;
///Register `KEYBUF` writer
pub type W = crate::W<KEYBUFrs>;
///Field `KEYBUF` reader - Key Buffer Access
pub type KeybufR = crate::FieldReader<u32>;
///Field `KEYBUF` writer - Key Buffer Access
pub type KeybufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Key Buffer Access
    #[inline(always)]
    pub fn keybuf(&self) -> KeybufR {
        KeybufR::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<KEYBUFrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Key Buffer Access
    #[inline(always)]
    #[must_use]
    pub fn keybuf(&mut self) -> KeybufW<KEYBUFrs> {
        KeybufW::new(self, 0)
    }
}
///KEY Buffer Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`keybuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keybuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct KEYBUFrs;
impl crate::RegisterSpec for KEYBUFrs {
    type Ux = u32;
}
///`read()` method returns [`keybuf::R`](R) reader structure
impl crate::Readable for KEYBUFrs {}
///`write(|w| ..)` method takes [`keybuf::W`](W) writer structure
impl crate::Writable for KEYBUFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets KEYBUF to value 0
impl crate::Resettable for KEYBUFrs {
    const RESET_VALUE: u32 = 0;
}
