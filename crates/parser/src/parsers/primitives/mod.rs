mod bytes;
mod string;
mod double;
mod identifier;
mod integer;
mod float;
mod regex;
mod boolean;
mod character;
mod hex;
mod octal;
mod binary;
mod any_primitive;

pub use {
    bytes::BytesParser,
    string::StringParser,
    double::DoubleParser,
    identifier::IdentifierParser,
    integer::IntegerParser,
    float::FloatParser,
    regex::RegexParser,
    boolean::BooleanParser,
    character::CharacterParser,
    hex::HexParser,
    octal::OctalParser,
    binary::BinaryParser,
    any_primitive::AnyPrimitiveParser,
};

#[derive(Debug, Clone)]
pub enum Primitive<'a> {
    String(&'a str),
    Character(&'a str),
    Bytes(&'a [u8]),
    Integer(i64),
    Float(f32),
    Double(f64),
    Hex(&'a str),
    Binary(&'a str),
    Octal(&'a str),
    Regex(&'a str),
    Boolean(bool),
    Ident(&'a str),
}
