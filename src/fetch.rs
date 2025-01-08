use bevy::prelude::{Component, DetectChanges, Entity, EntityRef, Query, Without};

/// A string segment on a component, as opposed to in a [`Text3d`](crate::Text3d).
#[derive(Debug, Component, Default)]
pub struct FetchedTextSegment(pub String);

impl FetchedTextSegment {
    pub const EMPTY: Self = Self(String::new());

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A usually generated component that fetches data as a string from the world.
#[derive(Component)]
#[require(FetchedTextSegment)]
pub struct TextFetch {
    entity: Entity,
    fetch: Box<dyn FnMut(EntityRef) -> Option<String> + Send + Sync>,
}

impl TextFetch {
    /// Create a text fetcher that fetches a string from a single component if the component changes.
    pub fn fetch_component<C: Component>(
        entity: Entity,
        mut fetch: impl (FnMut(&C) -> String) + Send + Sync + 'static,
    ) -> Self {
        TextFetch {
            entity,
            fetch: Box::new(move |entity: EntityRef| {
                if let Some(component) = entity.get_ref::<C>() {
                    if component.is_changed() {
                        return Some(fetch(&component));
                    }
                }
                None
            }),
        }
    }

    /// Create a text fetcher that fetches from an [`EntityRef`].
    pub fn fetch_entity_ref(
        entity: Entity,
        fetch: impl (FnMut(EntityRef) -> Option<String>) + Send + Sync + 'static,
    ) -> Self {
        TextFetch {
            entity,
            fetch: Box::new(fetch),
        }
    }
}

/// Triggers the [`TextFetch`] component.
pub fn text_fetch_system(
    mut channels: Query<(&mut TextFetch, &mut FetchedTextSegment)>,
    other: Query<EntityRef, Without<TextFetch>>,
) {
    for (mut channel, mut text) in channels.iter_mut() {
        if let Ok(entity_ref) = other.get(channel.entity) {
            if let Some(output) = (channel.fetch)(entity_ref) {
                text.0 = output;
            }
        }
    }
}
