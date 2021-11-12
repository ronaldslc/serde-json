use rkyv::{
    archived_root,
    ser::{serializers::AllocSerializer, Serializer},
    Deserialize as RkyvDe, Infallible,
};
use serde_json::{from_str, Value};

#[test]
fn test_json_value_rkyv() {
    // Sorted order
    #[cfg(not(feature = "preserve_order"))]
    const EXPECTED: &[&str] = &["a", "b", "c"];

    // Insertion order
    #[cfg(feature = "preserve_order")]
    const EXPECTED: &[&str] = &["b", "a", "c"];

    let v: Value = from_str(r#"{"b":null,"a":32,"c":"some string"}"#).unwrap();

    let mut serializer = AllocSerializer::<4096>::default();
    serializer.serialize_value(&v).unwrap();
    let bytes = serializer.into_serializer().into_inner();

    let archived = unsafe { archived_root::<Value>(&bytes[..]) };
    let v2: Value = archived.deserialize(&mut Infallible).unwrap();
    assert_eq!(v.get("a"), v2.get("a"));
    assert_eq!(v.get("b"), v2.get("b"));
    assert_eq!(v.get("c"), v2.get("c"));

    let keys: Vec<_> = v.as_object().unwrap().keys().collect();
    assert_eq!(keys, EXPECTED);
}
