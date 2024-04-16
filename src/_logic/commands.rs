use bevy::{ ecs::system::EntityCommands, prelude::*, utils::smallvec::SmallVec };
use derive_new::new;

use crate::Desync;

use super::components::{ Source, Sink };

/// A trait that upgrades a `Commands` into a `LogicCommands`.
///
/// This is useful for spawning entities with logic components and
/// inserting `Desync` delay components to get rid of infinite feedback loops.
pub trait LogicCommandsExt<'w, 's> {
    /// Upgrade this `Commands` into a `LogicCommands`.
    ///
    /// The advantage of using `LogicCommands` is that it automatically
    /// inserts a `Desync` component into every entity that is spawned,
    /// which is useful for preventing infinite feedback loops in logic circuits.
    fn logic(&mut self) -> LogicCommands<'w, 's, '_>;
}

impl<'w, 's> LogicCommandsExt<'w, 's> for Commands<'w, 's> {
    fn logic(&mut self) -> LogicCommands<'w, 's, '_> {
        LogicCommands {
            desync: Desync::default(),
            commands: self,
        }
    }
}

pub struct LogicCommands<'w, 's, 'a> {
    pub commands: &'a mut Commands<'w, 's>,
    pub desync: Desync,
}

#[derive(new)]
pub struct LogicEntityCommands<'a> {
    entity_commands: EntityCommands<'a>,
    #[new(default)]
    sources: SmallVec<[Entity; 4]>,
    #[new(default)]
    sinks: SmallVec<[Entity; 4]>,
}

impl LogicEntityCommands<'_> {
    /// Insert a bundle of components using the entity commands
    /// associated with this logic entity.
    #[inline]
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity_commands.insert(bundle);
        self
    }

    /// Calls [BuildChildren::with_children] on the entity commands associated with this logic entity.
    pub fn with_children(&mut self, f: impl FnOnce(&mut ChildBuilder)) -> &mut Self {
        self.entity_commands.with_children(f);
        self
    }

    /// Downgrade this command builder into a logic entity, dropping
    /// the mutable reference to entity commands.
    pub fn downgrade(self) -> LogicEntity {
        LogicEntity {
            node: self.entity_commands.id(),
            sources: self.sources,
            sinks: self.sinks,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogicEntity {
    node: Entity,
    sources: SmallVec<[Entity; 4]>,
    sinks: SmallVec<[Entity; 4]>,
}

/// A trait that allows getting a source or sink [Entity] by index.
pub trait SourceSinkGetter {
    /// Returns the source [Entity] at `index`, if it exists.
    fn source(&self, index: usize) -> Option<Entity>;

    /// Returns the sink [Entity] at `index`, if it exists.
    fn sink(&self, index: usize) -> Option<Entity>;
}

impl SourceSinkGetter for LogicEntityCommands<'_> {
    fn source(&self, index: usize) -> Option<Entity> {
        self.sources.get(index).copied()
    }

    fn sink(&self, index: usize) -> Option<Entity> {
        self.sinks.get(index).copied()
    }
}

impl SourceSinkGetter for LogicEntity {
    fn source(&self, index: usize) -> Option<Entity> {
        self.sources.get(index).copied()
    }

    fn sink(&self, index: usize) -> Option<Entity> {
        self.sinks.get(index).copied()
    }
}

/// A trait that allows spawning child entities with [`Source`] and [`Sink`] components.
pub trait SpawnSourcesAndSinks {
    /// Spawns `count` child entities with [`Source`] components and a [`SpatialBundle`].
    #[doc(alias = "with_outputs")]
    fn with_sources(&mut self, count: usize) -> &mut Self;

    /// Spawns `count` child entities with [`Sink`] components and a [`SpatialBundle`].
    #[doc(alias = "with_inputs")]
    fn with_sinks(&mut self, count: usize) -> &mut Self;

    /// Spawns each bundle in `bundles` as a child entity with [`Source`] components.
    ///
    /// Bundles should include a [`SpatialBundle`].
    fn with_output_bundles(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) -> &mut Self;

    /// Spawns each bundle in `bundles` as a child entity with [`Sink`] components.
    ///
    /// Bundles should include a [`SpatialBundle`].
    fn with_input_bundles(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) -> &mut Self;
}

impl SpawnSourcesAndSinks for LogicEntityCommands<'_> {
    fn with_sources(&mut self, count: usize) -> &mut Self {
        self.entity_commands.with_children(|entity| {
            for _ in 0..count {
                let source = entity.spawn((Source::default(), SpatialBundle::default())).id();
                self.sources.push(source);
            }
        });
        self
    }

    fn with_sinks(&mut self, count: usize) -> &mut Self {
        self.entity_commands.with_children(|entity| {
            for _ in 0..count {
                let sink = entity.spawn((Sink::default(), SpatialBundle::default())).id();
                self.sinks.push(sink);
            }
        });
        self
    }

    fn with_output_bundles(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) -> &mut Self {
        self.entity_commands.with_children(|entity| {
            for bundle in bundles {
                let source = entity.spawn((Source::default(), bundle)).id();
                self.sources.push(source);
            }
        });
        self
    }

    fn with_input_bundles(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) -> &mut Self {
        self.entity_commands.with_children(|entity| {
            for bundle in bundles {
                let sink = entity.spawn((Sink::default(), bundle)).id();
                self.sinks.push(sink);
            }
        });
        self
    }
}
