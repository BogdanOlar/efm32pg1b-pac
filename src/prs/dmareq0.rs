///Register `DMAREQ0` reader
pub type R = crate::R<DMAREQ0rs>;
///Register `DMAREQ0` writer
pub type W = crate::W<DMAREQ0rs>;
///DMA Request 0 PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL {
    ///0: PRS Channel 0 selected
    Prsch0 = 0,
    ///1: PRS Channel 1 selected
    Prsch1 = 1,
    ///2: PRS Channel 2 selected
    Prsch2 = 2,
    ///3: PRS Channel 3 selected
    Prsch3 = 3,
    ///4: PRS Channel 4 selected
    Prsch4 = 4,
    ///5: PRS Channel 5 selected
    Prsch5 = 5,
    ///6: PRS Channel 6 selected
    Prsch6 = 6,
    ///7: PRS Channel 7 selected
    Prsch7 = 7,
    ///8: PRS Channel 8 selected
    Prsch8 = 8,
    ///9: PRS Channel 9 selected
    Prsch9 = 9,
    ///10: PRS Channel 10 selected
    Prsch10 = 10,
    ///11: PRS Channel 11 selected
    Prsch11 = 11,
}
impl From<PRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSSEL {}
///Field `PRSSEL` reader - DMA Request 0 PRS Channel Select
pub type PrsselR = crate::FieldReader<PRSSEL>;
impl PrsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL> {
        match self.bits {
            0 => Some(PRSSEL::Prsch0),
            1 => Some(PRSSEL::Prsch1),
            2 => Some(PRSSEL::Prsch2),
            3 => Some(PRSSEL::Prsch3),
            4 => Some(PRSSEL::Prsch4),
            5 => Some(PRSSEL::Prsch5),
            6 => Some(PRSSEL::Prsch6),
            7 => Some(PRSSEL::Prsch7),
            8 => Some(PRSSEL::Prsch8),
            9 => Some(PRSSEL::Prsch9),
            10 => Some(PRSSEL::Prsch10),
            11 => Some(PRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL::Prsch0
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL::Prsch1
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL::Prsch2
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL::Prsch3
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL::Prsch4
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL::Prsch5
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL::Prsch6
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL::Prsch7
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL::Prsch8
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL::Prsch9
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL::Prsch10
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL::Prsch11
    }
}
///Field `PRSSEL` writer - DMA Request 0 PRS Channel Select
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSEL>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch11)
    }
}
impl R {
    ///Bits 6:9 - DMA Request 0 PRS Channel Select
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAREQ0")
            .field("prssel", &self.prssel())
            .finish()
    }
}
impl W {
    ///Bits 6:9 - DMA Request 0 PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<DMAREQ0rs> {
        PrsselW::new(self, 6)
    }
}
///DMA Request 0 Register
///
///You can [`read`](crate::Reg::read) this register and get [`dmareq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DMAREQ0rs;
impl crate::RegisterSpec for DMAREQ0rs {
    type Ux = u32;
}
///`read()` method returns [`dmareq0::R`](R) reader structure
impl crate::Readable for DMAREQ0rs {}
///`write(|w| ..)` method takes [`dmareq0::W`](W) writer structure
impl crate::Writable for DMAREQ0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMAREQ0 to value 0
impl crate::Resettable for DMAREQ0rs {
    const RESET_VALUE: u32 = 0;
}
