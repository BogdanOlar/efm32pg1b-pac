#[doc = "Register `CH5_CFG` reader"]
pub type R = crate::R<CH5_CFGrs>;
#[doc = "Register `CH5_CFG` writer"]
pub type W = crate::W<CH5_CFGrs>;
#[doc = "Arbitration Slot Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBSLOTS {
    #[doc = "0: One arbitration slot selected"]
    One = 0,
    #[doc = "1: Two arbitration slots selected"]
    Two = 1,
    #[doc = "2: Four arbitration slots selected"]
    Four = 2,
    #[doc = "3: Eight arbitration slots selected"]
    Eight = 3,
}
impl From<ARBSLOTS> for u8 {
    #[inline(always)]
    fn from(variant: ARBSLOTS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARBSLOTS {
    type Ux = u8;
}
impl crate::IsEnum for ARBSLOTS {}
#[doc = "Field `ARBSLOTS` reader - Arbitration Slot Number Select"]
pub type ArbslotsR = crate::FieldReader<ARBSLOTS>;
impl ArbslotsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARBSLOTS {
        match self.bits {
            0 => ARBSLOTS::One,
            1 => ARBSLOTS::Two,
            2 => ARBSLOTS::Four,
            3 => ARBSLOTS::Eight,
            _ => unreachable!(),
        }
    }
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ARBSLOTS::One
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ARBSLOTS::Two
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == ARBSLOTS::Four
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == ARBSLOTS::Eight
    }
}
#[doc = "Field `ARBSLOTS` writer - Arbitration Slot Number Select"]
pub type ArbslotsW<'a, REG> = crate::FieldWriter<'a, REG, 2, ARBSLOTS, crate::Safe>;
impl<'a, REG> ArbslotsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::One)
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::Two)
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::Four)
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::Eight)
    }
}
#[doc = "Field `SRCINCSIGN` reader - Source Address Increment Sign"]
pub type SrcincsignR = crate::BitReader;
#[doc = "Field `SRCINCSIGN` writer - Source Address Increment Sign"]
pub type SrcincsignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTINCSIGN` reader - Destination Address Increment Sign"]
pub type DstincsignR = crate::BitReader;
#[doc = "Field `DSTINCSIGN` writer - Destination Address Increment Sign"]
pub type DstincsignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&self) -> ArbslotsR {
        ArbslotsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&self) -> SrcincsignR {
        SrcincsignR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&self) -> DstincsignR {
        DstincsignR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    #[must_use]
    pub fn arbslots(&mut self) -> ArbslotsW<CH5_CFGrs> {
        ArbslotsW::new(self, 16)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    #[must_use]
    pub fn srcincsign(&mut self) -> SrcincsignW<CH5_CFGrs> {
        SrcincsignW::new(self, 20)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    #[must_use]
    pub fn dstincsign(&mut self) -> DstincsignW<CH5_CFGrs> {
        DstincsignW::new(self, 21)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH5_CFGrs;
impl crate::RegisterSpec for CH5_CFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_cfg::R`](R) reader structure"]
impl crate::Readable for CH5_CFGrs {}
#[doc = "`write(|w| ..)` method takes [`ch5_cfg::W`](W) writer structure"]
impl crate::Writable for CH5_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH5_CFG to value 0"]
impl crate::Resettable for CH5_CFGrs {
    const RESET_VALUE: u32 = 0;
}
