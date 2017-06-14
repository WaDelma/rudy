use ::util::locksteparray::LockstepArray;
use ::util::locksteparray;
use super::innerptr::{InnerPtr, IntoPtr};
use ::Key;
use super::traits::JpmNode;
use ::rudymap::results::InsertResult;
use super::branch_bitmap::BranchBitmap;
use std::iter::FromIterator;

pub struct BranchLinear<K: Key, V> {
    array: LockstepArray<[u8; 7], [InnerPtr<K, V>; 7]>
}

impl<K: Key, V> BranchLinear<K, V> {
    pub fn new() -> BranchLinear<K, V> {
        BranchLinear {
            array: Default::default()
        }
    }
}

impl<K: Key, V> JpmNode<K, V> for BranchLinear<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        let (byte, subkey) = key.split_first().unwrap();
        self.array.array1()
            .binary_search(byte)
            .ok()
            .and_then(|index| self.array.array2()[index].get(subkey))
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        let (byte, subkey) = key.split_first().unwrap();
        self.array.array1_mut()
            .binary_search(byte)
            .ok()
            .and_then(move |index|
                      self.array.array2_mut()[index].get_mut(subkey))
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        let (&byte, subkey) = key.split_first().unwrap();
        match self.array.array1().binary_search(&byte) {
            Ok(found) => {
                InsertResult::Success(
                    self.array.array2_mut()[found].insert(subkey, value))
            },
            Err(insert) => {
                match self.array.insert(insert, byte, InnerPtr::empty()) {
                    Ok(()) => {},
                    Err(locksteparray::InsertError::Overflow(..)) => {
                        return InsertResult::Resize(value);
                    },
                    Err(locksteparray::InsertError::OutOfBounds(..)) => {
                        unreachable!()
                    }
                }
                let node = &mut self.array.array2_mut()[insert];
                InsertResult::Success(node.insert(subkey, value))
            }
        }
    }

    fn expand(self: Box<Self>, pop: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        let mut branch: BranchBitmap<K, V> = self.array
            .into_iter()
            .collect();
        branch.insert(key, value).success();
        IntoPtr::into_ptr(Box::new(branch), pop)
    }
}

impl<K: Key, V> FromIterator<(u8, InnerPtr<K, V>)> for BranchLinear<K, V> {
    fn from_iter<I>(iter: I) -> BranchLinear<K, V>
        where I: IntoIterator<Item=(u8, InnerPtr<K, V>)> {
        let mut node = BranchLinear::new();
        for (k, v) in iter {
            node.array.push(k, v);
        }
        node
    }
}
