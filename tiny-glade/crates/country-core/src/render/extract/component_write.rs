//! Producer-side query writes feed the existing bounded component journal.
//! Query access and fetching delegate to Bevy; one record per mutable item,
//! with no extraction-time scan or value comparison.
use super::ComponentJournal;
use bevy_ecs::{
    archetype::Archetype,
    change_detection::{DetectChanges, DetectChangesMut, MaybeLocation, Mut},
    component::{Component, ComponentId, Components, Mutable, Tick},
    entity::Entity,
    query::{FilteredAccess, QueryData, ReadOnlyQueryData, WorldQuery},
    storage::{Table, TableRow},
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};
use core::{marker::PhantomData, ops::{Deref, DerefMut}};

pub struct ComponentWrite<C: Component>(PhantomData<C>);
pub struct ComponentRead<C: Component>(PhantomData<C>);

pub struct ComponentMut<'w, C: Component> {
    value: Mut<'w, C>,
    journal: &'w ComponentJournal<C>,
    entity: Entity,
    recorded: bool,
}
impl<'w, C: Component<Mutability = Mutable>> ComponentMut<'w, C> {
    fn record(&mut self) {
        if !self.recorded { self.journal.record(self.entity); self.recorded = true; }
    }
    pub fn into_inner(mut self) -> &'w mut C { self.record(); self.value.into_inner() }
}
impl<C: Component> Deref for ComponentMut<'_, C> {
    type Target = C;
    fn deref(&self) -> &C { &self.value }
}
impl<C: Component<Mutability = Mutable>> DerefMut for ComponentMut<'_, C> {
    fn deref_mut(&mut self) -> &mut C { self.record(); &mut self.value }
}
impl<C: Component> DetectChanges for ComponentMut<'_, C> {
    fn is_added(&self) -> bool { self.value.is_added() }
    fn is_changed(&self) -> bool { self.value.is_changed() }
    fn last_changed(&self) -> Tick { self.value.last_changed() }
    fn added(&self) -> Tick { self.value.added() }
    fn changed_by(&self) -> MaybeLocation { self.value.changed_by() }
}
impl<C: Component<Mutability = Mutable>> DetectChangesMut for ComponentMut<'_, C> {
    type Inner = C;
    fn set_changed(&mut self) { self.record(); self.value.set_changed(); }
    fn set_added(&mut self) { self.record(); self.value.set_added(); }
    fn set_last_changed(&mut self, tick: Tick) { self.record(); self.value.set_last_changed(tick); }
    fn set_last_added(&mut self, tick: Tick) { self.record(); self.value.set_last_added(tick); }
    fn bypass_change_detection(&mut self) -> &mut C { self.record(); self.value.bypass_change_detection() }
    fn set_if_neq(&mut self, value: C) -> bool where C: PartialEq {
        if *self.value == value { return false; }
        self.record();
        self.value.set_if_neq(value)
    }
    fn replace_if_neq(&mut self, value: C) -> Option<C> where C: PartialEq {
        if *self.value == value { return None; }
        self.record();
        self.value.replace_if_neq(value)
    }
    fn clone_from_if_neq<T>(&mut self, value: &T) -> bool
        where T: ToOwned<Owned = C> + ?Sized, C: PartialEq<T> {
        if &*self.value == value { return false; }
        self.record();
        self.value.clone_from_if_neq(value)
    }
}

