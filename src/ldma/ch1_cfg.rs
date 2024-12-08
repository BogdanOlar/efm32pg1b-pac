///Register `CH1_CFG` reader
pub type R = crate::R<CH1_CFGrs>;
///Register `CH1_CFG` writer
pub type W = crate::W<CH1_CFGrs>;
///Arbitration Slot Number Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBSLOTS {
    ///0: One arbitration slot selected
    One = 0,
    ///1: Two arbitration slots selected
    Two = 1,
    ///2: Four arbitration slots selected
    Four = 2,
    ///3: Eight arbitration slots selected
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
///Field `ARBSLOTS` reader - Arbitration Slot Number Select
pub type ArbslotsR = crate::FieldReader<ARBSLOTS>;
impl ArbslotsR {
    ///Get enumerated values variant
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
    ///One arbitration slot selected
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ARBSLOTS::One
    }
    ///Two arbitration slots selected
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ARBSLOTS::Two
    }
    ///Four arbitration slots selected
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == ARBSLOTS::Four
    }
    ///Eight arbitration slots selected
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == ARBSLOTS::Eight
    }
}
///Field `ARBSLOTS` writer - Arbitration Slot Number Select
pub type ArbslotsW<'a, REG> = crate::FieldWriter<'a, REG, 2, ARBSLOTS, crate::Safe>;
impl<'a, REG> ArbslotsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///One arbitration slot selected
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::One)
    }
    ///Two arbitration slots selected
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::Two)
    }
    ///Four arbitration slots selected
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::Four)
    }
    ///Eight arbitration slots selected
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS::Eight)
    }
}
///Field `SRCINCSIGN` reader - Source Address Increment Sign
pub type SrcincsignR = crate::BitReader;
///Field `SRCINCSIGN` writer - Source Address Increment Sign
pub type SrcincsignW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSTINCSIGN` reader - Destination Address Increment Sign
pub type DstincsignR = crate::BitReader;
///Field `DSTINCSIGN` writer - Destination Address Increment Sign
pub type DstincsignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 16:17 - Arbitration Slot Number Select
    #[inline(always)]
    pub fn arbslots(&self) -> ArbslotsR {
        ArbslotsR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 20 - Source Address Increment Sign
    #[inline(always)]
    pub fn srcincsign(&self) -> SrcincsignR {
        SrcincsignR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Destination Address Increment Sign
    #[inline(always)]
    pub fn dstincsign(&self) -> DstincsignR {
        DstincsignR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_CFG")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
impl W {
    ///Bits 16:17 - Arbitration Slot Number Select
    #[inline(always)]
    #[must_use]
    pub fn arbslots(&mut self) -> ArbslotsW<CH1_CFGrs> {
        ArbslotsW::new(self, 16)
    }
    ///Bit 20 - Source Address Increment Sign
    #[inline(always)]
    #[must_use]
    pub fn srcincsign(&mut self) -> SrcincsignW<CH1_CFGrs> {
        SrcincsignW::new(self, 20)
    }
    ///Bit 21 - Destination Address Increment Sign
    #[inline(always)]
    #[must_use]
    pub fn dstincsign(&mut self) -> DstincsignW<CH1_CFGrs> {
        DstincsignW::new(self, 21)
    }
}
///Channel Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH1_CFGrs;
impl crate::RegisterSpec for CH1_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`ch1_cfg::R`](R) reader structure
impl crate::Readable for CH1_CFGrs {}
///`write(|w| ..)` method takes [`ch1_cfg::W`](W) writer structure
impl crate::Writable for CH1_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH1_CFG to value 0
impl crate::Resettable for CH1_CFGrs {
    const RESET_VALUE: u32 = 0;
}
