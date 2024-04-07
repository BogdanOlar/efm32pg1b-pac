#[doc = "Register `KEYBUF` reader"]
pub type R = crate::R<KEYBUFrs>;
#[doc = "Register `KEYBUF` writer"]
pub type W = crate::W<KEYBUFrs>;
#[doc = "Field `KEYBUF` reader - Key Buffer Access"]
pub type KeybufR = crate::FieldReader<u32>;
#[doc = "Field `KEYBUF` writer - Key Buffer Access"]
pub type KeybufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&self) -> KeybufR {
        KeybufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    #[must_use]
    pub fn keybuf(&mut self) -> KeybufW<KEYBUFrs> {
        KeybufW::new(self, 0)
    }
}
#[doc = "KEY Buffer Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keybuf::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keybuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYBUFrs;
impl crate::RegisterSpec for KEYBUFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keybuf::R`](R) reader structure"]
impl crate::Readable for KEYBUFrs {}
#[doc = "`write(|w| ..)` method takes [`keybuf::W`](W) writer structure"]
impl crate::Writable for KEYBUFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYBUF to value 0"]
impl crate::Resettable for KEYBUFrs {
    const RESET_VALUE: u32 = 0;
}
