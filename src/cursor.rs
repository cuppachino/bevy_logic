use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins).add_systems(Update, normal_at_cursor_position);
    }
}

#[derive(Default, Clone, Copy)]
struct LocalArrow {
    pub start: Vec3,
    pub end: Vec3,
}

fn normal_at_cursor_position(
    mut gizmos: Gizmos,
    mut ev_reader: EventReader<Pointer<Move>>,
    mut last: Local<Option<LocalArrow>>
) {
    for ev in ev_reader.read() {
        let Some(normal) = ev.event.hit.normal else {
            continue;
        };

        let Some(start) = ev.event.hit.position else {
            continue;
        };

        let len = 0.5;
        let end = start + normal * len;
        last.replace(LocalArrow { start, end });
    }

    if let Some(LocalArrow { start, end }) = *last {
        gizmos.arrow(start, end, Color::BLUE);
    }
}
