// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod teste {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ANY: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ANY: u8 = 4;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ANY: [Any; 5] = [
  Any::NONE,
  Any::TypeA,
  Any::TypeB,
  Any::TypeC,
  Any::TypeD,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Any(pub u8);
#[allow(non_upper_case_globals)]
impl Any {
  pub const NONE: Self = Self(0);
  pub const TypeA: Self = Self(1);
  pub const TypeB: Self = Self(2);
  pub const TypeC: Self = Self(3);
  pub const TypeD: Self = Self(4);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 4;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::TypeA,
    Self::TypeB,
    Self::TypeC,
    Self::TypeD,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::TypeA => Some("TypeA"),
      Self::TypeB => Some("TypeB"),
      Self::TypeC => Some("TypeC"),
      Self::TypeD => Some("TypeD"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for Any {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Any {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Any {
    type Output = Any;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Any {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Any {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Any {}
pub struct AnyUnionTableOffset {}

pub enum TypeAOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TypeA<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TypeA<'a> {
  type Inner = TypeA<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TypeA<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TypeA { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TypeAArgs
  ) -> flatbuffers::WIPOffset<TypeA<'bldr>> {
    let mut builder = TypeABuilder::new(_fbb);
    builder.add_data(args.data);
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(TypeA::VT_DATA, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for TypeA<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct TypeAArgs {
    pub data: i32,
}
impl<'a> Default for TypeAArgs {
  #[inline]
  fn default() -> Self {
    TypeAArgs {
      data: 0,
    }
  }
}

pub struct TypeABuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TypeABuilder<'a, 'b> {
  #[inline]
  pub fn add_data(&mut self, data: i32) {
    self.fbb_.push_slot::<i32>(TypeA::VT_DATA, data, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TypeABuilder<'a, 'b> {
    let start = _fbb.start_table();
    TypeABuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TypeA<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TypeA<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TypeA");
      ds.field("data", &self.data());
      ds.finish()
  }
}
pub enum TypeBOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TypeB<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TypeB<'a> {
  type Inner = TypeB<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TypeB<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TypeB { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TypeBArgs
  ) -> flatbuffers::WIPOffset<TypeB<'bldr>> {
    let mut builder = TypeBBuilder::new(_fbb);
    builder.add_data(args.data);
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(TypeB::VT_DATA, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for TypeB<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct TypeBArgs {
    pub data: i32,
}
impl<'a> Default for TypeBArgs {
  #[inline]
  fn default() -> Self {
    TypeBArgs {
      data: 0,
    }
  }
}

pub struct TypeBBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TypeBBuilder<'a, 'b> {
  #[inline]
  pub fn add_data(&mut self, data: i32) {
    self.fbb_.push_slot::<i32>(TypeB::VT_DATA, data, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TypeBBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TypeBBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TypeB<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TypeB<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TypeB");
      ds.field("data", &self.data());
      ds.finish()
  }
}
pub enum TypeCOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TypeC<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TypeC<'a> {
  type Inner = TypeC<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TypeC<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TypeC { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TypeCArgs
  ) -> flatbuffers::WIPOffset<TypeC<'bldr>> {
    let mut builder = TypeCBuilder::new(_fbb);
    builder.add_data(args.data);
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(TypeC::VT_DATA, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for TypeC<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct TypeCArgs {
    pub data: i32,
}
impl<'a> Default for TypeCArgs {
  #[inline]
  fn default() -> Self {
    TypeCArgs {
      data: 0,
    }
  }
}

pub struct TypeCBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TypeCBuilder<'a, 'b> {
  #[inline]
  pub fn add_data(&mut self, data: i32) {
    self.fbb_.push_slot::<i32>(TypeC::VT_DATA, data, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TypeCBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TypeCBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TypeC<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TypeC<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TypeC");
      ds.field("data", &self.data());
      ds.finish()
  }
}
pub enum TypeDOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TypeD<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TypeD<'a> {
  type Inner = TypeD<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TypeD<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TypeD { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TypeDArgs
  ) -> flatbuffers::WIPOffset<TypeD<'bldr>> {
    let mut builder = TypeDBuilder::new(_fbb);
    builder.add_data(args.data);
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(TypeD::VT_DATA, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for TypeD<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct TypeDArgs {
    pub data: i32,
}
impl<'a> Default for TypeDArgs {
  #[inline]
  fn default() -> Self {
    TypeDArgs {
      data: 0,
    }
  }
}

pub struct TypeDBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TypeDBuilder<'a, 'b> {
  #[inline]
  pub fn add_data(&mut self, data: i32) {
    self.fbb_.push_slot::<i32>(TypeD::VT_DATA, data, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TypeDBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TypeDBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TypeD<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TypeD<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TypeD");
      ds.field("data", &self.data());
      ds.finish()
  }
}
pub enum BaseOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Base<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Base<'a> {
  type Inner = Base<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Base<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_NAME: flatbuffers::VOffsetT = 6;
  pub const VT_DEVICE_TYPE: flatbuffers::VOffsetT = 8;
  pub const VT_DEVICE: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Base { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args BaseArgs<'args>
  ) -> flatbuffers::WIPOffset<Base<'bldr>> {
    let mut builder = BaseBuilder::new(_fbb);
    if let Some(x) = args.device { builder.add_device(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_id(args.id);
    builder.add_device_type(args.device_type);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(Base::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Base::VT_NAME, None)}
  }
  #[inline]
  pub fn device_type(&self) -> Any {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Any>(Base::VT_DEVICE_TYPE, Some(Any::NONE)).unwrap()}
  }
  #[inline]
  pub fn device(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Base::VT_DEVICE, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn device_as_type_a(&self) -> Option<TypeA<'a>> {
    if self.device_type() == Any::TypeA {
      self.device().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { TypeA::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn device_as_type_b(&self) -> Option<TypeB<'a>> {
    if self.device_type() == Any::TypeB {
      self.device().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { TypeB::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn device_as_type_c(&self) -> Option<TypeC<'a>> {
    if self.device_type() == Any::TypeC {
      self.device().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { TypeC::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn device_as_type_d(&self) -> Option<TypeD<'a>> {
    if self.device_type() == Any::TypeD {
      self.device().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { TypeD::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Base<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_union::<Any, _>("device_type", Self::VT_DEVICE_TYPE, "device", Self::VT_DEVICE, false, |key, v, pos| {
        match key {
          Any::TypeA => v.verify_union_variant::<flatbuffers::ForwardsUOffset<TypeA>>("Any::TypeA", pos),
          Any::TypeB => v.verify_union_variant::<flatbuffers::ForwardsUOffset<TypeB>>("Any::TypeB", pos),
          Any::TypeC => v.verify_union_variant::<flatbuffers::ForwardsUOffset<TypeC>>("Any::TypeC", pos),
          Any::TypeD => v.verify_union_variant::<flatbuffers::ForwardsUOffset<TypeD>>("Any::TypeD", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct BaseArgs<'a> {
    pub id: i32,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub device_type: Any,
    pub device: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for BaseArgs<'a> {
  #[inline]
  fn default() -> Self {
    BaseArgs {
      id: 0,
      name: None,
      device_type: Any::NONE,
      device: None,
    }
  }
}

pub struct BaseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BaseBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: i32) {
    self.fbb_.push_slot::<i32>(Base::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Base::VT_NAME, name);
  }
  #[inline]
  pub fn add_device_type(&mut self, device_type: Any) {
    self.fbb_.push_slot::<Any>(Base::VT_DEVICE_TYPE, device_type, Any::NONE);
  }
  #[inline]
  pub fn add_device(&mut self, device: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Base::VT_DEVICE, device);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BaseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BaseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Base<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Base<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Base");
      ds.field("id", &self.id());
      ds.field("name", &self.name());
      ds.field("device_type", &self.device_type());
      match self.device_type() {
        Any::TypeA => {
          if let Some(x) = self.device_as_type_a() {
            ds.field("device", &x)
          } else {
            ds.field("device", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Any::TypeB => {
          if let Some(x) = self.device_as_type_b() {
            ds.field("device", &x)
          } else {
            ds.field("device", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Any::TypeC => {
          if let Some(x) = self.device_as_type_c() {
            ds.field("device", &x)
          } else {
            ds.field("device", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Any::TypeD => {
          if let Some(x) = self.device_as_type_d() {
            ds.field("device", &x)
          } else {
            ds.field("device", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("device", &x)
        },
      };
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Base`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_base_unchecked`.
pub fn root_as_base(buf: &[u8]) -> Result<Base, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Base>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Base` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_base_unchecked`.
pub fn size_prefixed_root_as_base(buf: &[u8]) -> Result<Base, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Base>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Base` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_base_unchecked`.
pub fn root_as_base_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Base<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Base<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Base` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_base_unchecked`.
pub fn size_prefixed_root_as_base_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Base<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Base<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Base and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Base`.
pub unsafe fn root_as_base_unchecked(buf: &[u8]) -> Base {
  flatbuffers::root_unchecked::<Base>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Base and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Base`.
pub unsafe fn size_prefixed_root_as_base_unchecked(buf: &[u8]) -> Base {
  flatbuffers::size_prefixed_root_unchecked::<Base>(buf)
}
#[inline]
pub fn finish_base_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Base<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_base_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Base<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Teste

