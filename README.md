# dfb

This crate defines and provides an interface for this type: 
```rust 
Dfb(HashMap<TypeId, VecDeque<Box<dyn Any>>>)
```
Most of its basic methods are simply passthroughs to the HashMap equivalents, the more interesting ones are these:

`insert`: Accepts just a value. The TypeId of its type will be used as the key. If one or more values of this type already exist in the map, this will push a new value into the FIFO that contains them. If no value exists, this will create a new FIFO containing the inserted element.

`remove`: Uses a generic type rather than a key parameter to determine what to remove. Returns and removes the earliest inserted element of this type if it exists. If the element returned was the last remaining element of its type, the internal FIFO for this type is deleted.

While the above two methods represent the target API of this crate, some lower-level interfaces are provided as well:

`insert_dyn` works like `insert` but takes a `Box<dyn Any>` and places it in the correct location based on the TypeId of its contained type.

`remove_all` removes an entire VecDeque based on the specified type (this is the true pass-through into the HashMap equivalent `remove`). The type returned is:

```rust
Option<VecDeque<DynBox<T>>> 
```

`get` and `get_mut`: Returns the entire VecDeque for a particular type, if it exists. The return type will be respectively:
```rust
Option<&VecDeque<DynBox<T>>> 
```
or
```rust
Option<&mut VecDeque<DynBox<T>>> 
```
The `DynBox` type is a necessary wrapper that allows an internally stored `Box<dyn Any>` to be transmuted to a `Box<T>` without mangling alignment in the VecDeque. It can easily be converted into a concrete `Box<T>` by calling DynBox::unwrap, and also implements a few useful traits for getting at its insides (Deref, DerefMut, etc).

Various trait implementations also exist that to mimic the HashMap interfaces, but most can only provide access to the true `VecDeque<Box<dyn Any>>` stored in the map.

The `dfb!(expr*)` macro can be used to quickly initialize a dfb of arbitrary elements of any type.