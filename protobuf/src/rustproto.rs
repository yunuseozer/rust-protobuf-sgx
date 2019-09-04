// This file is generated by rust-protobuf 2.26.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `rustproto.proto`

/// Extension fields
use std::prelude::v1::*;
pub mod exts {

    pub const expose_oneof_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const serde_derive_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const lite_runtime_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17035, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const serde_derive: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_fields_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const serde_rename_all: crate::ext::ExtFieldOptional<crate::descriptor::EnumOptions, crate::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17032, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:J\n\x11expose_fields_all\x18\
    \xeb\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0fexpose\
    FieldsAll:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x14generateAccessorsAll:b\n\x1ecarller\
    che_bytes_for_bytes_all\x18\xf3\x84\x01\x20\x01(\x08\x12\x1c.google.prot\
    obuf.FileOptionsR\x1acarllercheBytesForBytesAll:d\n\x1fcarllerche_bytes_\
    for_string_all\x18\xf4\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.File\
    OptionsR\x1bcarllercheBytesForStringAll:H\n\x10serde_derive_all\x18\x86\
    \x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eserdeDeriv\
    eAll:O\n\x14serde_derive_cfg_all\x18\x87\x85\x01\x20\x01(\t\x12\x1c.goog\
    le.protobuf.FileOptionsR\x11serdeDeriveCfgAll:H\n\x10lite_runtime_all\
    \x18\x8b\x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eli\
    teRuntimeAll:D\n\x0cexpose_oneof\x18\xe9\x84\x01\x20\x01(\x08\x12\x1f.go\
    ogle.protobuf.MessageOptionsR\x0bexposeOneof:F\n\rexpose_fields\x18\xeb\
    \x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0cexposeF\
    ields:P\n\x12generate_accessors\x18\xec\x84\x01\x20\x01(\x08\x12\x1f.goo\
    gle.protobuf.MessageOptionsR\x11generateAccessors:^\n\x1acarllerche_byte\
    s_for_bytes\x18\xf3\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.Message\
    OptionsR\x17carllercheBytesForBytes:`\n\x1bcarllerche_bytes_for_string\
    \x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\
    \x18carllercheBytesForString:D\n\x0cserde_derive\x18\x86\x85\x01\x20\x01\
    (\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bserdeDerive:K\n\x10serd\
    e_derive_cfg\x18\x87\x85\x01\x20\x01(\t\x12\x1f.google.protobuf.MessageO\
    ptionsR\x0eserdeDeriveCfg:O\n\x13expose_fields_field\x18\xeb\x84\x01\x20\
    \x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x11exposeFieldsField:Y\n\
    \x18generate_accessors_field\x18\xec\x84\x01\x20\x01(\x08\x12\x1d.google\
    .protobuf.FieldOptionsR\x16generateAccessorsField:g\n\x20carllerche_byte\
    s_for_bytes_field\x18\xf3\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.F\
    ieldOptionsR\x1ccarllercheBytesForBytesField:i\n!carllerche_bytes_for_st\
    ring_field\x18\xf4\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOpt\
    ionsR\x1dcarllercheBytesForStringField:H\n\x10serde_rename_all\x18\x88\
    \x85\x01\x20\x01(\t\x12\x1c.google.protobuf.EnumOptionsR\x0eserdeRenameA\
    llJ\xea\x14\n\x06\x12\x04\0\0<\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\
    \x02\x03\0\x12\x03\x02\0*\nh\n\x01\x02\x12\x03\x07\0\x122^\x20see\x20htt\
    ps://github.com/gogo/protobuf/blob/master/gogoproto/gogo.proto\n\x20for\
    \x20the\x20original\x20idea\n\n\t\n\x01\x07\x12\x04\t\0\x1b\x01\n7\n\x02\
    \x07\0\x12\x03\x0b\x04+\x1a,\x20When\x20true,\x20oneof\x20field\x20is\
    \x20generated\x20public\n\n\n\n\x03\x07\0\x02\x12\x03\t\x07\"\n\n\n\x03\
    \x07\0\x04\x12\x03\x0b\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0b\r\x11\n\n\
    \n\x03\x07\0\x01\x12\x03\x0b\x12\"\n\n\n\x03\x07\0\x03\x12\x03\x0b%*\nI\
    \n\x02\x07\x01\x12\x03\r\x04,\x1a>\x20When\x20true\x20all\x20fields\x20a\
    re\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\
    \x01\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\r\x04\x0c\n\n\n\
    \x03\x07\x01\x05\x12\x03\r\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\r\x12#\n\
    \n\n\x03\x07\x01\x03\x12\x03\r&+\nP\n\x02\x07\x02\x12\x03\x0f\x041\x1aE\
    \x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\
    \x20are\x20not\x20generated\n\n\n\n\x03\x07\x02\x02\x12\x03\t\x07\"\n\n\
    \n\x03\x07\x02\x04\x12\x03\x0f\x04\x0c\n\n\n\x03\x07\x02\x05\x12\x03\x0f\
    \r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x0f\x12(\n\n\n\x03\x07\x02\x03\x12\
    \x03\x0f+0\n2\n\x02\x07\x03\x12\x03\x11\x049\x1a'\x20Use\x20`bytes::Byte\
    s`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x03\x02\x12\x03\t\x07\"\n\
    \n\n\x03\x07\x03\x04\x12\x03\x11\x04\x0c\n\n\n\x03\x07\x03\x05\x12\x03\
    \x11\r\x11\n\n\n\x03\x07\x03\x01\x12\x03\x11\x120\n\n\n\x03\x07\x03\x03\
    \x12\x03\x1138\n3\n\x02\x07\x04\x12\x03\x13\x04:\x1a(\x20Use\x20`bytes::\
    Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x04\x02\x12\x03\t\
    \x07\"\n\n\n\x03\x07\x04\x04\x12\x03\x13\x04\x0c\n\n\n\x03\x07\x04\x05\
    \x12\x03\x13\r\x11\n\n\n\x03\x07\x04\x01\x12\x03\x13\x121\n\n\n\x03\x07\
    \x04\x03\x12\x03\x1349\nJ\n\x02\x07\x05\x12\x03\x15\x04+\x1a?\x20Use\x20\
    `serde_derive`\x20to\x20implement\x20`Serialize`\x20and\x20`Deserialize`\
    \n\n\n\n\x03\x07\x05\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x05\x04\x12\x03\
    \x15\x04\x0c\n\n\n\x03\x07\x05\x05\x12\x03\x15\r\x11\n\n\n\x03\x07\x05\
    \x01\x12\x03\x15\x12\"\n\n\n\x03\x07\x05\x03\x12\x03\x15%*\n3\n\x02\x07\
    \x06\x12\x03\x17\x041\x1a(\x20Guard\x20serde\x20annotations\x20with\x20c\
    fg\x20attr.\n\n\n\n\x03\x07\x06\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x06\
    \x04\x12\x03\x17\x04\x0c\n\n\n\x03\x07\x06\x05\x12\x03\x17\r\x13\n\n\n\
    \x03\x07\x06\x01\x12\x03\x17\x14(\n\n\n\x03\x07\x06\x03\x12\x03\x17+0\nN\
    \n\x02\x07\x07\x12\x03\x1a\x04+\x1aC\x20When\x20true,\x20will\x20only\
    \x20generate\x20codes\x20that\x20works\x20with\x20lite\x20runtime.\n\n\n\
    \n\x03\x07\x07\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x07\x04\x12\x03\x1a\x04\
    \x0c\n\n\n\x03\x07\x07\x05\x12\x03\x1a\r\x11\n\n\n\x03\x07\x07\x01\x12\
    \x03\x1a\x12\"\n\n\n\x03\x07\x07\x03\x12\x03\x1a%*\n\t\n\x01\x07\x12\x04\
    \x1d\0,\x01\n7\n\x02\x07\x08\x12\x03\x1f\x04'\x1a,\x20When\x20true,\x20o\
    neof\x20field\x20is\x20generated\x20public\n\n\n\n\x03\x07\x08\x02\x12\
    \x03\x1d\x07%\n\n\n\x03\x07\x08\x04\x12\x03\x1f\x04\x0c\n\n\n\x03\x07\
    \x08\x05\x12\x03\x1f\r\x11\n\n\n\x03\x07\x08\x01\x12\x03\x1f\x12\x1e\n\n\
    \n\x03\x07\x08\x03\x12\x03\x1f!&\nI\n\x02\x07\t\x12\x03!\x04(\x1a>\x20Wh\
    en\x20true\x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\
    \x20generated\n\n\n\n\x03\x07\t\x02\x12\x03\x1d\x07%\n\n\n\x03\x07\t\x04\
    \x12\x03!\x04\x0c\n\n\n\x03\x07\t\x05\x12\x03!\r\x11\n\n\n\x03\x07\t\x01\
    \x12\x03!\x12\x1f\n\n\n\x03\x07\t\x03\x12\x03!\"'\nP\n\x02\x07\n\x12\x03\
    #\x04-\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\
    \x20accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\n\x02\x12\x03\
    \x1d\x07%\n\n\n\x03\x07\n\x04\x12\x03#\x04\x0c\n\n\n\x03\x07\n\x05\x12\
    \x03#\r\x11\n\n\n\x03\x07\n\x01\x12\x03#\x12$\n\n\n\x03\x07\n\x03\x12\
    \x03#',\n2\n\x02\x07\x0b\x12\x03%\x045\x1a'\x20Use\x20`bytes::Bytes`\x20\
    for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x0b\x02\x12\x03\x1d\x07%\n\n\n\
    \x03\x07\x0b\x04\x12\x03%\x04\x0c\n\n\n\x03\x07\x0b\x05\x12\x03%\r\x11\n\
    \n\n\x03\x07\x0b\x01\x12\x03%\x12,\n\n\n\x03\x07\x0b\x03\x12\x03%/4\n3\n\
    \x02\x07\x0c\x12\x03'\x046\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`stri\
    ng`\x20fields\n\n\n\n\x03\x07\x0c\x02\x12\x03\x1d\x07%\n\n\n\x03\x07\x0c\
    \x04\x12\x03'\x04\x0c\n\n\n\x03\x07\x0c\x05\x12\x03'\r\x11\n\n\n\x03\x07\
    \x0c\x01\x12\x03'\x12-\n\n\n\x03\x07\x0c\x03\x12\x03'05\nJ\n\x02\x07\r\
    \x12\x03)\x04'\x1a?\x20Use\x20`serde_derive`\x20to\x20implement\x20`Seri\
    alize`\x20and\x20`Deserialize`\n\n\n\n\x03\x07\r\x02\x12\x03\x1d\x07%\n\
    \n\n\x03\x07\r\x04\x12\x03)\x04\x0c\n\n\n\x03\x07\r\x05\x12\x03)\r\x11\n\
    \n\n\x03\x07\r\x01\x12\x03)\x12\x1e\n\n\n\x03\x07\r\x03\x12\x03)!&\n3\n\
    \x02\x07\x0e\x12\x03+\x04-\x1a(\x20Guard\x20serde\x20annotations\x20with\
    \x20cfg\x20attr.\n\n\n\n\x03\x07\x0e\x02\x12\x03\x1d\x07%\n\n\n\x03\x07\
    \x0e\x04\x12\x03+\x04\x0c\n\n\n\x03\x07\x0e\x05\x12\x03+\r\x13\n\n\n\x03\
    \x07\x0e\x01\x12\x03+\x14$\n\n\n\x03\x07\x0e\x03\x12\x03+',\n\t\n\x01\
    \x07\x12\x04.\07\x01\nI\n\x02\x07\x0f\x12\x030\x04.\x1a>\x20When\x20true\
    \x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\x20genera\
    ted\n\n\n\n\x03\x07\x0f\x02\x12\x03.\x07#\n\n\n\x03\x07\x0f\x04\x12\x030\
    \x04\x0c\n\n\n\x03\x07\x0f\x05\x12\x030\r\x11\n\n\n\x03\x07\x0f\x01\x12\
    \x030\x12%\n\n\n\x03\x07\x0f\x03\x12\x030(-\nP\n\x02\x07\x10\x12\x032\
    \x043\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20\
    accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x10\x02\x12\x03.\
    \x07#\n\n\n\x03\x07\x10\x04\x12\x032\x04\x0c\n\n\n\x03\x07\x10\x05\x12\
    \x032\r\x11\n\n\n\x03\x07\x10\x01\x12\x032\x12*\n\n\n\x03\x07\x10\x03\
    \x12\x032-2\n2\n\x02\x07\x11\x12\x034\x04;\x1a'\x20Use\x20`bytes::Bytes`\
    \x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x11\x02\x12\x03.\x07#\n\n\n\
    \x03\x07\x11\x04\x12\x034\x04\x0c\n\n\n\x03\x07\x11\x05\x12\x034\r\x11\n\
    \n\n\x03\x07\x11\x01\x12\x034\x122\n\n\n\x03\x07\x11\x03\x12\x0345:\n3\n\
    \x02\x07\x12\x12\x036\x04<\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`stri\
    ng`\x20fields\n\n\n\n\x03\x07\x12\x02\x12\x03.\x07#\n\n\n\x03\x07\x12\
    \x04\x12\x036\x04\x0c\n\n\n\x03\x07\x12\x05\x12\x036\r\x11\n\n\n\x03\x07\
    \x12\x01\x12\x036\x123\n\n\n\x03\x07\x12\x03\x12\x0366;\n\t\n\x01\x07\
    \x12\x049\0<\x01\n/\n\x02\x07\x13\x12\x03;\x04-\x1a$\x20use\x20rename_al\
    l\x20attribute\x20for\x20serde\n\n\n\n\x03\x07\x13\x02\x12\x039\x07\"\n\
    \n\n\x03\x07\x13\x04\x12\x03;\x04\x0c\n\n\n\x03\x07\x13\x05\x12\x03;\r\
    \x13\n\n\n\x03\x07\x13\x01\x12\x03;\x14$\n\n\n\x03\x07\x13\x03\x12\x03;'\
    ,\
";

static file_descriptor_proto_lazy: crate::rt::LazyV2<crate::descriptor::FileDescriptorProto> = crate::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> crate::descriptor::FileDescriptorProto {
    crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
