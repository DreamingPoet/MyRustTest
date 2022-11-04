// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `UserModels.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:LoginReq)
pub struct LoginReq {
    // message fields
    ///  uint32 id = 1;
    // @@protoc_insertion_point(field:LoginReq.userName)
    pub userName: ::std::string::String,
    // @@protoc_insertion_point(field:LoginReq.password)
    pub password: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:LoginReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LoginReq {
    fn default() -> &'a LoginReq {
        <LoginReq as ::protobuf::Message>::default_instance()
    }
}

impl LoginReq {
    pub fn new() -> LoginReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "userName",
            |m: &LoginReq| { &m.userName },
            |m: &mut LoginReq| { &mut m.userName },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "password",
            |m: &LoginReq| { &m.password },
            |m: &mut LoginReq| { &mut m.password },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LoginReq>(
            "LoginReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LoginReq {
    const NAME: &'static str = "LoginReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.userName = is.read_string()?;
                },
                18 => {
                    self.password = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.userName.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.userName);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.userName.is_empty() {
            os.write_string(1, &self.userName)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> LoginReq {
        LoginReq::new()
    }

    fn clear(&mut self) {
        self.userName.clear();
        self.password.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LoginReq {
        static instance: LoginReq = LoginReq {
            userName: ::std::string::String::new(),
            password: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LoginReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LoginReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LoginReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:LoginResp)
pub struct LoginResp {
    // message fields
    // @@protoc_insertion_point(field:LoginResp.state)
    pub state: ::protobuf::EnumOrUnknown<login_resp::LoginState>,
    // special fields
    // @@protoc_insertion_point(special_field:LoginResp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LoginResp {
    fn default() -> &'a LoginResp {
        <LoginResp as ::protobuf::Message>::default_instance()
    }
}

impl LoginResp {
    pub fn new() -> LoginResp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "state",
            |m: &LoginResp| { &m.state },
            |m: &mut LoginResp| { &mut m.state },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LoginResp>(
            "LoginResp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LoginResp {
    const NAME: &'static str = "LoginResp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.state = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.state != ::protobuf::EnumOrUnknown::new(login_resp::LoginState::Succeed) {
            my_size += ::protobuf::rt::int32_size(1, self.state.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.state != ::protobuf::EnumOrUnknown::new(login_resp::LoginState::Succeed) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.state))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> LoginResp {
        LoginResp::new()
    }

    fn clear(&mut self) {
        self.state = ::protobuf::EnumOrUnknown::new(login_resp::LoginState::Succeed);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LoginResp {
        static instance: LoginResp = LoginResp {
            state: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LoginResp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LoginResp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LoginResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginResp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LoginResp`
pub mod login_resp {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:LoginResp.LoginState)
    pub enum LoginState {
        // @@protoc_insertion_point(enum_value:LoginResp.LoginState.Succeed)
        Succeed = 0,
        // @@protoc_insertion_point(enum_value:LoginResp.LoginState.UserNameNotFound)
        UserNameNotFound = 1,
        // @@protoc_insertion_point(enum_value:LoginResp.LoginState.WrongPassworld)
        WrongPassworld = 2,
    }

    impl ::protobuf::Enum for LoginState {
        const NAME: &'static str = "LoginState";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<LoginState> {
            match value {
                0 => ::std::option::Option::Some(LoginState::Succeed),
                1 => ::std::option::Option::Some(LoginState::UserNameNotFound),
                2 => ::std::option::Option::Some(LoginState::WrongPassworld),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [LoginState] = &[
            LoginState::Succeed,
            LoginState::UserNameNotFound,
            LoginState::WrongPassworld,
        ];
    }

    impl ::protobuf::EnumFull for LoginState {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("LoginResp.LoginState").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for LoginState {
        fn default() -> Self {
            LoginState::Succeed
        }
    }

    impl LoginState {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LoginState>("LoginResp.LoginState")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10UserModels.proto\"B\n\x08LoginReq\x12\x1a\n\x08userName\x18\x01\
    \x20\x01(\tR\x08userName\x12\x1a\n\x08password\x18\x02\x20\x01(\tR\x08pa\
    ssword\"}\n\tLoginResp\x12+\n\x05state\x18\x01\x20\x01(\x0e2\x15.LoginRe\
    sp.LoginStateR\x05state\"C\n\nLoginState\x12\x0b\n\x07Succeed\x10\0\x12\
    \x14\n\x10UserNameNotFound\x10\x01\x12\x12\n\x0eWrongPassworld\x10\x02J\
    \xa2\x03\n\x06\x12\x04\0\0\x10\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\
    \x02\x04\0\x12\x04\x02\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x10\
    \n\x1d\n\x04\x04\0\x02\0\x12\x03\x04\x04\x18\x1a\x10\x20uint32\x20id\x20\
    =\x201;\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x03\x04\x0b\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04\
    \x16\x17\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x05\x04\x18\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x05\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x05\
    \x0b\x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x05\x16\x17\n\n\n\x02\x04\
    \x01\x12\x04\t\0\x10\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\x11\n\x0c\n\
    \x04\x04\x01\x04\0\x12\x04\n\x04\x0e\x05\n\x0c\n\x05\x04\x01\x04\0\x01\
    \x12\x03\n\t\x13\n\r\n\x06\x04\x01\x04\0\x02\0\x12\x03\x0b\x08\x14\n\x0e\
    \n\x07\x04\x01\x04\0\x02\0\x01\x12\x03\x0b\x08\x0f\n\x0e\n\x07\x04\x01\
    \x04\0\x02\0\x02\x12\x03\x0b\x12\x13\n\r\n\x06\x04\x01\x04\0\x02\x01\x12\
    \x03\x0c\x08\x1d\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x01\x12\x03\x0c\x08\
    \x18\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x03\x0c\x1b\x1c\n\r\n\x06\
    \x04\x01\x04\0\x02\x02\x12\x03\r\x08\x1b\n\x0e\n\x07\x04\x01\x04\0\x02\
    \x02\x01\x12\x03\r\x08\x16\n\x0e\n\x07\x04\x01\x04\0\x02\x02\x02\x12\x03\
    \r\x19\x1a\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0f\x04\x19\n\x0c\n\x05\x04\
    \x01\x02\0\x06\x12\x03\x0f\x04\x0e\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x0f\x0f\x14\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0f\x17\x18b\x06proto\
    3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(LoginReq::generated_message_descriptor_data());
            messages.push(LoginResp::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(login_resp::LoginState::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
