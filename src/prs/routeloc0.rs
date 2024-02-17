#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<ROUTELOC0rs>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<ROUTELOC0rs>;
#[doc = "Field `CH0LOC` reader - I/O Location"]
pub type CH0LOC_R = crate::FieldReader<CH0LOC>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH0LOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
}
impl From<CH0LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH0LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH0LOC {
    type Ux = u8;
}
impl CH0LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH0LOC> {
        match self.bits {
            0 => Some(CH0LOC::Loc0),
            1 => Some(CH0LOC::Loc1),
            2 => Some(CH0LOC::Loc2),
            3 => Some(CH0LOC::Loc3),
            4 => Some(CH0LOC::Loc4),
            5 => Some(CH0LOC::Loc5),
            6 => Some(CH0LOC::Loc6),
            7 => Some(CH0LOC::Loc7),
            8 => Some(CH0LOC::Loc8),
            9 => Some(CH0LOC::Loc9),
            10 => Some(CH0LOC::Loc10),
            11 => Some(CH0LOC::Loc11),
            12 => Some(CH0LOC::Loc12),
            13 => Some(CH0LOC::Loc13),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH0LOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH0LOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH0LOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH0LOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH0LOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH0LOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH0LOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH0LOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH0LOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH0LOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH0LOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CH0LOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CH0LOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CH0LOC::Loc13
    }
}
#[doc = "Field `CH0LOC` writer - I/O Location"]
pub type CH0LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH0LOC>;
impl<'a, REG> CH0LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CH0LOC::Loc13)
    }
}
#[doc = "Field `CH1LOC` reader - I/O Location"]
pub type CH1LOC_R = crate::FieldReader<CH1LOC>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH1LOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
}
impl From<CH1LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH1LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH1LOC {
    type Ux = u8;
}
impl CH1LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH1LOC> {
        match self.bits {
            0 => Some(CH1LOC::Loc0),
            1 => Some(CH1LOC::Loc1),
            2 => Some(CH1LOC::Loc2),
            3 => Some(CH1LOC::Loc3),
            4 => Some(CH1LOC::Loc4),
            5 => Some(CH1LOC::Loc5),
            6 => Some(CH1LOC::Loc6),
            7 => Some(CH1LOC::Loc7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH1LOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH1LOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH1LOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH1LOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH1LOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH1LOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH1LOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH1LOC::Loc7
    }
}
#[doc = "Field `CH1LOC` writer - I/O Location"]
pub type CH1LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH1LOC>;
impl<'a, REG> CH1LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH1LOC::Loc7)
    }
}
#[doc = "Field `CH2LOC` reader - I/O Location"]
pub type CH2LOC_R = crate::FieldReader<CH2LOC>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH2LOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
}
impl From<CH2LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH2LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH2LOC {
    type Ux = u8;
}
impl CH2LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH2LOC> {
        match self.bits {
            0 => Some(CH2LOC::Loc0),
            1 => Some(CH2LOC::Loc1),
            2 => Some(CH2LOC::Loc2),
            3 => Some(CH2LOC::Loc3),
            4 => Some(CH2LOC::Loc4),
            5 => Some(CH2LOC::Loc5),
            6 => Some(CH2LOC::Loc6),
            7 => Some(CH2LOC::Loc7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH2LOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH2LOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH2LOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH2LOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH2LOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH2LOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH2LOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH2LOC::Loc7
    }
}
#[doc = "Field `CH2LOC` writer - I/O Location"]
pub type CH2LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH2LOC>;
impl<'a, REG> CH2LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH2LOC::Loc7)
    }
}
#[doc = "Field `CH3LOC` reader - I/O Location"]
pub type CH3LOC_R = crate::FieldReader<CH3LOC>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH3LOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
}
impl From<CH3LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH3LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH3LOC {
    type Ux = u8;
}
impl CH3LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH3LOC> {
        match self.bits {
            0 => Some(CH3LOC::Loc0),
            1 => Some(CH3LOC::Loc1),
            2 => Some(CH3LOC::Loc2),
            3 => Some(CH3LOC::Loc3),
            4 => Some(CH3LOC::Loc4),
            5 => Some(CH3LOC::Loc5),
            6 => Some(CH3LOC::Loc6),
            7 => Some(CH3LOC::Loc7),
            8 => Some(CH3LOC::Loc8),
            9 => Some(CH3LOC::Loc9),
            10 => Some(CH3LOC::Loc10),
            11 => Some(CH3LOC::Loc11),
            12 => Some(CH3LOC::Loc12),
            13 => Some(CH3LOC::Loc13),
            14 => Some(CH3LOC::Loc14),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH3LOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH3LOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH3LOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH3LOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH3LOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH3LOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH3LOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH3LOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH3LOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH3LOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH3LOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CH3LOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CH3LOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CH3LOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CH3LOC::Loc14
    }
}
#[doc = "Field `CH3LOC` writer - I/O Location"]
pub type CH3LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH3LOC>;
impl<'a, REG> CH3LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CH3LOC::Loc14)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch0loc(&self) -> CH0LOC_R {
        CH0LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch1loc(&self) -> CH1LOC_R {
        CH1LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch2loc(&self) -> CH2LOC_R {
        CH2LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch3loc(&self) -> CH3LOC_R {
        CH3LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch0loc(&mut self) -> CH0LOC_W<ROUTELOC0rs> {
        CH0LOC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch1loc(&mut self) -> CH1LOC_W<ROUTELOC0rs> {
        CH1LOC_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch2loc(&mut self) -> CH2LOC_W<ROUTELOC0rs> {
        CH2LOC_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch3loc(&mut self) -> CH3LOC_W<ROUTELOC0rs> {
        CH3LOC_W::new(self, 24)
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
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC0rs;
impl crate::RegisterSpec for ROUTELOC0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc0::R`](R) reader structure"]
impl crate::Readable for ROUTELOC0rs {}
#[doc = "`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure"]
impl crate::Writable for ROUTELOC0rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0rs {
    const RESET_VALUE: u32 = 0;
}
