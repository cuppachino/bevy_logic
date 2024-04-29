use bevy::{ prelude::*, render::{ mesh::PrimitiveTopology, render_asset::RenderAssetUsages } };
use bevy_logic::prelude::*;

use crate::{ helpers::*, triangulation::* };

pub fn gizmo_wires(
    mut gizmos: Gizmos,
    query_wires: Query<(&Wire, &Signal)>,
    query_fans: Query<(&GlobalTransform, &Signal), (With<GateFan>, Without<Wire>)>
) {
    for (gt, signal) in query_fans.iter() {
        gizmos.circle(gt.translation(), Direction3d::Z, 0.08, if signal.is_truthy() {
            Color::GREEN
        } else {
            Color::GRAY
        });
    }

    for (wire, signal) in query_wires.iter() {
        let Ok(from) = query_fans.get(wire.from).map(|(t, _)| t.translation()) else {
            continue;
        };
        let Ok(to) = query_fans.get(wire.to).map(|(t, _)| t.translation()) else {
            continue;
        };

        let color = if signal.is_truthy() {
            (Color::GREEN, Color::DARK_GREEN)
        } else {
            (Color::GRAY, Color::BLACK)
        };

        gizmos.line_gradient(from, to, color.0, color.1);
    }
}

/// Return a list of points around an arc.
fn sample_arc(size: Vec2, subdivisions: usize) -> Vec<Vec2> {
    let step = std::f32::consts::FRAC_PI_2 / ((subdivisions - 1) as f32);
    (0..subdivisions)
        .map(|i| {
            let angle = step * (i as f32);
            let (sin, cos) = angle.sin_cos();
            Vec2::new(sin, cos) * size
        })
        .collect::<Vec<_>>()
}

/// Generate points for a rounded rectangle in clockwise order.
///
/// The `size` is the full size of the rectangle.
/// The `corner_half_size` is the half size of the corner.
/// The number of `subdivisions` determines the smoothness of the corners.
fn round_rect_points(size: Vec2, corner_half_size: Vec2, subdivisions: usize) -> Vec<Vec2> {
    let body_size = size - corner_half_size * 2.0;
    let body_half_size = body_size * 0.5;
    let tr_corner_pos = body_half_size;
    let br_corner_pos = body_half_size * Vec2::new(1.0, -1.0);
    let bl_corner_pos = -body_half_size;
    let tl_corner_pos = body_half_size * Vec2::new(-1.0, 1.0);

    let tr_corner = sample_arc(corner_half_size, subdivisions);
    let br_corner = tr_corner
        .iter()
        .map(|v| { Vec2::new(v.x, -v.y) })
        .rev()
        .collect::<Vec<_>>();
    let bl_corner = br_corner
        .iter()
        .map(|v| { Vec2::new(-v.x, v.y) })
        .rev()
        .collect::<Vec<_>>();
    let tl_corner = tr_corner
        .iter()
        .map(|v| { Vec2::new(-v.x, v.y) })
        .rev()
        .collect::<Vec<_>>();

    tr_corner
        .translate(tr_corner_pos)
        .into_iter()
        .chain(br_corner.translate(br_corner_pos))
        .chain(bl_corner.translate(bl_corner_pos))
        .chain(tl_corner.translate(tl_corner_pos))
        .collect()
}

/// Triangulate a 2.5D mesh from a 2D contour.
fn round_rect_mesh(points: Vec<Vec2>, half_thickness: f32) -> Mesh {
    let Triangulation2_5D { points, indices } = points.triangulate_2_5d(half_thickness);
    let indices = bevy::render::mesh::Indices::U16(
        indices
            .into_iter()
            .map(|i| { i as u16 })
            .collect()
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh.insert_indices(indices);
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh
}

/// Construct a mesh for a logic gate.
pub fn build_mesh(num_inputs: usize, num_outputs: usize, num_special_inputs: usize) -> Mesh {
    let num_fans = num_inputs.max(num_outputs);
    let height = ((num_fans as f32) * 0.5 * GATE_UNIT_SIZE).max(GATE_UNIT_SIZE);
    let width = ((num_special_inputs as f32) * 0.5 * GATE_UNIT_SIZE).max(GATE_UNIT_SIZE);
    let size = Vec2::new(width, height);
    let points = round_rect_points(size, Vec2::splat(0.1), 4);
    round_rect_mesh(points, GATE_UNIT_HALF_THICKNESS)
}
