///Register `TIMING` reader
pub type R = crate::R<TIMINGrs>;
///Register `TIMING` writer
pub type W = crate::W<TIMINGrs>;
///TX Frame Start Delay
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXDELAY {
    ///0: Disable - TXDELAY in USARTn_CTRL can be used for legacy
    Disable = 0,
    ///1: Start of transmission is delayed for 1 baud-times
    One = 1,
    ///2: Start of transmission is delayed for 2 baud-times
    Two = 2,
    ///3: Start of transmission is delayed for 3 baud-times
    Three = 3,
    ///4: Start of transmission is delayed for 7 baud-times
    Seven = 4,
    ///5: Start of transmission is delayed for TCMPVAL0 baud-times
    Tcmp0 = 5,
    ///6: Start of transmission is delayed for TCMPVAL1 baud-times
    Tcmp1 = 6,
    ///7: Start of transmission is delayed for TCMPVAL2 baud-times
    Tcmp2 = 7,
}
impl From<TXDELAY> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXDELAY {
    type Ux = u8;
}
impl crate::IsEnum for TXDELAY {}
///Field `TXDELAY` reader - TX Frame Start Delay
pub type TxdelayR = crate::FieldReader<TXDELAY>;
impl TxdelayR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDELAY {
        match self.bits {
            0 => TXDELAY::Disable,
            1 => TXDELAY::One,
            2 => TXDELAY::Two,
            3 => TXDELAY::Three,
            4 => TXDELAY::Seven,
            5 => TXDELAY::Tcmp0,
            6 => TXDELAY::Tcmp1,
            7 => TXDELAY::Tcmp2,
            _ => unreachable!(),
        }
    }
    ///Disable - TXDELAY in USARTn_CTRL can be used for legacy
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXDELAY::Disable
    }
    ///Start of transmission is delayed for 1 baud-times
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TXDELAY::One
    }
    ///Start of transmission is delayed for 2 baud-times
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == TXDELAY::Two
    }
    ///Start of transmission is delayed for 3 baud-times
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == TXDELAY::Three
    }
    ///Start of transmission is delayed for 7 baud-times
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == TXDELAY::Seven
    }
    ///Start of transmission is delayed for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == TXDELAY::Tcmp0
    }
    ///Start of transmission is delayed for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == TXDELAY::Tcmp1
    }
    ///Start of transmission is delayed for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TXDELAY::Tcmp2
    }
}
///Field `TXDELAY` writer - TX Frame Start Delay
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3, TXDELAY, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disable - TXDELAY in USARTn_CTRL can be used for legacy
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Disable)
    }
    ///Start of transmission is delayed for 1 baud-times
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::One)
    }
    ///Start of transmission is delayed for 2 baud-times
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Two)
    }
    ///Start of transmission is delayed for 3 baud-times
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Three)
    }
    ///Start of transmission is delayed for 7 baud-times
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Seven)
    }
    ///Start of transmission is delayed for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Tcmp0)
    }
    ///Start of transmission is delayed for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Tcmp1)
    }
    ///Start of transmission is delayed for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Tcmp2)
    }
}
///Chip Select Setup
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSSETUP {
    ///0: CS is not asserted before start of transmission
    Zero = 0,
    ///1: CS is asserted for 1 baud-times before start of transmission
    One = 1,
    ///2: CS is asserted for 2 baud-times before start of transmission
    Two = 2,
    ///3: CS is asserted for 3 baud-times before start of transmission
    Three = 3,
    ///4: CS is asserted for 7 baud-times before start of transmission
    Seven = 4,
    ///5: CS is asserted before the start of transmission for TCMPVAL0 baud-times
    Tcmp0 = 5,
    ///6: CS is asserted before the start of transmission for TCMPVAL1 baud-times
    Tcmp1 = 6,
    ///7: CS is asserted before the start of transmission for TCMPVAL2 baud-times
    Tcmp2 = 7,
}
impl From<CSSETUP> for u8 {
    #[inline(always)]
    fn from(variant: CSSETUP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSSETUP {
    type Ux = u8;
}
impl crate::IsEnum for CSSETUP {}
///Field `CSSETUP` reader - Chip Select Setup
pub type CssetupR = crate::FieldReader<CSSETUP>;
impl CssetupR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSETUP {
        match self.bits {
            0 => CSSETUP::Zero,
            1 => CSSETUP::One,
            2 => CSSETUP::Two,
            3 => CSSETUP::Three,
            4 => CSSETUP::Seven,
            5 => CSSETUP::Tcmp0,
            6 => CSSETUP::Tcmp1,
            7 => CSSETUP::Tcmp2,
            _ => unreachable!(),
        }
    }
    ///CS is not asserted before start of transmission
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSSETUP::Zero
    }
    ///CS is asserted for 1 baud-times before start of transmission
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSSETUP::One
    }
    ///CS is asserted for 2 baud-times before start of transmission
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSSETUP::Two
    }
    ///CS is asserted for 3 baud-times before start of transmission
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSSETUP::Three
    }
    ///CS is asserted for 7 baud-times before start of transmission
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSSETUP::Seven
    }
    ///CS is asserted before the start of transmission for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSSETUP::Tcmp0
    }
    ///CS is asserted before the start of transmission for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSSETUP::Tcmp1
    }
    ///CS is asserted before the start of transmission for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSSETUP::Tcmp2
    }
}
///Field `CSSETUP` writer - Chip Select Setup
pub type CssetupW<'a, REG> = crate::FieldWriter<'a, REG, 3, CSSETUP, crate::Safe>;
impl<'a, REG> CssetupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CS is not asserted before start of transmission
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Zero)
    }
    ///CS is asserted for 1 baud-times before start of transmission
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::One)
    }
    ///CS is asserted for 2 baud-times before start of transmission
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Two)
    }
    ///CS is asserted for 3 baud-times before start of transmission
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Three)
    }
    ///CS is asserted for 7 baud-times before start of transmission
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Seven)
    }
    ///CS is asserted before the start of transmission for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Tcmp0)
    }
    ///CS is asserted before the start of transmission for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Tcmp1)
    }
    ///CS is asserted before the start of transmission for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Tcmp2)
    }
}
///Inter-character Spacing
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICS {
    ///0: There is no space between charcters
    Zero = 0,
    ///1: Create a space of 1 baud-times before start of transmission
    One = 1,
    ///2: Create a space of 2 baud-times before start of transmission
    Two = 2,
    ///3: Create a space of 3 baud-times before start of transmission
    Three = 3,
    ///4: Create a space of 7 baud-times before start of transmission
    Seven = 4,
    ///5: Create a space of before the start of transmission for TCMPVAL0 baud-times
    Tcmp0 = 5,
    ///6: Create a space of before the start of transmission for TCMPVAL1 baud-times
    Tcmp1 = 6,
    ///7: Create a space of before the start of transmission for TCMPVAL2 baud-times
    Tcmp2 = 7,
}
impl From<ICS> for u8 {
    #[inline(always)]
    fn from(variant: ICS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICS {
    type Ux = u8;
}
impl crate::IsEnum for ICS {}
///Field `ICS` reader - Inter-character Spacing
pub type IcsR = crate::FieldReader<ICS>;
impl IcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICS {
        match self.bits {
            0 => ICS::Zero,
            1 => ICS::One,
            2 => ICS::Two,
            3 => ICS::Three,
            4 => ICS::Seven,
            5 => ICS::Tcmp0,
            6 => ICS::Tcmp1,
            7 => ICS::Tcmp2,
            _ => unreachable!(),
        }
    }
    ///There is no space between charcters
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICS::Zero
    }
    ///Create a space of 1 baud-times before start of transmission
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICS::One
    }
    ///Create a space of 2 baud-times before start of transmission
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ICS::Two
    }
    ///Create a space of 3 baud-times before start of transmission
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == ICS::Three
    }
    ///Create a space of 7 baud-times before start of transmission
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == ICS::Seven
    }
    ///Create a space of before the start of transmission for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == ICS::Tcmp0
    }
    ///Create a space of before the start of transmission for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == ICS::Tcmp1
    }
    ///Create a space of before the start of transmission for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == ICS::Tcmp2
    }
}
///Field `ICS` writer - Inter-character Spacing
pub type IcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, ICS, crate::Safe>;
impl<'a, REG> IcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///There is no space between charcters
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Zero)
    }
    ///Create a space of 1 baud-times before start of transmission
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::One)
    }
    ///Create a space of 2 baud-times before start of transmission
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Two)
    }
    ///Create a space of 3 baud-times before start of transmission
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Three)
    }
    ///Create a space of 7 baud-times before start of transmission
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Seven)
    }
    ///Create a space of before the start of transmission for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Tcmp0)
    }
    ///Create a space of before the start of transmission for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Tcmp1)
    }
    ///Create a space of before the start of transmission for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Tcmp2)
    }
}
///Chip Select Hold
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSHOLD {
    ///0: Disable CS being asserted after the end of transmission
    Zero = 0,
    ///1: CS is asserted for 1 baud-times after the end of transmission
    One = 1,
    ///2: CS is asserted for 2 baud-times after the end of transmission
    Two = 2,
    ///3: CS is asserted for 3 baud-times after the end of transmission
    Three = 3,
    ///4: CS is asserted for 7 baud-times after the end of transmission
    Seven = 4,
    ///5: CS is asserted after the end of transmission for TCMPVAL0 baud-times
    Tcmp0 = 5,
    ///6: CS is asserted after the end of transmission for TCMPVAL1 baud-times
    Tcmp1 = 6,
    ///7: CS is asserted after the end of transmission for TCMPVAL2 baud-times
    Tcmp2 = 7,
}
impl From<CSHOLD> for u8 {
    #[inline(always)]
    fn from(variant: CSHOLD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSHOLD {
    type Ux = u8;
}
impl crate::IsEnum for CSHOLD {}
///Field `CSHOLD` reader - Chip Select Hold
pub type CsholdR = crate::FieldReader<CSHOLD>;
impl CsholdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSHOLD {
        match self.bits {
            0 => CSHOLD::Zero,
            1 => CSHOLD::One,
            2 => CSHOLD::Two,
            3 => CSHOLD::Three,
            4 => CSHOLD::Seven,
            5 => CSHOLD::Tcmp0,
            6 => CSHOLD::Tcmp1,
            7 => CSHOLD::Tcmp2,
            _ => unreachable!(),
        }
    }
    ///Disable CS being asserted after the end of transmission
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSHOLD::Zero
    }
    ///CS is asserted for 1 baud-times after the end of transmission
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSHOLD::One
    }
    ///CS is asserted for 2 baud-times after the end of transmission
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSHOLD::Two
    }
    ///CS is asserted for 3 baud-times after the end of transmission
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSHOLD::Three
    }
    ///CS is asserted for 7 baud-times after the end of transmission
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSHOLD::Seven
    }
    ///CS is asserted after the end of transmission for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSHOLD::Tcmp0
    }
    ///CS is asserted after the end of transmission for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSHOLD::Tcmp1
    }
    ///CS is asserted after the end of transmission for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSHOLD::Tcmp2
    }
}
///Field `CSHOLD` writer - Chip Select Hold
pub type CsholdW<'a, REG> = crate::FieldWriter<'a, REG, 3, CSHOLD, crate::Safe>;
impl<'a, REG> CsholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disable CS being asserted after the end of transmission
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Zero)
    }
    ///CS is asserted for 1 baud-times after the end of transmission
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::One)
    }
    ///CS is asserted for 2 baud-times after the end of transmission
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Two)
    }
    ///CS is asserted for 3 baud-times after the end of transmission
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Three)
    }
    ///CS is asserted for 7 baud-times after the end of transmission
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Seven)
    }
    ///CS is asserted after the end of transmission for TCMPVAL0 baud-times
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Tcmp0)
    }
    ///CS is asserted after the end of transmission for TCMPVAL1 baud-times
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Tcmp1)
    }
    ///CS is asserted after the end of transmission for TCMPVAL2 baud-times
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Tcmp2)
    }
}
impl R {
    ///Bits 16:18 - TX Frame Start Delay
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Chip Select Setup
    #[inline(always)]
    pub fn cssetup(&self) -> CssetupR {
        CssetupR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Inter-character Spacing
    #[inline(always)]
    pub fn ics(&self) -> IcsR {
        IcsR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Chip Select Hold
    #[inline(always)]
    pub fn cshold(&self) -> CsholdR {
        CsholdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMING")
            .field("txdelay", &self.txdelay())
            .field("cssetup", &self.cssetup())
            .field("ics", &self.ics())
            .field("cshold", &self.cshold())
            .finish()
    }
}
impl W {
    ///Bits 16:18 - TX Frame Start Delay
    #[inline(always)]
    #[must_use]
    pub fn txdelay(&mut self) -> TxdelayW<TIMINGrs> {
        TxdelayW::new(self, 16)
    }
    ///Bits 20:22 - Chip Select Setup
    #[inline(always)]
    #[must_use]
    pub fn cssetup(&mut self) -> CssetupW<TIMINGrs> {
        CssetupW::new(self, 20)
    }
    ///Bits 24:26 - Inter-character Spacing
    #[inline(always)]
    #[must_use]
    pub fn ics(&mut self) -> IcsW<TIMINGrs> {
        IcsW::new(self, 24)
    }
    ///Bits 28:30 - Chip Select Hold
    #[inline(always)]
    #[must_use]
    pub fn cshold(&mut self) -> CsholdW<TIMINGrs> {
        CsholdW::new(self, 28)
    }
}
///Timing Register
///
///You can [`read`](crate::Reg::read) this register and get [`timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TIMINGrs;
impl crate::RegisterSpec for TIMINGrs {
    type Ux = u32;
}
///`read()` method returns [`timing::R`](R) reader structure
impl crate::Readable for TIMINGrs {}
///`write(|w| ..)` method takes [`timing::W`](W) writer structure
impl crate::Writable for TIMINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMING to value 0
impl crate::Resettable for TIMINGrs {
    const RESET_VALUE: u32 = 0;
}
