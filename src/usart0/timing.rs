#[doc = "Register `TIMING` reader"]
pub type R = crate::R<TIMINGrs>;
#[doc = "Register `TIMING` writer"]
pub type W = crate::W<TIMINGrs>;
#[doc = "TX Frame Start Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXDELAY {
    #[doc = "0: Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    Disable = 0,
    #[doc = "1: Start of transmission is delayed for 1 baud-times"]
    One = 1,
    #[doc = "2: Start of transmission is delayed for 2 baud-times"]
    Two = 2,
    #[doc = "3: Start of transmission is delayed for 3 baud-times"]
    Three = 3,
    #[doc = "4: Start of transmission is delayed for 7 baud-times"]
    Seven = 4,
    #[doc = "5: Start of transmission is delayed for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: Start of transmission is delayed for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: Start of transmission is delayed for TCMPVAL2 baud-times"]
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
#[doc = "Field `TXDELAY` reader - TX Frame Start Delay"]
pub type TxdelayR = crate::FieldReader<TXDELAY>;
impl TxdelayR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXDELAY::Disable
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TXDELAY::One
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == TXDELAY::Two
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == TXDELAY::Three
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == TXDELAY::Seven
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == TXDELAY::Tcmp0
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == TXDELAY::Tcmp1
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TXDELAY::Tcmp2
    }
}
#[doc = "Field `TXDELAY` writer - TX Frame Start Delay"]
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3, TXDELAY, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Disable)
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::One)
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Two)
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Three)
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Seven)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Tcmp0)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Tcmp1)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Tcmp2)
    }
}
#[doc = "Chip Select Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSSETUP {
    #[doc = "0: CS is not asserted before start of transmission"]
    Zero = 0,
    #[doc = "1: CS is asserted for 1 baud-times before start of transmission"]
    One = 1,
    #[doc = "2: CS is asserted for 2 baud-times before start of transmission"]
    Two = 2,
    #[doc = "3: CS is asserted for 3 baud-times before start of transmission"]
    Three = 3,
    #[doc = "4: CS is asserted for 7 baud-times before start of transmission"]
    Seven = 4,
    #[doc = "5: CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
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
#[doc = "Field `CSSETUP` reader - Chip Select Setup"]
pub type CssetupR = crate::FieldReader<CSSETUP>;
impl CssetupR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSSETUP::Zero
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSSETUP::One
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSSETUP::Two
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSSETUP::Three
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSSETUP::Seven
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSSETUP::Tcmp0
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSSETUP::Tcmp1
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSSETUP::Tcmp2
    }
}
#[doc = "Field `CSSETUP` writer - Chip Select Setup"]
pub type CssetupW<'a, REG> = crate::FieldWriter<'a, REG, 3, CSSETUP, crate::Safe>;
impl<'a, REG> CssetupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Zero)
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::One)
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Two)
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Three)
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Seven)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Tcmp0)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Tcmp1)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP::Tcmp2)
    }
}
#[doc = "Inter-character Spacing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICS {
    #[doc = "0: There is no space between charcters"]
    Zero = 0,
    #[doc = "1: Create a space of 1 baud-times before start of transmission"]
    One = 1,
    #[doc = "2: Create a space of 2 baud-times before start of transmission"]
    Two = 2,
    #[doc = "3: Create a space of 3 baud-times before start of transmission"]
    Three = 3,
    #[doc = "4: Create a space of 7 baud-times before start of transmission"]
    Seven = 4,
    #[doc = "5: Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: Create a space of before the start of transmission for TCMPVAL2 baud-times"]
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
#[doc = "Field `ICS` reader - Inter-character Spacing"]
pub type IcsR = crate::FieldReader<ICS>;
impl IcsR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICS::Zero
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICS::One
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ICS::Two
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == ICS::Three
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == ICS::Seven
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == ICS::Tcmp0
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == ICS::Tcmp1
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == ICS::Tcmp2
    }
}
#[doc = "Field `ICS` writer - Inter-character Spacing"]
pub type IcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, ICS, crate::Safe>;
impl<'a, REG> IcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Zero)
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::One)
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Two)
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Three)
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Seven)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Tcmp0)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Tcmp1)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(ICS::Tcmp2)
    }
}
#[doc = "Chip Select Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSHOLD {
    #[doc = "0: Disable CS being asserted after the end of transmission"]
    Zero = 0,
    #[doc = "1: CS is asserted for 1 baud-times after the end of transmission"]
    One = 1,
    #[doc = "2: CS is asserted for 2 baud-times after the end of transmission"]
    Two = 2,
    #[doc = "3: CS is asserted for 3 baud-times after the end of transmission"]
    Three = 3,
    #[doc = "4: CS is asserted for 7 baud-times after the end of transmission"]
    Seven = 4,
    #[doc = "5: CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
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
#[doc = "Field `CSHOLD` reader - Chip Select Hold"]
pub type CsholdR = crate::FieldReader<CSHOLD>;
impl CsholdR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSHOLD::Zero
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSHOLD::One
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSHOLD::Two
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSHOLD::Three
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSHOLD::Seven
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSHOLD::Tcmp0
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSHOLD::Tcmp1
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSHOLD::Tcmp2
    }
}
#[doc = "Field `CSHOLD` writer - Chip Select Hold"]
pub type CsholdW<'a, REG> = crate::FieldWriter<'a, REG, 3, CSHOLD, crate::Safe>;
impl<'a, REG> CsholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Zero)
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::One)
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Two)
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Three)
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Seven)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Tcmp0)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Tcmp1)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD::Tcmp2)
    }
}
impl R {
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&self) -> CssetupR {
        CssetupR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline(always)]
    pub fn ics(&self) -> IcsR {
        IcsR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&self) -> CsholdR {
        CsholdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn txdelay(&mut self) -> TxdelayW<TIMINGrs> {
        TxdelayW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    #[must_use]
    pub fn cssetup(&mut self) -> CssetupW<TIMINGrs> {
        CssetupW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline(always)]
    #[must_use]
    pub fn ics(&mut self) -> IcsW<TIMINGrs> {
        IcsW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    #[must_use]
    pub fn cshold(&mut self) -> CsholdW<TIMINGrs> {
        CsholdW::new(self, 28)
    }
}
#[doc = "Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMINGrs;
impl crate::RegisterSpec for TIMINGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing::R`](R) reader structure"]
impl crate::Readable for TIMINGrs {}
#[doc = "`write(|w| ..)` method takes [`timing::W`](W) writer structure"]
impl crate::Writable for TIMINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMINGrs {
    const RESET_VALUE: u32 = 0;
}
