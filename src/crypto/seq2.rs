#[doc = "Register `SEQ2` reader"]
pub type R = crate::R<SEQ2rs>;
#[doc = "Register `SEQ2` writer"]
pub type W = crate::W<SEQ2rs>;
#[doc = "Field `INSTR8` reader - Sequence Instruction 8"]
pub type Instr8R = crate::FieldReader;
#[doc = "Field `INSTR8` writer - Sequence Instruction 8"]
pub type Instr8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR9` reader - Sequence Instruction 9"]
pub type Instr9R = crate::FieldReader;
#[doc = "Field `INSTR9` writer - Sequence Instruction 9"]
pub type Instr9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR10` reader - Sequence Instruction 10"]
pub type Instr10R = crate::FieldReader;
#[doc = "Field `INSTR10` writer - Sequence Instruction 10"]
pub type Instr10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR11` reader - Sequence Instruction 11"]
pub type Instr11R = crate::FieldReader;
#[doc = "Field `INSTR11` writer - Sequence Instruction 11"]
pub type Instr11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 8"]
    #[inline(always)]
    pub fn instr8(&self) -> Instr8R {
        Instr8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 9"]
    #[inline(always)]
    pub fn instr9(&self) -> Instr9R {
        Instr9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 10"]
    #[inline(always)]
    pub fn instr10(&self) -> Instr10R {
        Instr10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 11"]
    #[inline(always)]
    pub fn instr11(&self) -> Instr11R {
        Instr11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 8"]
    #[inline(always)]
    #[must_use]
    pub fn instr8(&mut self) -> Instr8W<SEQ2rs> {
        Instr8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 9"]
    #[inline(always)]
    #[must_use]
    pub fn instr9(&mut self) -> Instr9W<SEQ2rs> {
        Instr9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 10"]
    #[inline(always)]
    #[must_use]
    pub fn instr10(&mut self) -> Instr10W<SEQ2rs> {
        Instr10W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 11"]
    #[inline(always)]
    #[must_use]
    pub fn instr11(&mut self) -> Instr11W<SEQ2rs> {
        Instr11W::new(self, 24)
    }
}
#[doc = "Sequence Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ2rs;
impl crate::RegisterSpec for SEQ2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq2::R`](R) reader structure"]
impl crate::Readable for SEQ2rs {}
#[doc = "`write(|w| ..)` method takes [`seq2::W`](W) writer structure"]
impl crate::Writable for SEQ2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ2 to value 0"]
impl crate::Resettable for SEQ2rs {
    const RESET_VALUE: u32 = 0;
}
