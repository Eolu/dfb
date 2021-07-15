#[cfg(test)]
mod test;

use core::any::*;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::ops::Deref;
use std::ops::DerefMut;

/// A macro to create a Dfb containing passed-in elements.
#[macro_export]
macro_rules! dfb
{
    () => { $crate::Dfb::new() };
    ($($item:expr),*) => { $crate::Dfb::from([$(Box::new($item) as Box<dyn std::any::Any>),*]) }
}

/// An "anymap" which uses TypeIDs as keys and VecDeques of that type as values.
#[derive(Debug)]
pub struct Dfb(HashMap<TypeId, VecDeque<Box<dyn Any>>>);

impl Dfb 
{
    /// Creates a Dfb backed by [HashMap::new]
    #[inline]
    pub fn new() -> Self
    {
        Dfb(HashMap::new())
    }

    /// Creates a Dfb backed by [HashMap::with_capacity]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self
    {
        Dfb(HashMap::with_capacity(capacity))
    }
    
    /// Wrapper for [HashMap::capacity]
    #[inline]
    pub fn capacity(&self) -> usize
    {
        self.0.capacity()
    }

    /// Wrapper for [HashMap::keys]
    #[inline]
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, TypeId, VecDeque<Box<dyn Any>>>
    {
        self.0.keys()
    }

    /// Wrapper for [HashMap::values]
    #[inline]
    pub fn values(&self) -> std::collections::hash_map::Values<'_, TypeId, VecDeque<Box<dyn Any>>>
    {
        self.0.values()
    }

    /// Wrapper for [HashMap::values_mut]
    #[inline]
    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, TypeId, VecDeque<Box<dyn Any>>>
    {
        self.0.values_mut()
    }
    
    /// Wrapper for [HashMap::iter]
    #[inline]
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, TypeId, VecDeque<Box<dyn Any>>>
    {
        self.0.iter()
    }

    /// Wrapper for [HashMap::iter_mut]
    #[inline]
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, TypeId, VecDeque<Box<dyn Any>>>
    {
        self.0.iter_mut()
    }

    /// Wrapper for [HashMap::len]
    #[inline]
    pub fn len(&self) -> usize
    {
        self.0.len()
    }

    /// Wrapper for [HashMap::is_empty]
    #[inline]
    pub fn is_empty(&self) -> bool
    {
        self.0.is_empty()
    }

    /// Wrapper for [HashMap::drain]
    #[inline]
    pub fn drain(&mut self) -> std::collections::hash_map::Drain<'_, TypeId, VecDeque<Box<dyn Any>>>
    {
        self.0.drain()
    }

    /// Wrapper for [HashMap::clear]
    #[inline]
    pub fn clear(&mut self)
    {
        self.0.clear()
    }

    /// Wrapper for [HashMap::reserve]
    #[inline]
    pub fn reserve(&mut self, additional: usize)
    {
        self.0.reserve(additional)
    }

    /// Wrapper for [HashMap::shrink_to_fit]
    #[inline]
    pub fn shrink_to_fit(&mut self)
    {
        self.0.shrink_to_fit()
    }

    /// Generic wrapper for [HashMap::entry]
    #[inline]
    pub fn entry<T: 'static>(&mut self) -> std::collections::hash_map::Entry<'_, TypeId, VecDeque<Box<dyn Any>>>
    {
        self.0.entry(TypeId::of::<T>())
    }

    /// Generic wrapper for [HashMap::get]. Performs a transmute under the
    /// hood to treat a boxed trait object as a concrete type. Returns the 
    /// entire VecDeque for type T. 
    pub fn get<T: Any>(&self) -> Option<&VecDeque<DynBox<T>>> 
    {
        unsafe
        {
            std::mem::transmute::
            <
                Option<&VecDeque<Box<dyn Any>>>, 
                Option<&VecDeque<DynBox<T>>>
            >
            (self.0.get(&TypeId::of::<T>()))
        }
    }

    /// Generic wrapper for [HashMap::get_mut]. Performs a transmute under the
    /// hood to treat a boxed trait object as a concrete type. Returns the 
    /// entire VecDeque for type T. 
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut VecDeque<DynBox<T>>> 
    {
        unsafe
        {
            std::mem::transmute::
            <
                Option<&mut VecDeque<Box<dyn Any>>>, 
                Option<&mut VecDeque<DynBox<T>>>
            >
            (self.0.get_mut(&TypeId::of::<T>()))
        }
    }

    /// Generic wrapper for [HashMap::get_key_value]
    #[inline]
    pub fn contains<T: Any>(&self) -> bool
    {
        self.0.contains_key(&TypeId::of::<T>())
    }

    /// Generic wrapper for [HashMap::insert]. If one or more values of this 
    /// type already exist in the map, this will push a new value into the FIFO
    /// that contains them. If no value exists, this will create a new FIFO 
    /// containing the inserted element.
    pub fn insert<T: Any>(&mut self, value: T) 
    {
        let type_id = value.type_id();
        match self.0.get_mut(&type_id)
        {
            Some(vec) => vec.push_back(Box::new(value)),
            None => 
            {
                let mut vec: VecDeque<Box<dyn Any>> = VecDeque::new();
                vec.push_back(Box::new(value));
                self.0.insert(type_id, vec);
            }
        }
    }

    /// Like [Dfb::insert], but allows values of unknown type.
    pub fn insert_dyn(&mut self, value: Box<dyn Any>)
    {
        let type_id = value.as_ref().type_id();
        match self.0.get_mut(&type_id)
        {
            Some(vec) => vec.push_back(value),
            None => 
            {
                let mut vec: VecDeque<Box<dyn Any>> = VecDeque::new();
                vec.push_back(value);
                self.0.insert(type_id, vec);
            }
        }
    }

