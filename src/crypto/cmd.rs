#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMDrs>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `INSTR` reader - Execute Instruction"]
pub type InstrR = crate::FieldReader;
#[doc = "Field `INSTR` writer - Execute Instruction"]
pub type InstrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEQSTART` writer - Encryption/Decryption SEQUENCE Start"]
pub type SeqstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQSTOP` writer - Sequence Stop"]
pub type SeqstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQSTEP` writer - Sequence Step"]
pub type SeqstepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&self) -> InstrR {
        InstrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    #[must_use]
    pub fn instr(&mut self) -> InstrW<CMDrs> {
        InstrW::new(self, 0)
    }
    #[doc = "Bit 9 - Encryption/Decryption SEQUENCE Start"]
    #[inline(always)]
    #[must_use]
    pub fn seqstart(&mut self) -> SeqstartW<CMDrs> {
        SeqstartW::new(self, 9)
    }
    #[doc = "Bit 10 - Sequence Stop"]
    #[inline(always)]
    #[must_use]
    pub fn seqstop(&mut self) -> SeqstopW<CMDrs> {
        SeqstopW::new(self, 10)
    }
    #[doc = "Bit 11 - Sequence Step"]
    #[inline(always)]
    #[must_use]
    pub fn seqstep(&mut self) -> SeqstepW<CMDrs> {
        SeqstepW::new(self, 11)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMDrs {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
