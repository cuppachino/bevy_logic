use bevy::{ ecs::system::EntityCommands, prelude::*, utils::smallvec::SmallVec };
use derive_new::new;

use super::components::{ Source, Sink };

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
    /// Spawns `count` child entities with [`Source`] components.
    fn with_sources(&mut self, count: usize) -> &mut Self;

    /// Spawns `count` child entities with [`Sink`] components.
    fn with_sinks(&mut self, count: usize) -> &mut Self;
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
}
