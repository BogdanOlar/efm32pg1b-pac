#[doc = "Register `ROUTELOC2` reader"]
pub type R = crate::R<ROUTELOC2_SPEC>;
#[doc = "Register `ROUTELOC2` writer"]
pub type W = crate::W<ROUTELOC2_SPEC>;
#[doc = "Field `CH8LOC` reader - I/O Location"]
pub type CH8LOC_R = crate::FieldReader<CH8LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH8LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
    #[doc = "8: Location 8"]
    LOC8 = 8,
    #[doc = "9: Location 9"]
    LOC9 = 9,
    #[doc = "10: Location 10"]
    LOC10 = 10,
}
impl From<CH8LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH8LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH8LOC_A {
    type Ux = u8;
}
impl CH8LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH8LOC_A> {
        match self.bits {
            0 => Some(CH8LOC_A::LOC0),
            1 => Some(CH8LOC_A::LOC1),
            2 => Some(CH8LOC_A::LOC2),
            3 => Some(CH8LOC_A::LOC3),
            4 => Some(CH8LOC_A::LOC4),
            5 => Some(CH8LOC_A::LOC5),
            6 => Some(CH8LOC_A::LOC6),
            7 => Some(CH8LOC_A::LOC7),
            8 => Some(CH8LOC_A::LOC8),
            9 => Some(CH8LOC_A::LOC9),
            10 => Some(CH8LOC_A::LOC10),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH8LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH8LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH8LOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH8LOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH8LOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH8LOC_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH8LOC_A::LOC6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH8LOC_A::LOC7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH8LOC_A::LOC8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH8LOC_A::LOC9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH8LOC_A::LOC10
    }
}
#[doc = "Field `CH8LOC` writer - I/O Location"]
pub type CH8LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH8LOC_A>;
impl<'a, REG, const O: u8> CH8LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC10)
    }
}
#[doc = "Field `CH9LOC` reader - I/O Location"]
pub type CH9LOC_R = crate::FieldReader<CH9LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH9LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
    #[doc = "8: Location 8"]
    LOC8 = 8,
    #[doc = "9: Location 9"]
    LOC9 = 9,
    #[doc = "10: Location 10"]
    LOC10 = 10,
    #[doc = "11: Location 11"]
    LOC11 = 11,
    #[doc = "12: Location 12"]
    LOC12 = 12,
    #[doc = "13: Location 13"]
    LOC13 = 13,
    #[doc = "14: Location 14"]
    LOC14 = 14,
    #[doc = "15: Location 15"]
    LOC15 = 15,
    #[doc = "16: Location 16"]
    LOC16 = 16,
}
impl From<CH9LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH9LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH9LOC_A {
    type Ux = u8;
}
impl CH9LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH9LOC_A> {
        match self.bits {
            0 => Some(CH9LOC_A::LOC0),
            1 => Some(CH9LOC_A::LOC1),
            2 => Some(CH9LOC_A::LOC2),
            3 => Some(CH9LOC_A::LOC3),
            4 => Some(CH9LOC_A::LOC4),
            5 => Some(CH9LOC_A::LOC5),
            6 => Some(CH9LOC_A::LOC6),
            7 => Some(CH9LOC_A::LOC7),
            8 => Some(CH9LOC_A::LOC8),
            9 => Some(CH9LOC_A::LOC9),
            10 => Some(CH9LOC_A::LOC10),
            11 => Some(CH9LOC_A::LOC11),
            12 => Some(CH9LOC_A::LOC12),
            13 => Some(CH9LOC_A::LOC13),
            14 => Some(CH9LOC_A::LOC14),
            15 => Some(CH9LOC_A::LOC15),
            16 => Some(CH9LOC_A::LOC16),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH9LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH9LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH9LOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH9LOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH9LOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH9LOC_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH9LOC_A::LOC6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH9LOC_A::LOC7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH9LOC_A::LOC8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH9LOC_A::LOC9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH9LOC_A::LOC10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CH9LOC_A::LOC11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CH9LOC_A::LOC12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CH9LOC_A::LOC13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CH9LOC_A::LOC14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CH9LOC_A::LOC15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CH9LOC_A::LOC16
    }
}
#[doc = "Field `CH9LOC` writer - I/O Location"]
pub type CH9LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH9LOC_A>;
impl<'a, REG, const O: u8> CH9LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC16)
    }
}
#[doc = "Field `CH10LOC` reader - I/O Location"]
pub type CH10LOC_R = crate::FieldReader<CH10LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH10LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
}
impl From<CH10LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH10LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH10LOC_A {
    type Ux = u8;
}
impl CH10LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH10LOC_A> {
        match self.bits {
            0 => Some(CH10LOC_A::LOC0),
            1 => Some(CH10LOC_A::LOC1),
            2 => Some(CH10LOC_A::LOC2),
            3 => Some(CH10LOC_A::LOC3),
            4 => Some(CH10LOC_A::LOC4),
            5 => Some(CH10LOC_A::LOC5),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH10LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH10LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH10LOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH10LOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH10LOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH10LOC_A::LOC5
    }
}
#[doc = "Field `CH10LOC` writer - I/O Location"]
pub type CH10LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH10LOC_A>;
impl<'a, REG, const O: u8> CH10LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC5)
    }
}
#[doc = "Field `CH11LOC` reader - I/O Location"]
pub type CH11LOC_R = crate::FieldReader<CH11LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH11LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
}
impl From<CH11LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH11LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH11LOC_A {
    type Ux = u8;
}
impl CH11LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH11LOC_A> {
        match self.bits {
            0 => Some(CH11LOC_A::LOC0),
            1 => Some(CH11LOC_A::LOC1),
            2 => Some(CH11LOC_A::LOC2),
            3 => Some(CH11LOC_A::LOC3),
            4 => Some(CH11LOC_A::LOC4),
            5 => Some(CH11LOC_A::LOC5),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH11LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH11LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH11LOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH11LOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH11LOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH11LOC_A::LOC5
    }
}
#[doc = "Field `CH11LOC` writer - I/O Location"]
pub type CH11LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH11LOC_A>;
impl<'a, REG, const O: u8> CH11LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC5)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch8loc(&self) -> CH8LOC_R {
        CH8LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&self) -> CH9LOC_R {
        CH9LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&self) -> CH10LOC_R {
        CH10LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&self) -> CH11LOC_R {
        CH11LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC2")
            .field("ch8loc", &format_args!("{}", self.ch8loc().bits()))
            .field("ch9loc", &format_args!("{}", self.ch9loc().bits()))
            .field("ch10loc", &format_args!("{}", self.ch10loc().bits()))
            .field("ch11loc", &format_args!("{}", self.ch11loc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ROUTELOC2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch8loc(&mut self) -> CH8LOC_W<ROUTELOC2_SPEC, 0> {
        CH8LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch9loc(&mut self) -> CH9LOC_W<ROUTELOC2_SPEC, 8> {
        CH9LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch10loc(&mut self) -> CH10LOC_W<ROUTELOC2_SPEC, 16> {
        CH10LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch11loc(&mut self) -> CH11LOC_W<ROUTELOC2_SPEC, 24> {
        CH11LOC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC2_SPEC;
impl crate::RegisterSpec for ROUTELOC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc2::R`](R) reader structure"]
impl crate::Readable for ROUTELOC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc2::W`](W) writer structure"]
impl crate::Writable for ROUTELOC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC2 to value 0"]
impl crate::Resettable for ROUTELOC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
