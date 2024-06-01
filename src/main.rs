use std::collections::HashMap;

// JSON Data types
#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

// Implement From Trait for JSon elements
impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
} 

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
} 

impl From<&str> for Json {
    fn from(s: &str) -> Json {
        Json::String(s.to_string())
    }
} 

// Implement Trait From for all number types through use of another macro
macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

// JSON parser macro
macro_rules! json {
    ( null ) => {
        Json::Null
    };
    ([ $( $element:tt ), * ]) => {
        Json::Array(vec! [ $( json!($element) ), * ])
    };
    ( { $( $key:tt : $value:tt ), * } ) => {
        Json::Object( Box::new(vec! [
            $( ( $key.to_string(), json!( $value )) ), *
        ].into_iter().collect()))
    };
    ( $other:tt ) => {
        Json::from( $other )
    };
}


// Entry point to test program
fn main() {
    println!("Implemented json macro with unittests.  Run 'cargo test' to check!");

    let _res = json! ( null );

    let width = 100;

    let _desc = json! (
        {
            "width" : width,
            "height" : 480.0,
            "elements" : [ 1, 2, 3, 43, 5 ],
            "dummy" : {
                "overview": true
            }
        }
    );

    println!("\r\n{:#?}", _desc);
}


// Unittests for json parser macro
#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_null_json() {
        let res = json!( null );
        assert_eq!(res, Json::Null);
    }

    #[test]
    fn test_empty_object() {
        let res = json!( {} );
        match res {
            Json::Object( a) => {
                assert_eq!((*a).len(), 0 as usize)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_single_element_object() {
        let res = json!( { "test" : 1} );
        match res {
            Json::Object( a) => {
                assert_eq!((*a).len(), 1 as usize);
                let elements = *a;
                let el1 = elements.get("test");
                match el1 {
                    Some( x) =>
                        match x {
                            Json::Number(v) => assert_eq!(v, &1.0_f64),
                            _ => assert!(false)
                        }
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_single_element_object_boolean() {
        let res = json!( { "test" : false} );
        match res {
            Json::Object( a) => {
                assert_eq!((*a).len(), 1 as usize);
                let elements = *a;
                let el1 = elements.get("test");
                match el1 {
                    Some( x) =>
                        match x {
                            Json::Boolean(v) => assert_eq!(v, &false),
                            _ => assert!(false)
                        }
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_single_element_object_string() {
        let res = json!( { "test" : "yes"} );
        match res {
            Json::Object( a) => {
                assert_eq!((*a).len(), 1 as usize);
                let elements = *a;
                let el1 = elements.get("test");
                match el1 {
                    Some( x) =>
                        match x {
                            Json::String(v) => assert_eq!(v, &"yes"),
                            _ => assert!(false)
                        }
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_single_element_object_array() {
        let res = json!( { "test" : [ "yes" ] } );
        match res {
            Json::Object( a) => {
                assert_eq!((*a).len(), 1 as usize);
                let elements = *a;
                let el1 = elements.get("test");
                match el1 {
                    Some( x) =>
                        match x {
                            Json::Array(v) => {
                                assert_eq!((*v).len(), 1);
                                match  v.first() {
                                    Some (e) => assert_eq!( e, &Json::String("yes".to_string()) ),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_single_element_object_object() {
        let res = json!( { "test" : { "tall" : 1.0 } } );
        match res {
            Json::Object( a) => {
                assert_eq!((*a).len(), 1 as usize);
                let elements = *a;
                let el1 = elements.get("test");
                match el1 {
                    Some( x) =>
                        match x {
                            Json::Object(v) => {
                                assert_eq!((*v).len(), 1);
                                match  v.get("tall") {
                                    Some (e) => assert_eq!( e, &Json::Number(1.0) ),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

}