    /// Generic wrapper for [HashMap::remove]. Returns and removes the earliest 
    /// inserted element of this type if it exists. If the element returned was 
    /// the last remaining element of its type, the internal FIFO for this type 
    /// is deleted.
    pub fn remove<T: Any>(&mut self) -> Option<T>
    {
        match self.get_mut::<T>()
        {
            Some(vec) => 
            {
                let result = vec.pop_front();
                if vec.is_empty()
                {
                    self.0.remove(&TypeId::of::<T>());
                }
                result.map(|b|*b.1)
            },
            None => None,
        }
    }

    /// Wrapper for [HashMap::retain]
    #[inline]
    pub fn retain<F: FnMut(&TypeId, &mut VecDeque<Box<dyn Any>>) -> bool>(&mut self, f: F) 
    {
        self.0.retain(f)
    }
}

impl FromIterator<Box<dyn Any>> for Dfb
{
    fn from_iter<T: IntoIterator<Item = Box<dyn Any>>>(iter: T) -> Self 
    {
        let mut data: HashMap<TypeId, VecDeque<Box<dyn Any>>> = HashMap::new();
        for value in iter
        {
            let type_id = (*value).type_id();
            match data.get_mut(&type_id)
            {
                Some(vec) => vec.push_back(value),
                None => 
                {
                    let mut vec: VecDeque<Box<dyn Any>> = VecDeque::new();
                    vec.push_back(value);
                    data.insert(type_id, vec);
                }
            }
        }
        Dfb(data)
    }
}

impl Default for Dfb
{
    fn default() -> Self
    {
        Dfb::new()
    }
}

impl Extend<(TypeId, VecDeque<Box<dyn Any>>)> for Dfb
{
    fn extend<T: IntoIterator<Item = (TypeId, VecDeque<Box<dyn Any>>)>>(&mut self, iter: T) 
    {
        self.0.extend(iter)
    }
}

impl<const N: usize> From<[Box<dyn Any>; N]> for Dfb
{
    fn from(items: [Box<dyn Any>; N]) -> Self 
    {
        let mut data: HashMap<TypeId, VecDeque<Box<dyn Any>>> = HashMap::with_capacity(N);
        for value in std::array::IntoIter::new(items)
        {
            let type_id = (*value).type_id();
            match data.get_mut(&type_id)
            {
                Some(vec) => vec.push_back(value),
                None => 
                {
                    let mut vec: VecDeque<Box<dyn Any>> = VecDeque::new();
                    vec.push_back(value);
                    data.insert(type_id, vec);
                }
            }
        }
        Dfb(data)
    }
}

impl std::iter::FromIterator<(TypeId, VecDeque<Box<dyn Any>>)> for Dfb
{
    fn from_iter<T: IntoIterator<Item = (TypeId, VecDeque<Box<dyn Any>>)>>(iter: T) -> Self 
    {
        let mut collection = Dfb::new();
        collection.extend(iter);
        collection
    }
}

impl std::ops::Index<TypeId> for Dfb
{
    type Output = VecDeque<Box<dyn Any>>;

    fn index(&self, key: TypeId) -> &Self::Output 
    {
        self.0.get(&key).expect("no instance of type found")
    }
}

impl IntoIterator for Dfb
{
    type Item = (TypeId, VecDeque<Box<dyn Any>>);
    type IntoIter = std::collections::hash_map::IntoIter<TypeId, VecDeque<Box<dyn Any>>>;

    fn into_iter(self) -> Self::IntoIter 
    {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Dfb
{
    type Item = (&'a TypeId, &'a VecDeque<Box<dyn Any>>);
    type IntoIter = std::collections::hash_map::Iter<'a, TypeId, VecDeque<Box<dyn Any>>>;

    fn into_iter(self) -> Self::IntoIter 
    {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Dfb
{
    type Item = (&'a TypeId, &'a mut VecDeque<Box<dyn Any>>);
    type IntoIter = std::collections::hash_map::IterMut<'a, TypeId, VecDeque<Box<dyn Any>>>;

    fn into_iter(self) -> Self::IntoIter 
    {
        self.0.iter_mut()
    }
}

/// Wraps a Box, necessary for transmuting a boxed trait object.
#[derive(Debug, Clone)]
pub struct DynBox<T: Any + ?Sized>(u8, pub Box<T>);

impl<T: Any + ?Sized> DynBox<T>
{
    pub fn unwrap(self) -> Box<T>
    {
        self.1
    }
}

impl<T: Any + ?Sized> Deref for DynBox<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target 
    {
        self.1.deref()
    }
}

impl<T: Any + ?Sized> DerefMut for DynBox<T>
{
    fn deref_mut(&mut self) -> &mut T
    {
        self.1.deref_mut()
    }
}

impl<T: Any + ?Sized> AsMut<T> for DynBox<T>
{
    fn as_mut(&mut self) -> &mut T 
    {
        self.1.as_mut()
    }
}

impl<T: Any + ?Sized> AsRef<T> for DynBox<T>
{
    fn as_ref(&self) -> &T 
    {
        self.1.as_ref()
    }
}

impl<T: Any + ?Sized> Borrow<T> for DynBox<T>
{
    fn borrow(&self) -> &T 
    {
        self.1.borrow()
    }
}

impl<T: Any + ?Sized> BorrowMut<T> for DynBox<T>
{
    fn borrow_mut(&mut self) -> &mut T
    {
        self.1.borrow_mut()
    }
}