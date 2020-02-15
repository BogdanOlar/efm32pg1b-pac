#[doc = "Reader of register ROUTELOC1"]
pub type R = crate::R<u32, super::ROUTELOC1>;
#[doc = "Writer for register ROUTELOC1"]
pub type W = crate::W<u32, super::ROUTELOC1>;
#[doc = "Register ROUTELOC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH4LOC_A {
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
}
impl From<CH4LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH4LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CH4LOC`"]
pub type CH4LOC_R = crate::R<u8, CH4LOC_A>;
impl CH4LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH4LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH4LOC_A::LOC0),
            1 => Val(CH4LOC_A::LOC1),
            2 => Val(CH4LOC_A::LOC2),
            3 => Val(CH4LOC_A::LOC3),
            4 => Val(CH4LOC_A::LOC4),
            5 => Val(CH4LOC_A::LOC5),
            6 => Val(CH4LOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH4LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH4LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH4LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH4LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH4LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH4LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH4LOC_A::LOC6
    }
}
#[doc = "Write proxy for field `CH4LOC`"]
pub struct CH4LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH5LOC_A {
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
}
impl From<CH5LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH5LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CH5LOC`"]
pub type CH5LOC_R = crate::R<u8, CH5LOC_A>;
impl CH5LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH5LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH5LOC_A::LOC0),
            1 => Val(CH5LOC_A::LOC1),
            2 => Val(CH5LOC_A::LOC2),
            3 => Val(CH5LOC_A::LOC3),
            4 => Val(CH5LOC_A::LOC4),
            5 => Val(CH5LOC_A::LOC5),
            6 => Val(CH5LOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH5LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH5LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH5LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH5LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH5LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH5LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH5LOC_A::LOC6
    }
}
#[doc = "Write proxy for field `CH5LOC`"]
pub struct CH5LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH6LOC_A {
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
    #[doc = "17: Location 17"]
    LOC17 = 17,
}
impl From<CH6LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH6LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CH6LOC`"]
pub type CH6LOC_R = crate::R<u8, CH6LOC_A>;
impl CH6LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH6LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH6LOC_A::LOC0),
            1 => Val(CH6LOC_A::LOC1),
            2 => Val(CH6LOC_A::LOC2),
            3 => Val(CH6LOC_A::LOC3),
            4 => Val(CH6LOC_A::LOC4),
            5 => Val(CH6LOC_A::LOC5),
            6 => Val(CH6LOC_A::LOC6),
            7 => Val(CH6LOC_A::LOC7),
            8 => Val(CH6LOC_A::LOC8),
            9 => Val(CH6LOC_A::LOC9),
            10 => Val(CH6LOC_A::LOC10),
            11 => Val(CH6LOC_A::LOC11),
            12 => Val(CH6LOC_A::LOC12),
            13 => Val(CH6LOC_A::LOC13),
            14 => Val(CH6LOC_A::LOC14),
            15 => Val(CH6LOC_A::LOC15),
            16 => Val(CH6LOC_A::LOC16),
            17 => Val(CH6LOC_A::LOC17),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH6LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH6LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH6LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH6LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH6LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH6LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH6LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH6LOC_A::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH6LOC_A::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH6LOC_A::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH6LOC_A::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CH6LOC_A::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CH6LOC_A::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CH6LOC_A::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CH6LOC_A::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CH6LOC_A::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CH6LOC_A::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CH6LOC_A::LOC17
    }
}
#[doc = "Write proxy for field `CH6LOC`"]
pub struct CH6LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC17)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH7LOC_A {
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
impl From<CH7LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH7LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CH7LOC`"]
pub type CH7LOC_R = crate::R<u8, CH7LOC_A>;
impl CH7LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH7LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH7LOC_A::LOC0),
            1 => Val(CH7LOC_A::LOC1),
            2 => Val(CH7LOC_A::LOC2),
            3 => Val(CH7LOC_A::LOC3),
            4 => Val(CH7LOC_A::LOC4),
            5 => Val(CH7LOC_A::LOC5),
            6 => Val(CH7LOC_A::LOC6),
            7 => Val(CH7LOC_A::LOC7),
            8 => Val(CH7LOC_A::LOC8),
            9 => Val(CH7LOC_A::LOC9),
            10 => Val(CH7LOC_A::LOC10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH7LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH7LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH7LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH7LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH7LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH7LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH7LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH7LOC_A::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH7LOC_A::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH7LOC_A::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH7LOC_A::LOC10
    }
}
#[doc = "Write proxy for field `CH7LOC`"]
pub struct CH7LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&self) -> CH4LOC_R {
        CH4LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&self) -> CH5LOC_R {
        CH5LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&self) -> CH6LOC_R {
        CH6LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&self) -> CH7LOC_R {
        CH7LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&mut self) -> CH4LOC_W {
        CH4LOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&mut self) -> CH5LOC_W {
        CH5LOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&mut self) -> CH6LOC_W {
        CH6LOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&mut self) -> CH7LOC_W {
        CH7LOC_W { w: self }
    }
}
