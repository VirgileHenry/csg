use crate::{
    object::Object,
    operations::{Op, union::Union},
    csg_primitives::{Primitive, sphere::CsgSphere}
};

fn assert_object_is_serialize_json_consistent(obj: Object) {
    let before_serialize_debug_str = format!("{obj:?}");
    
    let json = serde_json::to_value(obj).unwrap();
    let serialized_once_debug_str = format!("{json:?}");
    
    let obj = serde_json::from_value::<Object>(json).unwrap();
    let after_serialize_debug_str = format!("{obj:?}");

    let json = serde_json::to_value(obj).unwrap();
    let serialized_twice_debug_str = format!("{json:?}");

    // test the object is consistent across multiple serialization / deserialization
    assert_eq!(before_serialize_debug_str, after_serialize_debug_str);
    assert_eq!(serialized_once_debug_str, serialized_twice_debug_str);
}

fn assert_object_is_serialize_bin_consistent(obj: Object) {
    let before_serialize_debug_str = format!("{obj:?}");
    
    let serialized_once_bin = serde_binary::to_vec(&obj, Default::default()).unwrap();

    let obj = serde_binary::from_slice::<Object>(&serialized_once_bin, Default::default()).unwrap();
    let after_serialize_debug_str = format!("{obj:?}");

    let serialized_twice_bin = serde_binary::to_vec(&obj, Default::default()).unwrap();

    // test the object is consistent across multiple serialization / deserialization
    assert_eq!(before_serialize_debug_str, after_serialize_debug_str);
    assert_eq!(serialized_once_bin, serialized_twice_bin);
}

#[test]
fn serialize_json() {
    let obj = Object::Operation(Op::Union(Union::new(vec![
        Object::Primitive(Primitive::Sphere(CsgSphere::centered(3.))),
        Object::Primitive(Primitive::Sphere(CsgSphere::centered(1.))),
        Object::Primitive(Primitive::Sphere(CsgSphere::centered(2.))),
    ])));

    assert_object_is_serialize_json_consistent(obj);
}

#[test]
fn serialize_binary() {
    let obj = Object::Operation(Op::Union(Union::new(vec![
        Object::Primitive(Primitive::Sphere(CsgSphere::centered(3.))),
        Object::Primitive(Primitive::Sphere(CsgSphere::centered(1.))),
        Object::Primitive(Primitive::Sphere(CsgSphere::centered(2.))),
    ])));

    assert_object_is_serialize_bin_consistent(obj);
}

