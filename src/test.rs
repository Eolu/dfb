use std::any::Any;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
struct DummyStruct1
{
    foo: u32,
    phoo: String
}
#[derive(Debug, Clone, PartialEq)]
struct DummyStruct2
{
    bar: isize,
    barr: &'static [isize]
}
#[derive(Debug, Clone, PartialEq)]
struct SimpleStruct1;
#[derive(Debug, Clone, PartialEq)]
struct SimpleStruct2;

#[test]
fn type_fifo_test()
{
    let mut collection = dfb!();
    let dummy1 = DummyStruct1{ foo: 22, phoo: String::from("I am a string") };
    let dummy2 = DummyStruct2{ bar: -435, barr: &[0, 1, 2, 3, 4, -4000, 6] };
    collection.insert(dummy1.clone());
    collection.insert(dummy2.clone());
    assert_eq!(dummy1, collection.remove::<DummyStruct1>().unwrap());
    assert_eq!(dummy2, collection.remove::<DummyStruct2>().unwrap());
}

#[test]
fn type_fifo_test_2()
{
    let dummy1 = DummyStruct1{ foo: 22, phoo: String::from("I am a string") };
    let dummy2 = DummyStruct1{ foo: 14, phoo: String::from("I am a different string") };
    let dummy3 = DummyStruct2{ bar: -435, barr: &[0, 1, 2, 3, 4, -4000, 6] };
    let mut collection = dfb!
    (
        dummy1.clone(), 
        dummy2.clone(), 
        dummy3.clone()
    );
    assert_eq!(dummy1, collection.remove::<DummyStruct1>().unwrap());
    assert_eq!(dummy2, collection.remove::<DummyStruct1>().unwrap());
    assert_eq!(None, collection.remove::<DummyStruct1>());
    assert_eq!(dummy3, collection.remove::<DummyStruct2>().unwrap());

    let mut collection2 = dfb!(SimpleStruct1, SimpleStruct2);
    assert_eq!(SimpleStruct1, collection2.remove::<SimpleStruct1>().unwrap());
    assert_eq!(None, collection2.remove::<SimpleStruct1>());
    assert_eq!(SimpleStruct2, collection2.remove::<SimpleStruct2>().unwrap());
    assert_eq!(None, collection2.remove::<SimpleStruct2>());
}

#[test]
fn dyn_box_text()
{
    use crate::DynBox;
    use std::mem::size_of;
    assert_eq!
    (
        size_of::<Box<dyn Any>>(),
        size_of::<DynBox<String>>()
    );
}