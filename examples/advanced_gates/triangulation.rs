use bevy::prelude::*;
use i_float::f32_vec::F32Vec;
use i_shape::fix_shape::FixShape;
use i_overlay::core::fill_rule::FillRule;
use i_triangle::triangulation::triangulate::Triangulate;
use itertools::Itertools;

/// Reverse the winding order of a list of triangle indices for a mesh.
fn reverse_triangle_list_windings(indices: &mut [usize]) {
    for i in (0..indices.len()).step_by(3) {
        indices.swap(i, i + 2);
    }
}

pub struct Triangulation2_5D {
    pub points: Vec<Vec3>,
    pub indices: Vec<usize>,
}

pub trait Triangulate2_5D {
    fn triangulate_2_5d(&self, thickness: f32) -> Triangulation2_5D;
}

impl Triangulate2_5D for Vec<Vec2> {
    fn triangulate_2_5d(&self, half_thickness: f32) -> Triangulation2_5D {
        #[allow(unused_assignments)]
        let mut len = 0;
        let back_face = {
            let shape = FixShape::new_with_contour(self.clone().to_fixed());
            let triangulation = shape.to_triangulation(Some(FillRule::NonZero));
            let points = triangulation.points
                .iter()
                .map(|v| {
                    let F32Vec { x, y } = v.to_f32vec();
                    Vec3::new(x, y, -half_thickness)
                })
                .collect::<Vec<_>>();

            len = points.len();

            (points, triangulation.indices)
        };

        let front_face = {
            let points = back_face.0
                .iter()
                .map(|v| { Vec3::new(v.x, v.y, half_thickness) })
                .collect::<Vec<_>>();

            let mut indices = back_face.1
                .iter()
                .map(|i| { i + len })
                .collect::<Vec<_>>();
            reverse_triangle_list_windings(&mut indices);

            len += points.len();

            (points, indices)
        };

        let walls = {
            // walk the original points and create a quad between the front and back face
            // from two triangles in counter-clockwise order
            let mut points = Vec::default();
            let mut indices = Vec::default();
            for (a, b) in self.iter().circular_tuple_windows() {
                let c = b.extend(-half_thickness);
                let d = a.extend(-half_thickness);
                let a = a.extend(half_thickness);
                let b = b.extend(half_thickness);
                points.push(a);
                points.push(b);
                points.push(c);
                points.push(d);
                for i in [0, 1, 2, 0, 2, 3] {
                    indices.push(i + len);
                }
                len += 4;
            }
            (points, indices)
        };

        Triangulation2_5D {
            points: back_face.0.into_iter().chain(front_face.0).chain(walls.0).collect(),
            indices: back_face.1.into_iter().chain(front_face.1).chain(walls.1).collect(),
        }
    }
}

trait ToFixed {
    type Fixed;
    fn to_fixed(self) -> Self::Fixed;
}

impl ToFixed for Vec2 {
    type Fixed = i_float::fix_vec::FixVec;
    fn to_fixed(self) -> Self::Fixed {
        i_float::fix_vec::FixVec::new_f32(self.x, self.y)
    }
}

impl ToFixed for Vec<Vec2> {
    type Fixed = Vec<i_float::fix_vec::FixVec>;
    fn to_fixed(self) -> Self::Fixed {
        self.into_iter()
            .map(|v| { v.to_fixed() })
            .collect()
    }
}

pub trait Translate<A> {
    fn translate(&self, by: A) -> Self;
}

impl Translate<Vec2> for Vec2 {
    fn translate(&self, by: Vec2) -> Self {
        use std::ops::Add;
        self.add(by)
    }
}

impl Translate<Vec2> for Vec<Vec2> {
    fn translate(&self, by: Vec2) -> Self {
        self.iter()
            .map(|v| v.translate(by))
            .collect()
    }
}
