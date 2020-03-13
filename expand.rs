#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use gtk::prelude::*;
use relm::Component;

mod config {







    // impl<WIDGET: relm::Widget> Base for Component<WIDGET> {}




    // Stylesheet

    // Init Bars From Config









    // Alsa Thread
    // I3 Thread
    // Mpris Thread
    // Cpu Thread


    use serde_derive::Deserialize;
    pub struct Bar {
        pub name: String,
        pub monitor: String,
        pub pos_x: i32,
        pub pos_y: i32,
        pub modules_left: Vec<Module>,
        pub modules_right: Vec<Module>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Bar {
        #[inline]
        fn clone(&self) -> Bar {
            match *self {
                Bar {
                name: ref __self_0_0,
                monitor: ref __self_0_1,
                pos_x: ref __self_0_2,
                pos_y: ref __self_0_3,
                modules_left: ref __self_0_4,
                modules_right: ref __self_0_5 } =>
                Bar{name: ::core::clone::Clone::clone(&(*__self_0_0)),
                    monitor: ::core::clone::Clone::clone(&(*__self_0_1)),
                    pos_x: ::core::clone::Clone::clone(&(*__self_0_2)),
                    pos_y: ::core::clone::Clone::clone(&(*__self_0_3)),
                    modules_left: ::core::clone::Clone::clone(&(*__self_0_4)),
                    modules_right:
                        ::core::clone::Clone::clone(&(*__self_0_5)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Bar {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Bar {
                name: ref __self_0_0,
                monitor: ref __self_0_1,
                pos_x: ref __self_0_2,
                pos_y: ref __self_0_3,
                modules_left: ref __self_0_4,
                modules_right: ref __self_0_5 } => {
                    let mut debug_trait_builder = f.debug_struct("Bar");
                    let _ =
                        debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("monitor", &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("pos_x", &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("pos_y", &&(*__self_0_3));
                    let _ =
                        debug_trait_builder.field("modules_left",
                                                  &&(*__self_0_4));
                    let _ =
                        debug_trait_builder.field("modules_right",
                                                  &&(*__self_0_5));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Bar: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($ __expr : expr) =>
                {
                    match $ __expr
                    {
                        _serde :: export :: Ok (__val) => __val, _serde ::
                        export :: Err (__err) =>
                        { return _serde :: export :: Err (__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for Bar {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                5u64 => _serde::export::Ok(__Field::__field5),
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"field index 0 <= i < 6")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "name" =>
                                _serde::export::Ok(__Field::__field0),
                                "monitor" =>
                                _serde::export::Ok(__Field::__field1),
                                "pos_x" =>
                                _serde::export::Ok(__Field::__field2),
                                "pos_y" =>
                                _serde::export::Ok(__Field::__field3),
                                "modules_left" =>
                                _serde::export::Ok(__Field::__field4),
                                "modules_right" =>
                                _serde::export::Ok(__Field::__field5),
                                _ => { _serde::export::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"name" =>
                                _serde::export::Ok(__Field::__field0),
                                b"monitor" =>
                                _serde::export::Ok(__Field::__field1),
                                b"pos_x" =>
                                _serde::export::Ok(__Field::__field2),
                                b"pos_y" =>
                                _serde::export::Ok(__Field::__field3),
                                b"modules_left" =>
                                _serde::export::Ok(__Field::__field4),
                                b"modules_right" =>
                                _serde::export::Ok(__Field::__field5),
                                _ => { _serde::export::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl <'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                         __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Bar>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        Bar;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct Bar")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct Bar with 6 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                     &"struct Bar with 6 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                     &"struct Bar with 6 elements"));
                                    }
                                };
                            let __field3 =
                                match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                     &"struct Bar with 6 elements"));
                                    }
                                };
                            let __field4 =
                                match match _serde::de::SeqAccess::next_element::<Vec<Module>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                     &"struct Bar with 6 elements"));
                                    }
                                };
                            let __field5 =
                                match match _serde::de::SeqAccess::next_element::<Vec<Module>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                     &"struct Bar with 6 elements"));
                                    }
                                };
                            _serde::export::Ok(Bar{name: __field0,
                                                   monitor: __field1,
                                                   pos_x: __field2,
                                                   pos_y: __field3,
                                                   modules_left: __field4,
                                                   modules_right: __field5,})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::export::Option<String> =
                                _serde::export::None;
                            let mut __field1: _serde::export::Option<String> =
                                _serde::export::None;
                            let mut __field2: _serde::export::Option<i32> =
                                _serde::export::None;
                            let mut __field3: _serde::export::Option<i32> =
                                _serde::export::None;
                            let mut __field4:
                                    _serde::export::Option<Vec<Module>> =
                                _serde::export::None;
                            let mut __field5:
                                    _serde::export::Option<Vec<Module>> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                      match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("name"));
                                        }
                                        __field0 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("monitor"));
                                        }
                                        __field1 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("pos_x"));
                                        }
                                        __field2 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<i32>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("pos_y"));
                                        }
                                        __field3 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<i32>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field4 => {
                                        if _serde::export::Option::is_some(&__field4)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("modules_left"));
                                        }
                                        __field4 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<Module>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field5 => {
                                        if _serde::export::Option::is_some(&__field5)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("modules_right"));
                                        }
                                        __field5 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<Module>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::export::Ok(__val) =>
                                                __val,
                                                _serde::export::Err(__err) =>
                                                {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::export::Some(__field0) =>
                                    __field0,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("name")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::export::Some(__field1) =>
                                    __field1,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("monitor")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::export::Some(__field2) =>
                                    __field2,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("pos_x")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field3 =
                                match __field3 {
                                    _serde::export::Some(__field3) =>
                                    __field3,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("pos_y")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field4 =
                                match __field4 {
                                    _serde::export::Some(__field4) =>
                                    __field4,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("modules_left")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field5 =
                                match __field5 {
                                    _serde::export::Some(__field5) =>
                                    __field5,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("modules_right")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            _serde::export::Ok(Bar{name: __field0,
                                                   monitor: __field1,
                                                   pos_x: __field2,
                                                   pos_y: __field3,
                                                   modules_left: __field4,
                                                   modules_right: __field5,})
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["name", "monitor", "pos_x", "pos_y", "modules_left",
                          "modules_right"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "Bar", FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<Bar>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    pub enum Module { I3, Clock, Alsa, Mpris, Cpu, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Module {
        #[inline]
        fn clone(&self) -> Module {
            match (&*self,) {
                (&Module::I3,) => Module::I3,
                (&Module::Clock,) => Module::Clock,
                (&Module::Alsa,) => Module::Alsa,
                (&Module::Mpris,) => Module::Mpris,
                (&Module::Cpu,) => Module::Cpu,
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Module { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Module {
        #[inline]
        fn eq(&self, other: &Module) -> bool {
            {
                let __self_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Module {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Module::I3,) => {
                    let mut debug_trait_builder = f.debug_tuple("I3");
                    debug_trait_builder.finish()
                }
                (&Module::Clock,) => {
                    let mut debug_trait_builder = f.debug_tuple("Clock");
                    debug_trait_builder.finish()
                }
                (&Module::Alsa,) => {
                    let mut debug_trait_builder = f.debug_tuple("Alsa");
                    debug_trait_builder.finish()
                }
                (&Module::Mpris,) => {
                    let mut debug_trait_builder = f.debug_tuple("Mpris");
                    debug_trait_builder.finish()
                }
                (&Module::Cpu,) => {
                    let mut debug_trait_builder = f.debug_tuple("Cpu");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Module: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($ __expr : expr) =>
                {
                    match $ __expr
                    {
                        _serde :: export :: Ok (__val) => __val, _serde ::
                        export :: Err (__err) =>
                        { return _serde :: export :: Err (__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for Module {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                    }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "variant identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"variant index 0 <= i < 5")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "I3" => _serde::export::Ok(__Field::__field0),
                                "Clock" =>
                                _serde::export::Ok(__Field::__field1),
                                "Alsa" =>
                                _serde::export::Ok(__Field::__field2),
                                "Mpris" =>
                                _serde::export::Ok(__Field::__field3),
                                "Cpu" =>
                                _serde::export::Ok(__Field::__field4),
                                _ => {
                                    _serde::export::Err(_serde::de::Error::unknown_variant(__value,
                                                                                           VARIANTS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"I3" =>
                                _serde::export::Ok(__Field::__field0),
                                b"Clock" =>
                                _serde::export::Ok(__Field::__field1),
                                b"Alsa" =>
                                _serde::export::Ok(__Field::__field2),
                                b"Mpris" =>
                                _serde::export::Ok(__Field::__field3),
                                b"Cpu" =>
                                _serde::export::Ok(__Field::__field4),
                                _ => {
                                    let __value =
                                        &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(__value,
                                                                                           VARIANTS))
                                }
                            }
                        }
                    }
                    impl <'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                         __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Module>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        Module;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "enum Module")
                        }
                        fn visit_enum<__A>(self, __data: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::EnumAccess<'de> {
                            match match _serde::de::EnumAccess::variant(__data)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                (__Field::__field0, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(Module::I3)
                                }
                                (__Field::__field1, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(Module::Clock)
                                }
                                (__Field::__field2, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(Module::Alsa)
                                }
                                (__Field::__field3, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(Module::Mpris)
                                }
                                (__Field::__field4, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(Module::Cpu)
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["I3", "Clock", "Alsa", "Mpris", "Cpu"];
                    _serde::Deserializer::deserialize_enum(__deserializer,
                                                           "Module", VARIANTS,
                                                           __Visitor{marker:
                                                                         _serde::export::PhantomData::<Module>,
                                                                     lifetime:
                                                                         _serde::export::PhantomData,})
                }
            }
        };
    pub struct Config {
        pub bars: Vec<Bar>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Config {
        #[inline]
        fn clone(&self) -> Config {
            match *self {
                Config { bars: ref __self_0_0 } =>
                Config{bars: ::core::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Config {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Config { bars: ref __self_0_0 } => {
                    let mut debug_trait_builder = f.debug_struct("Config");
                    let _ =
                        debug_trait_builder.field("bars", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Config: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($ __expr : expr) =>
                {
                    match $ __expr
                    {
                        _serde :: export :: Ok (__val) => __val, _serde ::
                        export :: Err (__err) =>
                        { return _serde :: export :: Err (__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for Config {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"field index 0 <= i < 1")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "bars" =>
                                _serde::export::Ok(__Field::__field0),
                                _ => { _serde::export::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"bars" =>
                                _serde::export::Ok(__Field::__field0),
                                _ => { _serde::export::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl <'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                         __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Config>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        Config;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct Config")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<Vec<Bar>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct Config with 1 element"));
                                    }
                                };
                            _serde::export::Ok(Config{bars: __field0,})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            let mut __field0:
                                    _serde::export::Option<Vec<Bar>> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                      match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("bars"));
                                        }
                                        __field0 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<Bar>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::export::Ok(__val) =>
                                                __val,
                                                _serde::export::Err(__err) =>
                                                {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::export::Some(__field0) =>
                                    __field0,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("bars")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            _serde::export::Ok(Config{bars: __field0,})
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["bars"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "Config", FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<Config>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    use std::path::PathBuf;
    pub fn config_dir() -> Option<PathBuf> {
        let home =
            std::env::var_os("HOME").and_then(|h|
                                                  if h.is_empty() {
                                                      None
                                                  } else { Some(h) });
        if let Some(home) = home {
            Some(PathBuf::from(home).join(".config"))
        } else { None }
    }
    pub fn get_config() -> Config {
        let config_dir = config_dir();
        let default_config =
            "[[bars]]\nname = \"bar-left\"\nmonitor = \"DP-1\"\npos_x = 0\npos_y = 1000\n\nmodules_left = [\"I3\"]\nmodules_right = [\"Clock\",\"Alsa\",\"Mpris\",\"Cpu\"]\n\n[[bars]]\nname = \"bar-right\"\nmonitor = \"HDMI-0\"\npos_x = 1920\npos_y = 1000\n\nmodules_left = [\"I3\"]\nmodules_right = [\"Clock\",\"Alsa\",\"Mpris\",\"Cpu\"]\n";
        let toml_str =
            if let Some(config_dir) = config_dir {
                let bar_config_dir = config_dir.join("YetAnotherBar");
                let _ = std::fs::create_dir_all(&bar_config_dir);
                if let Ok(file) =
                       std::fs::read_to_string(&bar_config_dir.join("config.toml"))
                   {
                    file
                } else { default_config.into() }
            } else { default_config.into() };
        let decoded: Config = toml::from_str(&toml_str).unwrap();
        decoded
    }
}
mod bar {
    use gtk::prelude::*;
    use gtk::{Inhibit, Window, WindowType};
    use relm::{connect, Relm, Update, Widget};
    use relm_derive::Msg;
    use crate::ModuleComponent;
    pub struct ModelParam {
        pub bar_name: String,
        pub monitor_name: String,
        pub x: i32,
        pub y: i32,
        pub modules_left: Vec<ModuleComponent>,
        pub modules_right: Vec<ModuleComponent>,
    }
    pub struct Model {
        params: ModelParam,
    }
    pub enum Msg { Quit, SetVisual, }
    impl ::relm::DisplayVariant for Msg {
        #[allow(unused_qualifications)]
        fn display_variant(&self) -> &'static str {
            match *self {
                Msg::Quit { .. } => "Quit",
                Msg::SetVisual { .. } => "SetVisual",
            }
        }
    }
    impl ::relm::IntoOption<Msg> for Msg {
        fn into_option(self) -> Option<Msg> { Some(self) }
    }
    pub struct Bar {
        gtk_window: gtk::Window,
        gtk_box: gtk::Box,
        model: Model,
    }
    impl Widget for Bar {
        fn init_view(&mut self) {
            for module in self.model.params.modules_left.iter() {
                self.gtk_box.pack_start(&module.widget(), false, false, 0);
            }
            for module in self.model.params.modules_right.iter() {
                self.gtk_box.pack_end(&module.widget(), false, false, 0);
            }
            self.gtk_window.show_all();
        }
        fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self {
            let gtk_window: gtk::Window =
                gtk::WindowBuilder::new().type_(WindowType::Toplevel).name(&model.params.bar_name).type_hint(gdk::WindowTypeHint::Dock).decorated(false).default_height(35).default_width(1920).app_paintable(true).build();
            gtk_window.move_(model.params.x, model.params.y);
            Self::set_visual(&gtk_window, None);
            let gtk_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            gtk_box.set_widget_name("main_box");
            gtk_window.add(&gtk_box);
            {
                {
                    let stream = relm.stream().clone();
                    let _ =
                        gtk_window.connect_screen_changed(move |_, _|
                                                              {
                                                                  let (msg,
                                                                       return_value) =
                                                                      ::relm::IntoPair::into_pair((Some(Msg::SetVisual),
                                                                                                   ()));
                                                                  let msg:
                                                                          Option<_> =
                                                                      ::relm::IntoOption::into_option(msg);
                                                                  if let Some(msg)
                                                                         = msg
                                                                     {
                                                                      stream.emit(msg);
                                                                  }
                                                                  return_value
                                                              });
                };
            };
            {
                {
                    let stream = relm.stream().clone();
                    let _ =
                        gtk_window.connect_delete_event(move |_, _|
                                                            {
                                                                let (msg,
                                                                     return_value) =
                                                                    ::relm::IntoPair::into_pair((Some(Msg::Quit),
                                                                                                 Inhibit(false)));
                                                                let msg:
                                                                        Option<_> =
                                                                    ::relm::IntoOption::into_option(msg);
                                                                if let Some(msg)
                                                                       = msg {
                                                                    stream.emit(msg);
                                                                }
                                                                return_value
                                                            });
                };
            };
            Bar{gtk_window, gtk_box, model,}
        }
        type
        Root
        =
        Window;
        fn root(&self) -> Self::Root { self.gtk_window.clone() }
    }
    impl Update for Bar {
        type
        Msg
        =
        Msg;
        type
        Model
        =
        Model;
        type
        ModelParam
        =
        ModelParam;
        fn update(&mut self, event: Msg) {
            match event {
                Msg::Quit => gtk::main_quit(),
                Msg::SetVisual => Self::set_visual(&self.gtk_window, None),
            }
        }
        fn model(_: &Relm<Self>, params: ModelParam) -> Model {
            Model{params,}
        }
    }
    impl Bar {
        fn set_visual(window: &Window, _screen: Option<&gdk::Screen>) {
            if let Some(screen) = window.get_screen() {
                if let Some(ref visual) = screen.get_rgba_visual() {
                    window.set_visual(Some(visual));
                }
            }
        }
    }
}
mod clock {
    use gtk::prelude::*;
    use gtk::Inhibit;
    use relm::{Relm, Widget};
    use relm_derive::{widget, Msg};
    pub struct Model {
        unfolded: bool,
        time: String,
    }
    pub enum Msg { Tick, Click, }
    impl ::relm::DisplayVariant for Msg {
        #[allow(unused_qualifications)]
        fn display_variant(&self) -> &'static str {
            match *self {
                Msg::Tick { .. } => "Tick",
                Msg::Click { .. } => "Click",
            }
        }
    }
    impl ::relm::IntoOption<Msg> for Msg {
        fn into_option(self) -> Option<Msg> { Some(self) }
    }
    #[allow(dead_code, missing_docs)]
    pub struct Clock {
        gtklabel1: gtk::Label,
        gtkeventbox1: gtk::EventBox,
        model: Model,
    }
    pub struct __ClockWidgets {
        pub gtkeventbox1: gtk::EventBox,
    }
    impl Widget for Clock {
        fn init_view(&mut self) { }
        #[allow(unused_variables)]
        fn view(relm: &::relm::Relm<Self>, __relm_model: Self::Model)
         -> Self {
            let gtkeventbox1: gtk::EventBox =
                unsafe {
                    if !gtk::is_initialized_main_thread() {
                        if gtk::is_initialized() {
                            {
                                ::std::rt::begin_panic("GTK may only be used from the main thread.")
                            };
                        } else {
                            {
                                ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                            };
                        }
                    }
                    use relm::StaticType;
                    use relm::{Cast, FromGlibPtrNone};
                    let values: &[::relm::Value] = &[];
                    let mut parameters = [];
                    ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::EventBox::static_type()),
                                                                        0u32,
                                                                        parameters.as_mut_ptr())
                                                      as
                                                      *mut _).downcast().unwrap()
                };
            let gtklabel1: gtk::Label =
                unsafe {
                    if !gtk::is_initialized_main_thread() {
                        if gtk::is_initialized() {
                            {
                                ::std::rt::begin_panic("GTK may only be used from the main thread.")
                            };
                        } else {
                            {
                                ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                            };
                        }
                    }
                    use relm::StaticType;
                    use relm::{Cast, FromGlibPtrNone};
                    let values: &[::relm::Value] = &[];
                    let mut parameters = [];
                    ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::Label::static_type()),
                                                                        0u32,
                                                                        parameters.as_mut_ptr())
                                                      as
                                                      *mut _).downcast().unwrap()
                };
            gtklabel1.set_widget_name("clock");
            gtklabel1.set_text(&{ let model = &__relm_model; model }.time);
            ::gtk::ContainerExt::add(&gtkeventbox1, &gtklabel1);
            ::gtk::WidgetExt::show(&gtklabel1);
            ::gtk::WidgetExt::show(&gtkeventbox1);
            {
                {
                    {
                        let stream = relm.stream().clone();
                        let _ =
                            gtkeventbox1.connect_button_press_event(move
                                                                        |_, _|
                                                                        {
                                                                            let (msg,
                                                                                 return_value) =
                                                                                ::relm::IntoPair::into_pair((Msg::Click,
                                                                                                             Inhibit(false)));
                                                                            let msg:
                                                                                    Option<_> =
                                                                                ::relm::IntoOption::into_option(msg);
                                                                            if let Some(msg)
                                                                                   =
                                                                                   msg
                                                                               {
                                                                                stream.emit(msg);
                                                                            }
                                                                            return_value
                                                                        });
                    };
                };
            }
            Clock{gtkeventbox1: gtkeventbox1,
                  gtklabel1: gtklabel1,
                  model: __relm_model,}
        }
        type
        Root
        =
        gtk::EventBox;
        fn root(&self) -> Self::Root { self.gtkeventbox1.clone() }
    }
    impl ::relm::Update for Clock {
        type
        Msg
        =
        Msg;
        type
        Model
        =
        Model;
        type
        ModelParam
        =
        ();
        fn update(&mut self, event: Msg) {
            match event {
                Msg::Click => {
                    { self.model.unfolded = !self.model.unfolded; };
                    self.update(Msg::Tick);
                }
                Msg::Tick => {
                    let time = chrono::Local::now();
                    if !self.model.unfolded {
                        {
                            self.model.time =
                                {
                                    let res =
                                        ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
                                                                                            &match (&time.format("%H:%M"),)
                                                                                                 {
                                                                                                 (arg0,)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Display::fmt)],
                                                                                             }));
                                    res
                                };
                            self.gtklabel1.set_text(&self.model.time);
                        };
                    } else {
                        {
                            self.model.time =
                                {
                                    let res =
                                        ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
                                                                                            &match (&time.format("%Y-%m-%d"),)
                                                                                                 {
                                                                                                 (arg0,)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Display::fmt)],
                                                                                             }));
                                    res
                                };
                            self.gtklabel1.set_text(&self.model.time);
                        };
                    }
                }
            }
        }
        fn model(_: &::relm::Relm<Self>, _: ()) -> Model {
            let time = chrono::Local::now();
            Model{unfolded: false,
                  time:
                      {
                          let res =
                              ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
                                                                                  &match (&time.format("%H:%M"),)
                                                                                       {
                                                                                       (arg0,)
                                                                                       =>
                                                                                       [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::core::fmt::Display::fmt)],
                                                                                   }));
                          res
                      },}
        }
        fn subscriptions(&mut self, relm: &Relm<Self>) {
            relm::interval(relm.stream(), 1000, || Msg::Tick);
        }
    }
    impl ::relm::WidgetTest for Clock {
        type
        Widgets
        =
        __ClockWidgets;
        fn get_widgets(&self) -> __ClockWidgets {
            __ClockWidgets{gtkeventbox1: self.gtkeventbox1.clone(),}
        }
    }
    impl Clock { }
}
mod alsa {
    mod alsa {
        use gtk::prelude::*;
        use gtk::Inhibit;
        use relm::Widget;
        use relm_derive::{widget, Msg};
        pub struct Model {
            alsa_mixer: alsa::Mixer,
            volume: String,
        }
        pub enum Msg {
            Update(i64, i32),
            Mute,
            VolumeChange(gdk::ScrollDirection),
        }
        impl ::relm::DisplayVariant for Msg {
            #[allow(unused_qualifications)]
            fn display_variant(&self) -> &'static str {
                match *self {
                    Msg::Update { .. } => "Update",
                    Msg::Mute { .. } => "Mute",
                    Msg::VolumeChange { .. } => "VolumeChange",
                }
            }
        }
        impl ::relm::IntoOption<Msg> for Msg {
            fn into_option(self) -> Option<Msg> { Some(self) }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Msg {
            #[inline]
            fn clone(&self) -> Msg {
                match (&*self,) {
                    (&Msg::Update(ref __self_0, ref __self_1),) =>
                    Msg::Update(::core::clone::Clone::clone(&(*__self_0)),
                                ::core::clone::Clone::clone(&(*__self_1))),
                    (&Msg::Mute,) => Msg::Mute,
                    (&Msg::VolumeChange(ref __self_0),) =>
                    Msg::VolumeChange(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[allow(dead_code, missing_docs)]
        pub struct Alsa {
            gtk_label: gtk::Label,
            gtkeventbox2: gtk::EventBox,
            model: Model,
        }
        pub struct __AlsaWidgets {
            pub gtk_label: gtk::Label,
            pub gtkeventbox2: gtk::EventBox,
        }
        impl Widget for Alsa {
            #[allow(unused_variables)]
            fn view(relm: &::relm::Relm<Self>, __relm_model: Self::Model)
             -> Self {
                let gtkeventbox2: gtk::EventBox =
                    unsafe {
                        if !gtk::is_initialized_main_thread() {
                            if gtk::is_initialized() {
                                {
                                    ::std::rt::begin_panic("GTK may only be used from the main thread.")
                                };
                            } else {
                                {
                                    ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                                };
                            }
                        }
                        use relm::StaticType;
                        use relm::{Cast, FromGlibPtrNone};
                        let values: &[::relm::Value] = &[];
                        let mut parameters = [];
                        ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::EventBox::static_type()),
                                                                            0u32,
                                                                            parameters.as_mut_ptr())
                                                          as
                                                          *mut _).downcast().unwrap()
                    };
                gtkeventbox2.set_events(gdk::EventMask::SCROLL_MASK);
                let gtk_label: gtk::Label =
                    unsafe {
                        if !gtk::is_initialized_main_thread() {
                            if gtk::is_initialized() {
                                {
                                    ::std::rt::begin_panic("GTK may only be used from the main thread.")
                                };
                            } else {
                                {
                                    ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                                };
                            }
                        }
                        use relm::StaticType;
                        use relm::{Cast, FromGlibPtrNone};
                        let values: &[::relm::Value] = &[];
                        let mut parameters = [];
                        ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::Label::static_type()),
                                                                            0u32,
                                                                            parameters.as_mut_ptr())
                                                          as
                                                          *mut _).downcast().unwrap()
                    };
                gtk_label.set_text(&{
                                        let model = &__relm_model;
                                        model
                                    }.volume);
                gtk_label.set_widget_name("pulse");
                ::gtk::ContainerExt::add(&gtkeventbox2, &gtk_label);
                ::gtk::WidgetExt::show(&gtk_label);
                ::gtk::WidgetExt::show(&gtkeventbox2);
                {
                    {
                        {
                            let stream = relm.stream().clone();
                            let _ =
                                gtkeventbox2.connect_button_press_event(move
                                                                            |_,
                                                                             _|
                                                                            {
                                                                                let (msg,
                                                                                     return_value) =
                                                                                    ::relm::IntoPair::into_pair((Msg::Mute,
                                                                                                                 Inhibit(false)));
                                                                                let msg:
                                                                                        Option<_> =
                                                                                    ::relm::IntoOption::into_option(msg);
                                                                                if let Some(msg)
                                                                                       =
                                                                                       msg
                                                                                   {
                                                                                    stream.emit(msg);
                                                                                }
                                                                                return_value
                                                                            });
                        };
                    };
                }
                {
                    {
                        {
                            let stream = relm.stream().clone();
                            let _ =
                                gtkeventbox2.connect_scroll_event(move |_, se|
                                                                      {
                                                                          let (msg,
                                                                               return_value) =
                                                                              ::relm::IntoPair::into_pair((Msg::VolumeChange(se.get_direction()),
                                                                                                           Inhibit(false)));
                                                                          let msg:
                                                                                  Option<_> =
                                                                              ::relm::IntoOption::into_option(msg);
                                                                          if let Some(msg)
                                                                                 =
                                                                                 msg
                                                                             {
                                                                              stream.emit(msg);
                                                                          }
                                                                          return_value
                                                                      });
                        };
                    };
                }
                Alsa{gtkeventbox2: gtkeventbox2,
                     gtk_label: gtk_label,
                     model: __relm_model,}
            }
            type
            Root
            =
            gtk::EventBox;
            fn root(&self) -> Self::Root { self.gtkeventbox2.clone() }
        }
        impl ::relm::Update for Alsa {
            type
            Msg
            =
            Msg;
            type
            Model
            =
            Model;
            type
            ModelParam
            =
            ();
            fn update(&mut self, event: Msg) {
                match event {
                    Msg::Update(volume, state) => {
                        let p =
                            (volume as f32 * 100.0 / 0x10000 as f32).round();
                        {
                            self.model.volume = p.to_string() + "%";
                            self.gtk_label.set_text(&self.model.volume);
                        };
                        if state == 0 {
                            self.gtk_label.get_style_context().add_class("muted");
                        } else {
                            self.gtk_label.get_style_context().remove_class("muted");
                        }
                    }
                    Msg::Mute => {
                        let master =
                            self.model.alsa_mixer.find_selem(&alsa::mixer::SelemId::new("Master",
                                                                                        0)).unwrap();
                        let state =
                            master.get_playback_switch(alsa::mixer::SelemChannelId::FrontLeft).unwrap();
                        if state == 0 {
                            let _ = master.set_playback_switch_all(1);
                        } else { let _ = master.set_playback_switch_all(0); }
                    }
                    Msg::VolumeChange(sd) => {
                        let master =
                            self.model.alsa_mixer.find_selem(&alsa::mixer::SelemId::new("Master",
                                                                                        0)).unwrap();
                        let mult =
                            match sd {
                                gdk::ScrollDirection::Up => 1,
                                gdk::ScrollDirection::Down => -1,
                                _ => return,
                            };
                        let add = (5.0 * 655.35) as i64 * mult;
                        let volume =
                            master.get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft).unwrap();
                        let _ = master.set_playback_volume_all(volume + add);
                    }
                }
            }
            fn model(_: &::relm::Relm<Self>, _: ()) -> Model {
                Model{alsa_mixer: alsa::Mixer::new("default", true).unwrap(),
                      volume: "0".into(),}
            }
        }
        impl ::relm::WidgetTest for Alsa {
            type
            Widgets
            =
            __AlsaWidgets;
            fn get_widgets(&self) -> __AlsaWidgets {
                __AlsaWidgets{gtk_label: self.gtk_label.clone(),
                              gtkeventbox2: self.gtkeventbox2.clone(),}
            }
        }
        impl Alsa { }
    }
    pub mod alsa_thread {
        pub fn run(streams: Vec<relm::EventStream<super::alsa::Msg>>) {
            let (_channel, sender) =
                relm::Channel::new(move |msg: super::alsa::Msg|
                                       {
                                           for s in streams.iter() {
                                               s.emit(msg.clone());
                                           }
                                       });
            std::thread::spawn(move ||
                                   {
                                       fn update_event(alsa_mixer:
                                                           &::alsa::Mixer)
                                        -> super::alsa::Msg {
                                           let master =
                                               alsa_mixer.find_selem(&::alsa::mixer::SelemId::new("Master",
                                                                                                  0)).unwrap();
                                           let volume =
                                               master.get_playback_volume(::alsa::mixer::SelemChannelId::FrontLeft).unwrap();
                                           let state =
                                               master.get_playback_switch(::alsa::mixer::SelemChannelId::FrontLeft).unwrap();
                                           super::alsa::Msg::Update(volume,
                                                                    state)
                                       }
                                       let alsa_mixer =
                                           ::alsa::Mixer::new("default",
                                                              true).unwrap();
                                       sender.send(update_event(&alsa_mixer)).expect("alsa send");
                                       loop  {
                                           if alsa_mixer.handle_events().unwrap()
                                                  == 1 {
                                               sender.send(update_event(&alsa_mixer)).expect("alsa send");
                                           }
                                           std::thread::sleep(std::time::Duration::from_millis(100));
                                       }
                                   });
        }
    }
    pub use self::alsa::*;
}
mod cpu {
    mod cpu {
        use gtk::prelude::*;
        use relm::{Relm, Widget};
        use relm_derive::{widget, Msg};
        pub struct Model {
            out: String,
        }
        pub enum Msg { Update(String), }
        impl ::relm::DisplayVariant for Msg {
            #[allow(unused_qualifications)]
            fn display_variant(&self) -> &'static str {
                match *self { Msg::Update { .. } => "Update", }
            }
        }
        impl ::relm::IntoOption<Msg> for Msg {
            fn into_option(self) -> Option<Msg> { Some(self) }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Msg {
            #[inline]
            fn clone(&self) -> Msg {
                match (&*self,) {
                    (&Msg::Update(ref __self_0),) =>
                    Msg::Update(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[allow(dead_code, missing_docs)]
        pub struct Cpu {
            gtklabel3: gtk::Label,
            gtkeventbox3: gtk::EventBox,
            model: Model,
        }
        pub struct __CpuWidgets {
            pub gtkeventbox3: gtk::EventBox,
        }
        impl Widget for Cpu {
            fn init_view(&mut self) { }
            #[allow(unused_variables)]
            fn view(relm: &::relm::Relm<Self>, __relm_model: Self::Model)
             -> Self {
                let gtkeventbox3: gtk::EventBox =
                    unsafe {
                        if !gtk::is_initialized_main_thread() {
                            if gtk::is_initialized() {
                                {
                                    ::std::rt::begin_panic("GTK may only be used from the main thread.")
                                };
                            } else {
                                {
                                    ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                                };
                            }
                        }
                        use relm::StaticType;
                        use relm::{Cast, FromGlibPtrNone};
                        let values: &[::relm::Value] = &[];
                        let mut parameters = [];
                        ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::EventBox::static_type()),
                                                                            0u32,
                                                                            parameters.as_mut_ptr())
                                                          as
                                                          *mut _).downcast().unwrap()
                    };
                let gtklabel3: gtk::Label =
                    unsafe {
                        if !gtk::is_initialized_main_thread() {
                            if gtk::is_initialized() {
                                {
                                    ::std::rt::begin_panic("GTK may only be used from the main thread.")
                                };
                            } else {
                                {
                                    ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                                };
                            }
                        }
                        use relm::StaticType;
                        use relm::{Cast, FromGlibPtrNone};
                        let values: &[::relm::Value] = &[];
                        let mut parameters = [];
                        ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::Label::static_type()),
                                                                            0u32,
                                                                            parameters.as_mut_ptr())
                                                          as
                                                          *mut _).downcast().unwrap()
                    };
                gtklabel3.set_text(&{ let model = &__relm_model; model }.out);
                gtklabel3.set_widget_name("cpu");
                ::gtk::ContainerExt::add(&gtkeventbox3, &gtklabel3);
                ::gtk::WidgetExt::show(&gtklabel3);
                ::gtk::WidgetExt::show(&gtkeventbox3);
                Cpu{gtkeventbox3: gtkeventbox3,
                    gtklabel3: gtklabel3,
                    model: __relm_model,}
            }
            type
            Root
            =
            gtk::EventBox;
            fn root(&self) -> Self::Root { self.gtkeventbox3.clone() }
        }
        impl ::relm::Update for Cpu {
            type
            Msg
            =
            Msg;
            type
            Model
            =
            Model;
            type
            ModelParam
            =
            ();
            fn update(&mut self, event: Msg) {
                match event {
                    Msg::Update(s) => {
                        self.model.out = s;
                        self.gtklabel3.set_text(&self.model.out);
                    }
                }
            }
            fn model(_: &::relm::Relm<Self>, _: ()) -> Model {
                Model{out: "▁ ▁ ▁ ▁".into(),}
            }
        }
        impl ::relm::WidgetTest for Cpu {
            type
            Widgets
            =
            __CpuWidgets;
            fn get_widgets(&self) -> __CpuWidgets {
                __CpuWidgets{gtkeventbox3: self.gtkeventbox3.clone(),}
            }
        }
        impl Cpu { }
    }
    pub mod cpu_thread {
        pub fn run(streams: Vec<relm::EventStream<super::cpu::Msg>>) {
            let (_channel, sender) =
                relm::Channel::new(move |u: String|
                                       {
                                           for s in streams.iter() {
                                               s.emit(super::cpu::Msg::Update(u.clone()));
                                           }
                                       });
            std::thread::spawn(move ||
                                   {
                                       use systemstat::{Platform, System};
                                       let sys = System::new();
                                       loop  {
                                           match sys.cpu_load() {
                                               Ok(cpu) => {
                                                   std::thread::sleep(std::time::Duration::from_secs(1));
                                                   let cpus =
                                                       cpu.done().unwrap();
                                                   let mut load =
                                                       String::new();
                                                   load += " ";
                                                   let list =
                                                       ['▁', '▂', '▃',
                                                        '▄', '▅', '▆',
                                                        '▇', '█'];
                                                   for cpu in cpus {
                                                       let cp =
                                                           (cpu.user +
                                                                cpu.nice +
                                                                cpu.system +
                                                                cpu.interrupt)
                                                               * 7.0;
                                                       load +=
                                                           &list[cp as
                                                                     usize].to_string();
                                                       load += " ";
                                                   }
                                                   sender.send(load).expect("cpu_thread send message");
                                               }
                                               Err(x) => {
                                                   ::std::io::_print(::core::fmt::Arguments::new_v1(&["\nCPU load: error: ",
                                                                                                      "\n"],
                                                                                                    &match (&x,)
                                                                                                         {
                                                                                                         (arg0,)
                                                                                                         =>
                                                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                                       ::core::fmt::Display::fmt)],
                                                                                                     }));
                                               }
                                           }
                                       }
                                   });
        }
    }
    pub use cpu::*;
}
mod i3 {
    mod i3 {
        use gtk::prelude::*;
        use relm::{Relm, Widget};
        use relm_derive::{widget, Msg};
        use std::{cell::RefCell, rc::Rc};
        use super::i3_thread;
        pub struct Model {
            monitor_name: String,
            gtk_buttons: Vec<gtk::Button>,
            gtk_label: gtk::Label,
            i3_connection: Rc<RefCell<i3ipc::I3Connection>>,
        }
        pub enum Msg {
            UpdateWorkspaces(Vec<i3_thread::I3Workspace>),
            UpdateMode(String),
        }
        impl ::relm::DisplayVariant for Msg {
            #[allow(unused_qualifications)]
            fn display_variant(&self) -> &'static str {
                match *self {
                    Msg::UpdateWorkspaces { .. } => "UpdateWorkspaces",
                    Msg::UpdateMode { .. } => "UpdateMode",
                }
            }
        }
        impl ::relm::IntoOption<Msg> for Msg {
            fn into_option(self) -> Option<Msg> { Some(self) }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Msg {
            #[inline]
            fn clone(&self) -> Msg {
                match (&*self,) {
                    (&Msg::UpdateWorkspaces(ref __self_0),) =>
                    Msg::UpdateWorkspaces(::core::clone::Clone::clone(&(*__self_0))),
                    (&Msg::UpdateMode(ref __self_0),) =>
                    Msg::UpdateMode(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[allow(dead_code, missing_docs)]
        pub struct I3 {
            gtk_box: gtk::Box,
            model: Model,
        }
        pub struct __I3Widgets {
            pub gtk_box: gtk::Box,
        }
        impl Widget for I3 {
            fn init_view(&mut self) {
                self.gtk_box.pack_end(&self.model.gtk_label, false, false, 5);
            }
            #[allow(unused_variables)]
            fn view(relm: &::relm::Relm<Self>, __relm_model: Self::Model)
             -> Self {
                let gtk_box: gtk::Box =
                    unsafe {
                        if !gtk::is_initialized_main_thread() {
                            if gtk::is_initialized() {
                                {
                                    ::std::rt::begin_panic("GTK may only be used from the main thread.")
                                };
                            } else {
                                {
                                    ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                                };
                            }
                        }
                        use relm::StaticType;
                        use relm::{Cast, FromGlibPtrNone};
                        let values: &[::relm::Value] = &[];
                        let mut parameters = [];
                        ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::Box::static_type()),
                                                                            0u32,
                                                                            parameters.as_mut_ptr())
                                                          as
                                                          *mut _).downcast().unwrap()
                    };
                gtk_box.set_widget_name("i3");
                gtk_box.set_orientation(gtk::Orientation::Horizontal);
                ::gtk::WidgetExt::show(&gtk_box);
                I3{gtk_box: gtk_box, model: __relm_model,}
            }
            type
            Root
            =
            gtk::Box;
            fn root(&self) -> Self::Root { self.gtk_box.clone() }
        }
        impl ::relm::Update for I3 {
            type
            Msg
            =
            Msg;
            type
            Model
            =
            Model;
            type
            ModelParam
            =
            String;
            fn update(&mut self, event: Msg) {
                match event {
                    Msg::UpdateWorkspaces(workspaces) => {
                        for btn in self.model.gtk_buttons.iter() {
                            btn.destroy();
                        }
                        for ws in workspaces.iter() {
                            if ws.output != self.model.monitor_name {
                                continue ;
                            }
                            let btn = gtk::Button::new_with_label(&ws.name);
                            if ws.focused {
                                btn.get_style_context().add_class("focused");
                            }
                            if ws.urgent {
                                btn.get_style_context().add_class("urgent");
                            }
                            let name = ws.name.clone();
                            let i3_connection =
                                self.model.i3_connection.clone();
                            btn.connect_clicked(move |_|
                                                    {
                                                        let _ =
                                                            i3_connection.borrow_mut().run_command(&{
                                                                                                        let res =
                                                                                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["workspace "],
                                                                                                                                                                &match (&name,)
                                                                                                                                                                     {
                                                                                                                                                                     (arg0,)
                                                                                                                                                                     =>
                                                                                                                                                                     [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                   ::core::fmt::Display::fmt)],
                                                                                                                                                                 }));
                                                                                                        res
                                                                                                    });
                                                    });
                            self.gtk_box.pack_start(&btn, false, false, 0);
                            gtk::WidgetExt::show(&btn);
                            self.model.gtk_buttons.push(btn);
                        }
                    }
                    Msg::UpdateMode(mode) => {
                        if mode != "default" {
                            self.model.gtk_label.set_text(&mode);
                        } else { self.model.gtk_label.set_text(""); }
                    }
                }
            }
            fn model(_: &Relm<Self>, monitor_name: String) -> Model {
                let i3_connection =
                    Rc::new(RefCell::new(i3ipc::I3Connection::connect().unwrap()));
                Model{monitor_name,
                      gtk_buttons: Vec::new(),
                      gtk_label:
                          gtk::LabelBuilder::new().name("mode").build(),
                      i3_connection,}
            }
        }
        impl ::relm::WidgetTest for I3 {
            type
            Widgets
            =
            __I3Widgets;
            fn get_widgets(&self) -> __I3Widgets {
                __I3Widgets{gtk_box: self.gtk_box.clone(),}
            }
        }
        impl I3 { }
    }
    pub mod i3_thread {
        pub struct RandrMonitor {
            pub name: String,
            pub x: i16,
            pub y: i16,
            pub w: u16,
            pub h: u16,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for RandrMonitor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    RandrMonitor {
                    name: ref __self_0_0,
                    x: ref __self_0_1,
                    y: ref __self_0_2,
                    w: ref __self_0_3,
                    h: ref __self_0_4 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("RandrMonitor");
                        let _ =
                            debug_trait_builder.field("name",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("x", &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("y", &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("w", &&(*__self_0_3));
                        let _ =
                            debug_trait_builder.field("h", &&(*__self_0_4));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for RandrMonitor {
            #[inline]
            fn clone(&self) -> RandrMonitor {
                match *self {
                    RandrMonitor {
                    name: ref __self_0_0,
                    x: ref __self_0_1,
                    y: ref __self_0_2,
                    w: ref __self_0_3,
                    h: ref __self_0_4 } =>
                    RandrMonitor{name:
                                     ::core::clone::Clone::clone(&(*__self_0_0)),
                                 x:
                                     ::core::clone::Clone::clone(&(*__self_0_1)),
                                 y:
                                     ::core::clone::Clone::clone(&(*__self_0_2)),
                                 w:
                                     ::core::clone::Clone::clone(&(*__self_0_3)),
                                 h:
                                     ::core::clone::Clone::clone(&(*__self_0_4)),},
                }
            }
        }
        pub struct I3Workspace {
            pub num: i32,
            pub name: String,
            pub focused: bool,
            pub urgent: bool,
            pub output: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for I3Workspace {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    I3Workspace {
                    num: ref __self_0_0,
                    name: ref __self_0_1,
                    focused: ref __self_0_2,
                    urgent: ref __self_0_3,
                    output: ref __self_0_4 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("I3Workspace");
                        let _ =
                            debug_trait_builder.field("num", &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("name",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("focused",
                                                      &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("urgent",
                                                      &&(*__self_0_3));
                        let _ =
                            debug_trait_builder.field("output",
                                                      &&(*__self_0_4));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for I3Workspace {
            #[inline]
            fn clone(&self) -> I3Workspace {
                match *self {
                    I3Workspace {
                    num: ref __self_0_0,
                    name: ref __self_0_1,
                    focused: ref __self_0_2,
                    urgent: ref __self_0_3,
                    output: ref __self_0_4 } =>
                    I3Workspace{num:
                                    ::core::clone::Clone::clone(&(*__self_0_0)),
                                name:
                                    ::core::clone::Clone::clone(&(*__self_0_1)),
                                focused:
                                    ::core::clone::Clone::clone(&(*__self_0_2)),
                                urgent:
                                    ::core::clone::Clone::clone(&(*__self_0_3)),
                                output:
                                    ::core::clone::Clone::clone(&(*__self_0_4)),},
                }
            }
        }
        fn get_workspaces_event(i3_conn: &mut i3ipc::I3Connection)
         -> super::i3::Msg {
            let workspaces =
                i3_conn.get_workspaces().expect("i3_thread get_workspaces").workspaces;
            let workspaces: Vec<I3Workspace> =
                workspaces.iter().map(|ws|
                                          {
                                              let num = ws.num;
                                              let name = ws.name.clone();
                                              let focused = ws.focused;
                                              let urgent = ws.urgent;
                                              let output = ws.output.clone();
                                              I3Workspace{num,
                                                          name,
                                                          focused,
                                                          urgent,
                                                          output,}
                                          }).collect();
            super::i3::Msg::UpdateWorkspaces(workspaces)
        }
        pub fn run(streams: Vec<relm::EventStream<super::i3::Msg>>) {
            let (_channel, sender) =
                relm::Channel::new(move |msg: super::i3::Msg|
                                       {
                                           for s in streams.iter() {
                                               s.emit(msg.clone());
                                           }
                                       });
            std::thread::spawn(move ||
                                   loop  {
                                       let i3_conn =
                                           i3ipc::I3Connection::connect();
                                       let mut i3_conn =
                                           if let Ok(i3_conn) = i3_conn {
                                               i3_conn
                                           } else {
                                               std::thread::sleep(std::time::Duration::from_secs(1));
                                               continue ;
                                           };
                                       sender.send(get_workspaces_event(&mut i3_conn)).expect("i3_thread sennder");
                                       let mut listener =
                                           i3ipc::I3EventListener::connect().expect("i3_thread listener");
                                       let subs =
                                           [i3ipc::Subscription::Mode,
                                            i3ipc::Subscription::Workspace];
                                       listener.subscribe(&subs).expect("i3_thread subscribe");
                                       for event in listener.listen() {
                                           use i3ipc::event::Event;
                                           let event =
                                               if let Some(event) = event.ok()
                                                  {
                                                   event
                                               } else { break ; };
                                           let event =
                                               match event {
                                                   Event::WorkspaceEvent { ..
                                                   } =>
                                                   get_workspaces_event(&mut i3_conn),
                                                   Event::ModeEvent(info) =>
                                                   super::i3::Msg::UpdateMode(info.change),
                                                   _ => continue ,
                                               };
                                           sender.send(event).expect("i3_thread sennder");
                                       }
                                       std::thread::sleep(std::time::Duration::from_secs(1));
                                   });
        }
    }
    pub use i3::*;
}
mod mpris {
    mod mpris {
        use gtk::prelude::*;
        use gtk::Inhibit;
        use relm::Widget;
        use relm_derive::{widget, Msg};
        pub struct Model {
            volume: String,
        }
        pub enum Msg {
            PlayersList(String),
            PausePlay,
            Status(mpris::PlaybackStatus),
        }
        impl ::relm::DisplayVariant for Msg {
            #[allow(unused_qualifications)]
            fn display_variant(&self) -> &'static str {
                match *self {
                    Msg::PlayersList { .. } => "PlayersList",
                    Msg::PausePlay { .. } => "PausePlay",
                    Msg::Status { .. } => "Status",
                }
            }
        }
        impl ::relm::IntoOption<Msg> for Msg {
            fn into_option(self) -> Option<Msg> { Some(self) }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Msg {
            #[inline]
            fn clone(&self) -> Msg {
                match (&*self,) {
                    (&Msg::PlayersList(ref __self_0),) =>
                    Msg::PlayersList(::core::clone::Clone::clone(&(*__self_0))),
                    (&Msg::PausePlay,) => Msg::PausePlay,
                    (&Msg::Status(ref __self_0),) =>
                    Msg::Status(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[allow(dead_code, missing_docs)]
        pub struct Mpris {
            gtk_label: gtk::Label,
            gtkeventbox4: gtk::EventBox,
            model: Model,
        }
        pub struct __MprisWidgets {
            pub gtk_label: gtk::Label,
            pub gtkeventbox4: gtk::EventBox,
        }
        impl Widget for Mpris {
            #[allow(unused_variables)]
            fn view(relm: &::relm::Relm<Self>, __relm_model: Self::Model)
             -> Self {
                let gtkeventbox4: gtk::EventBox =
                    unsafe {
                        if !gtk::is_initialized_main_thread() {
                            if gtk::is_initialized() {
                                {
                                    ::std::rt::begin_panic("GTK may only be used from the main thread.")
                                };
                            } else {
                                {
                                    ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                                };
                            }
                        }
                        use relm::StaticType;
                        use relm::{Cast, FromGlibPtrNone};
                        let values: &[::relm::Value] = &[];
                        let mut parameters = [];
                        ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::EventBox::static_type()),
                                                                            0u32,
                                                                            parameters.as_mut_ptr())
                                                          as
                                                          *mut _).downcast().unwrap()
                    };
                let gtk_label: gtk::Label =
                    unsafe {
                        if !gtk::is_initialized_main_thread() {
                            if gtk::is_initialized() {
                                {
                                    ::std::rt::begin_panic("GTK may only be used from the main thread.")
                                };
                            } else {
                                {
                                    ::std::rt::begin_panic("GTK has not been initialized. Call `gtk::init` first.")
                                };
                            }
                        }
                        use relm::StaticType;
                        use relm::{Cast, FromGlibPtrNone};
                        let values: &[::relm::Value] = &[];
                        let mut parameters = [];
                        ::gtk::Widget::from_glib_none(::relm::g_object_newv(::relm::ToGlib::to_glib(&gtk::Label::static_type()),
                                                                            0u32,
                                                                            parameters.as_mut_ptr())
                                                          as
                                                          *mut _).downcast().unwrap()
                    };
                gtk_label.set_text(&{
                                        let model = &__relm_model;
                                        model
                                    }.volume);
                gtk_label.set_widget_name("mpris");
                ::gtk::ContainerExt::add(&gtkeventbox4, &gtk_label);
                ::gtk::WidgetExt::show(&gtk_label);
                ::gtk::WidgetExt::show(&gtkeventbox4);
                {
                    {
                        {
                            let stream = relm.stream().clone();
                            let _ =
                                gtkeventbox4.connect_button_press_event(move
                                                                            |_,
                                                                             _|
                                                                            {
                                                                                let (msg,
                                                                                     return_value) =
                                                                                    ::relm::IntoPair::into_pair((Msg::PausePlay,
                                                                                                                 Inhibit(false)));
                                                                                let msg:
                                                                                        Option<_> =
                                                                                    ::relm::IntoOption::into_option(msg);
                                                                                if let Some(msg)
                                                                                       =
                                                                                       msg
                                                                                   {
                                                                                    stream.emit(msg);
                                                                                }
                                                                                return_value
                                                                            });
                        };
                    };
                }
                Mpris{gtkeventbox4: gtkeventbox4,
                      gtk_label: gtk_label,
                      model: __relm_model,}
            }
            type
            Root
            =
            gtk::EventBox;
            fn root(&self) -> Self::Root { self.gtkeventbox4.clone() }
        }
        impl ::relm::Update for Mpris {
            type
            Msg
            =
            Msg;
            type
            Model
            =
            Model;
            type
            ModelParam
            =
            ();
            fn update(&mut self, event: Msg) {
                match event {
                    Msg::PlayersList(s) => {
                        self.model.volume = s;
                        self.gtk_label.set_text(&self.model.volume);
                    }
                    Msg::PausePlay => { }
                    Msg::Status(status) => {
                        let ctx = self.gtk_label.get_style_context();
                        match status {
                            mpris::PlaybackStatus::Playing => {
                                ctx.add_class("playing");
                            }
                            _ => { ctx.remove_class("playing"); }
                        }
                    }
                }
            }
            fn model(_: &::relm::Relm<Self>, _: ()) -> Model {
                Model{volume: "0".into(),}
            }
        }
        impl ::relm::WidgetTest for Mpris {
            type
            Widgets
            =
            __MprisWidgets;
            fn get_widgets(&self) -> __MprisWidgets {
                __MprisWidgets{gtk_label: self.gtk_label.clone(),
                               gtkeventbox4: self.gtkeventbox4.clone(),}
            }
        }
        impl Mpris { }
    }
    pub mod mpris_thread {
        enum MpscEvent { PausePlay, }
        pub fn run(streams: Vec<relm::EventStream<super::mpris::Msg>>) {
            use std::sync::mpsc;
            let (tx, rx) = mpsc::channel();
            for s in streams.iter() {
                let tx = tx.clone();
                s.observe(move |msg|
                              match msg {
                                  super::mpris::Msg::PausePlay => {
                                      let _ = tx.send(MpscEvent::PausePlay);
                                  }
                                  _ => { }
                              });
            }
            let (_channel, sender) =
                relm::Channel::new(move |msg: super::mpris::Msg|
                                       {
                                           for s in streams.iter() {
                                               s.emit(msg.clone());
                                           }
                                       });
            std::thread::spawn(move ||
                                   {
                                       sender.send(super::mpris::Msg::PlayersList("None".into())).expect("mpris_thread send");
                                       let finder =
                                           mpris::PlayerFinder::new().expect("Could not connect to D-Bus");
                                       let mut active_player:
                                               Option<mpris::Player> =
                                           finder.find_active().ok();
                                       loop  {
                                           if let Some(ap) = &active_player {
                                               if !ap.is_running() {
                                                   active_player = None;
                                               }
                                           }
                                           if let Some(event) =
                                                  rx.try_recv().ok() {
                                               match event {
                                                   MpscEvent::PausePlay => {
                                                       if let Some(active_player)
                                                              = &active_player
                                                          {
                                                           let _ =
                                                               active_player.play_pause();
                                                       }
                                                   }
                                               }
                                           }
                                           if let Some(player) =
                                                  &active_player {
                                               let status =
                                                   player.get_playback_status();
                                               if let Ok(status) = status {
                                                   sender.send(super::mpris::Msg::Status(status)).expect("mpris_thread send");
                                                   sender.send(super::mpris::Msg::PlayersList(player.identity().to_string()
                                                                                                  +
                                                                                                  &" : ".to_string()
                                                                                                  +
                                                                                                  &{
                                                                                                       let res =
                                                                                                           ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
                                                                                                                                                               &match (&status,)
                                                                                                                                                                    {
                                                                                                                                                                    (arg0,)
                                                                                                                                                                    =>
                                                                                                                                                                    [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                  ::core::fmt::Debug::fmt)],
                                                                                                                                                                }));
                                                                                                       res
                                                                                                   })).expect("mpris_thread send");
                                               } else {
                                                   active_player =
                                                       finder.find_active().ok();
                                               };
                                           } else {
                                               sender.send(super::mpris::Msg::PlayersList("None".into())).expect("mpris_thread send");
                                               active_player =
                                                   finder.find_active().ok();
                                           }
                                           std::thread::sleep(std::time::Duration::from_millis(100));
                                       }
                                   });
        }
    }
    pub use self::mpris::*;
}
use bar::Bar;
pub enum ModuleComponent {
    Clock(Component<crate::clock::Clock>),
    I3(Component<crate::i3::I3>),
    Alsa(Component<crate::alsa::Alsa>),
    Mpris(Component<crate::mpris::Mpris>),
    Cpu(Component<crate::cpu::Cpu>),
}
impl ModuleComponent {
    fn widget(&self) -> gtk::Widget {
        match self {
            ModuleComponent::Clock(m) =>
            m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::I3(m) =>
            m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Alsa(m) =>
            m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Mpris(m) =>
            m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Cpu(m) =>
            m.widget().clone().upcast::<gtk::Widget>(),
        }
    }
}
use downcast_rs::{impl_downcast, Downcast};
trait Base: Downcast { }
impl dyn Base<> {
    /// Returns true if the trait object wraps an object of type `__T`.
    #[inline]
    pub fn is<__T: Base<>>(&self) -> bool {
        ::downcast_rs::Downcast::as_any(self).is::<__T>()
    }
    /// Returns a boxed object from a boxed trait object if the underlying object is of type
    /// `__T`. Returns the original boxed trait if it isn't.
    #[inline]
    pub fn downcast<__T: Base<>>(self: ::std::boxed::Box<Self>)
     ->
         ::std::result::Result<::std::boxed::Box<__T>,
                               ::std::boxed::Box<Self>> {
        if self.is::<__T>() {
            Ok(::downcast_rs::Downcast::into_any(self).downcast::<__T>().unwrap())
        } else { Err(self) }
    }
    /// Returns an `Rc`-ed object from an `Rc`-ed trait object if the underlying object is of
    /// type `__T`. Returns the original `Rc`-ed trait if it isn't.
    #[inline]
    pub fn downcast_rc<__T: Base<>>(self: ::std::rc::Rc<Self>)
     -> ::std::result::Result<::std::rc::Rc<__T>, ::std::rc::Rc<Self>> {
        if self.is::<__T>() {
            Ok(::downcast_rs::Downcast::into_any_rc(self).downcast::<__T>().unwrap())
        } else { Err(self) }
    }
    /// Returns a reference to the object within the trait object if it is of type `__T`, or
    /// `None` if it isn't.
    #[inline]
    pub fn downcast_ref<__T: Base<>>(&self) -> ::std::option::Option<&__T> {
        ::downcast_rs::Downcast::as_any(self).downcast_ref::<__T>()
    }
    /// Returns a mutable reference to the object within the trait object if it is of type
    /// `__T`, or `None` if it isn't.
    #[inline]
    pub fn downcast_mut<__T: Base<>>(&mut self)
     -> ::std::option::Option<&mut __T> {
        ::downcast_rs::Downcast::as_any_mut(self).downcast_mut::<__T>()
    }
}
macro_rules! module_get(($ e : expr, $ p : path) =>
                        (if let $ p (m) = $ e { Some (m) } else { None }))
macro_rules! thread_run(($ run : path, $ module : path, $ bars : expr) =>
                        ({
                             let streams : Vec < _ > = $ bars . iter () .
                             flat_map
                             (| m | m . modules_left . iter () . chain
                              (m . modules_right . iter ())) . filter_map
                             (| m | module_get ! (m, $ module)) . map
                             (| m | m . stream () . to_owned ()) . collect ()
                             ; if streams . len () > 0 { $ run (streams) ; }
                         }))
fn main() {
    gtk::init().unwrap();
    {
        let stylesheet =
            b"* {\n  /* all: unset; */\n  border: none;\n  border-radius: 0;\n  /* font-family: Roboto, Helvetica, Arial, sans-serif; */\n  font-size: 13px;\n  font-weight: 600;\n  min-height: 0;\n}\n\n#clock,\n#pulse,\n#mpris\n{\n  font-size: 15px;\n\n  padding: 0px 10px;\n  margin: 5px 4px;\n  color: #ffffff;\n  border-radius: 10px;\n}\n#clock {\n  background-color: #ff0066;\n}\n#pulse {\n  background-color: #2ecc71;\n  color: black;\n  font-weight: 500;\n  transition-property: background-color, color;\n  transition-duration: 0.5s;\n}\n#pulse.muted{\n  background-color: #90b1b1;\n}\n\n#mpris{\n  background-color: #90b1b1;\n  color: #ffffff;\n  transition-property: background-color, color;\n  transition-duration: 0.5s;\n}\n#mpris.playing{\n  background-color: #25a75b;\n}\n#cpu{\n  /* background-color: #ff0066; */\n  /* color: #a80043; */\n  margin: 5px 4px;\n  color: #424242;\n}\n\n\nwindow,#main_box{\n  background: rgba(35, 41, 47, 0.5);\n  color: #ffffff;\n}\n\n\n#i3 #mode{\n  margin-left: 15px;\n}\n#i3 button {\n  background: unset;\n  /* padding: 5px 20px; */\n  padding: 0 15px;\n  background-color: transparent;\n  color: #ffffff;\n  border-bottom: 4px solid rgba(255, 255, 255, 0.2);\n}\n#i3 button.focused {\n  border-bottom: 4px solid rgba(255, 255, 255, 0.8);\n}\n#i3 button.urgent {\n  border-bottom: 4px solid rgb(226, 51, 51);\n}\n#i3 button:hover {\n  background: rgba(0, 0, 0, 0.2);\n  box-shadow: inherit;\n  border-bottom: 4px solid #ffffff;\n}\n#i3 button:focus {\n  background-color: rgb(32, 32, 32);\n}";
        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(stylesheet).unwrap();
        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(),
                                                   &style_provider,
                                                   gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
    let bars =
        {
            let config = config::get_config();
            let config_bars = config.bars;
            let mut bars = Vec::new();
            for config_bar in config_bars {
                fn m(module: &config::Module, config_bar: &config::Bar,
                     vec: &mut Vec<ModuleComponent>) {
                    vec.push(match module {
                                 config::Module::Clock => {
                                     ModuleComponent::Clock(relm::init::<crate::clock::Clock>(()).unwrap())
                                 }
                                 config::Module::I3 =>
                                 ModuleComponent::I3(relm::init::<crate::i3::I3>(config_bar.monitor.clone()).unwrap()),
                                 config::Module::Alsa => {
                                     ModuleComponent::Alsa(relm::init::<crate::alsa::Alsa>(()).unwrap())
                                 }
                                 config::Module::Mpris => {
                                     ModuleComponent::Mpris(relm::init::<crate::mpris::Mpris>(()).unwrap())
                                 }
                                 config::Module::Cpu => {
                                     ModuleComponent::Cpu(relm::init::<crate::cpu::Cpu>(()).unwrap())
                                 }
                             });
                }
                let mut modules_left = Vec::new();
                for module in &config_bar.modules_left {
                    m(module, &config_bar, &mut modules_left);
                }
                let mut modules_right = Vec::new();
                for module in &config_bar.modules_right {
                    m(module, &config_bar, &mut modules_right);
                }
                bars.push(bar::ModelParam{bar_name: config_bar.name,
                                          monitor_name:
                                              config_bar.monitor.clone(),
                                          x: config_bar.pos_x,
                                          y: config_bar.pos_y,
                                          modules_left,
                                          modules_right,});
            }
            bars
        };
    {
        let streams: Vec<_> =
            bars.iter().flat_map(|m|
                                     m.modules_left.iter().chain(m.modules_right.iter())).filter_map(|m|
                                                                                                         if let ModuleComponent::Alsa(m)
                                                                                                                =
                                                                                                                m
                                                                                                            {
                                                                                                             Some(m)
                                                                                                         } else {
                                                                                                             None
                                                                                                         }).map(|m|
                                                                                                                    m.stream().to_owned()).collect();
        if streams.len() > 0 { alsa::alsa_thread::run(streams); }
    };
    {
        let streams: Vec<_> =
            bars.iter().flat_map(|m|
                                     m.modules_left.iter().chain(m.modules_right.iter())).filter_map(|m|
                                                                                                         if let ModuleComponent::I3(m)
                                                                                                                =
                                                                                                                m
                                                                                                            {
                                                                                                             Some(m)
                                                                                                         } else {
                                                                                                             None
                                                                                                         }).map(|m|
                                                                                                                    m.stream().to_owned()).collect();
        if streams.len() > 0 { i3::i3_thread::run(streams); }
    };
    {
        let streams: Vec<_> =
            bars.iter().flat_map(|m|
                                     m.modules_left.iter().chain(m.modules_right.iter())).filter_map(|m|
                                                                                                         if let ModuleComponent::Mpris(m)
                                                                                                                =
                                                                                                                m
                                                                                                            {
                                                                                                             Some(m)
                                                                                                         } else {
                                                                                                             None
                                                                                                         }).map(|m|
                                                                                                                    m.stream().to_owned()).collect();
        if streams.len() > 0 { mpris::mpris_thread::run(streams); }
    };
    {
        let streams: Vec<_> =
            bars.iter().flat_map(|m|
                                     m.modules_left.iter().chain(m.modules_right.iter())).filter_map(|m|
                                                                                                         if let ModuleComponent::Cpu(m)
                                                                                                                =
                                                                                                                m
                                                                                                            {
                                                                                                             Some(m)
                                                                                                         } else {
                                                                                                             None
                                                                                                         }).map(|m|
                                                                                                                    m.stream().to_owned()).collect();
        if streams.len() > 0 { cpu::cpu_thread::run(streams); }
    };
    bars.into_iter().for_each(|m| { let _ = relm::init::<Bar>(m).unwrap(); });
    gtk::main();
}