// Both variants retain exactly the underlying component access plus a shared
// journal resource read. The caller supplies the same World, archetype/table
// and row invariants to the delegated Bevy implementation.
macro_rules! query {
    ($name:ident, $base:ty) => {
        unsafe impl<C: Component<Mutability = Mutable>> WorldQuery for $name<C> {
            type State = (ComponentId, ComponentId);
            type Fetch<'w> = (<$base as WorldQuery>::Fetch<'w>, &'w ComponentJournal<C>);
            const IS_DENSE: bool = <$base as WorldQuery>::IS_DENSE;
            fn shrink_fetch<'a: 'b, 'b>(fetch: Self::Fetch<'a>) -> Self::Fetch<'b> {
                (<$base as WorldQuery>::shrink_fetch(fetch.0), fetch.1)
            }
            unsafe fn init_fetch<'w>(world: UnsafeWorldCell<'w>, state: &Self::State,
                last: Tick, now: Tick) -> Self::Fetch<'w> {
                unsafe { (<$base as WorldQuery>::init_fetch(world, &state.0, last, now),
                    world.get_resource::<ComponentJournal<C>>().expect("registered component journal")) }
            }
            unsafe fn set_archetype<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State,
                archetype: &'w Archetype, table: &'w Table) {
                unsafe { <$base as WorldQuery>::set_archetype(&mut fetch.0, &state.0, archetype, table); }
            }
            unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &'w Table) {
                unsafe { <$base as WorldQuery>::set_table(&mut fetch.0, &state.0, table); }
            }
            fn update_component_access(state: &Self::State, access: &mut FilteredAccess<ComponentId>) {
                <$base as WorldQuery>::update_component_access(&state.0, access);
                assert!(!access.access().has_resource_write(state.1), "component journal has conflicting write access");
                access.access_mut().add_resource_read(state.1);
            }
            fn init_state(world: &mut World) -> Self::State {
                world.init_resource::<ComponentJournal<C>>();
                (<$base as WorldQuery>::init_state(world), world.components().resource_id::<ComponentJournal<C>>().unwrap())
            }
            fn get_state(components: &Components) -> Option<Self::State> {
                Some((<$base as WorldQuery>::get_state(components)?, components.resource_id::<ComponentJournal<C>>()?))
            }
            fn matches_component_set(state: &Self::State, contains: &impl Fn(ComponentId) -> bool) -> bool {
                <$base as WorldQuery>::matches_component_set(&state.0, contains)
            }
        }
    }
}
query!(ComponentWrite, &'static mut C);
query!(ComponentRead, &'static C);

// Exclusive component access is unchanged from &mut C. The journal itself
// records through its mutex, so disjoint producers can run concurrently.
unsafe impl<C: Component<Mutability = Mutable>> QueryData for ComponentWrite<C> {
    const IS_READ_ONLY: bool = false;
    type ReadOnly = ComponentRead<C>;
    type Item<'w> = ComponentMut<'w, C>;
    fn shrink<'a: 'b, 'b>(item: Self::Item<'a>) -> Self::Item<'b> { item }
    unsafe fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: Entity,
        row: TableRow) -> Self::Item<'w> {
        ComponentMut { value: unsafe { <&mut C as QueryData>::fetch(&mut fetch.0, entity, row) },
            journal: fetch.1, entity, recorded: false }
    }
}
unsafe impl<C: Component<Mutability = Mutable>> QueryData for ComponentRead<C> {
    const IS_READ_ONLY: bool = true;
    type ReadOnly = Self;
    type Item<'w> = &'w C;
    fn shrink<'a: 'b, 'b>(item: Self::Item<'a>) -> Self::Item<'b> { item }
    unsafe fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: Entity,
        row: TableRow) -> Self::Item<'w> {
        unsafe { <&C as QueryData>::fetch(&mut fetch.0, entity, row) }
    }
}
unsafe impl<C: Component<Mutability = Mutable>> ReadOnlyQueryData for ComponentRead<C> {}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;

    #[derive(Component, Clone, Default, PartialEq, Debug)]
    #[component(on_insert = super::super::record_component_change::<Self>,
        on_remove = super::super::record_component_change::<Self>)]
    struct Value(u32);

    #[test]
    fn query_reads_and_unchanged_assignments_publish_nothing_and_writes_publish_once() {
        let mut world = World::new();
        let entity = world.spawn(Value(1)).id();
        let mut query = world.query::<ComponentWrite<Value>>();
        let cursor = world.resource::<ComponentJournal<Value>>().cursor();
        assert_eq!(query.get(&world, entity).unwrap().0, 1);
        {
            let mut value = query.get_mut(&mut world, entity).unwrap();
            assert!(!value.set_if_neq(Value(1)));
            assert!(value.replace_if_neq(Value(1)).is_none());
            assert!(!value.clone_from_if_neq(&Value(1)));
        }
        assert!(world.resource::<ComponentJournal<Value>>().since(Some(cursor)).unwrap().1.is_empty());
        {
            let mut value = query.get_mut(&mut world, entity).unwrap();
            value.0 = 2;
            value.0 += 3;
        }
        assert_eq!(world.resource::<ComponentJournal<Value>>().since(Some(cursor)).unwrap().1, [entity]);
    }

    #[derive(Component)] struct Left;
    #[derive(Component)] struct Right;
    #[test]
    fn disjoint_query_producers_share_the_journal_without_serializing_component_access() {
        fn left(mut rows: Query<ComponentWrite<Value>, (With<Left>, Without<Right>)>) {
            for mut value in &mut rows { value.0 += 10; }
        }
        fn right(mut rows: Query<ComponentWrite<Value>, (With<Right>, Without<Left>)>) {
            for mut value in &mut rows { value.0 += 20; }
        }
        let mut world = World::new();
        world.init_resource::<ComponentJournal<Value>>();
        let a = world.spawn((Value(1), Left)).id();
        let b = world.spawn((Value(2), Right)).id();
        let cursor = world.resource::<ComponentJournal<Value>>().cursor();
        let mut schedule = Schedule::default();
        schedule.add_systems((left, right));
        schedule.run(&mut world);
        assert_eq!(world.get::<Value>(a), Some(&Value(11)));
        assert_eq!(world.get::<Value>(b), Some(&Value(22)));
        let mut entities = world.resource::<ComponentJournal<Value>>().since(Some(cursor)).unwrap().1;
        entities.sort();
        let mut expected = vec![a, b]; expected.sort();
        assert_eq!(entities, expected);
    }
}
