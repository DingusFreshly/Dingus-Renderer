use std::marker::PhantomData;
use crate::handle::Handle;
use std::convert::{From, Into};
///One item in the slotmap
struct Slot<V> {
    value: Option<V>,
    ///increased each time slot is used
    generation: u32
}
///Generational arena that maps typed handles to gpu resources
pub struct SlotMap<K, V> {
    slots: Vec<Slot<V>>,
    /// recycled indices go here
    free: Vec<usize>,
    _key : PhantomData<K>
}

impl<K, V> SlotMap<K, V>
where K: From<Handle<V>> + Into<Handle<V>> + Copy
{   //
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
            _key : PhantomData
        }
    }
    /// Push the resource into the slot map, returns a typed handle to it
    pub fn insert(&mut self, value: V) -> K {
        if let Some(index) = self.free.pop() {
            //reuse slot
            let mut slot = &mut self.slots[index as usize];
            slot.generation += 1;
            slot.value = Some(value);

            K::from(Handle::new(index, slot.generation))
        } else {
            //append to slot list
            let index = self.slots.len();

            self.slots.push(Slot {
                value: Some(value),
                generation: 0,
            });

            K::from(Handle::new(index, 0))
        }
    }

    pub fn get(& self, key: K) -> Option<&V> {
        let handle : Handle<V> = key.into();

        if handle.is_null() {
            return None;
        }
        let slot = self.slots.get(handle.slot_index)?;

        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_ref()
    }
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let handle : Handle<V> = key.into();

        if handle.is_null() {
            return None;
        }
        let slot = self.slots.get_mut(handle.slot_index)?;

        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_mut()
    }
    pub fn remove(&mut self, key: K) -> Option<V> {
        let handle : Handle<V> = key.into();
        if handle.is_null() { return None; };
        let slot = self.slots.get_mut(handle.slot_index)?;

        if slot.generation != handle.generation { return None; }

        let value = slot.value.take();
        self.free.push(handle.slot_index);
        
        value
    }
    ///get all slots with a value by refeference
    pub fn values(&self) -> impl Iterator<Item = &V> {

        self.slots.iter().filter(|x| {
            x.value.is_some()
        }).map(|x| {
            x.value.as_ref().unwrap()
        })

    }
    ///get all slots with a value by mut reference
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {

        self.slots.iter_mut().filter(|x| {
            x.value.is_some()
        }).map(|x| {
            x.value.as_mut().unwrap()
        })

    }

    pub fn len(&self) -> usize {
        self.slots.len() - self.free.len()
    }
}