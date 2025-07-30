// geometry/src/lib.rs

pub mod shapes; 

#[derive(Debug, PartialEq, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn translate(&self, vector: &Vector3D) -> Point3D {
        Point3D {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }

    pub fn rotate_x(&self, angle_rad: f64) -> Point3D {
        let y = self.y * angle_rad.cos() - self.z * angle_rad.sin();
        let z = self.y * angle_rad.sin() + self.z * angle_rad.cos();
        Point3D { x: self.x, y, z }
    }

    pub fn rotate_y(&self, angle_rad: f64) -> Point3D {
        let x = self.x * angle_rad.cos() + self.z * angle_rad.sin();
        let z = -self.x * angle_rad.sin() + self.z * angle_rad.cos();
        Point3D { x, y: self.y, z }
    }

    pub fn rotate_z(&self, angle_rad: f64) -> Point3D {
        let x = self.x * angle_rad.cos() - self.y * angle_rad.sin();
        let y = self.x * angle_rad.sin() + self.y * angle_rad.cos();
        Point3D { x, y, z: self.z }
    }

    pub fn scale(&self, factor: f64) -> Point3D {
        Point3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn vector_to(&self, other: &Point3D) -> Vector3D {
        Vector3D::new(other.x - self.x, other.y - self.y, other.z - self.z)
    }

    pub fn rotate_around_axis(&self, axis_origin: &Point3D, axis_direction: &Vector3D, angle_rad: f64) -> Point3D {
        let p_minus_a = self.vector_to(axis_origin).scale(-1.0); // Vector from axis origin to point
        let rotated_p_minus_a = p_minus_a.scale(angle_rad.cos()) + 
                                  axis_direction.cross(&p_minus_a).scale(angle_rad.sin()) + 
                                  axis_direction.scale(axis_direction.dot(&p_minus_a));
        axis_origin.translate(&rotated_p_minus_a)
    }
}

use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let len = self.length();
        if len == 0.0 {
            Vector3D::new(0.0, 0.0, 0.0)
        } else {
            Vector3D::new(self.x / len, self.y / len, self.z / len)
        }
    }

    pub fn rotate_x(&self, angle_rad: f64) -> Vector3D {
        let y = self.y * angle_rad.cos() - self.z * angle_rad.sin();
        let z = self.y * angle_rad.sin() + self.z * angle_rad.cos();
        Vector3D { x: self.x, y, z }
    }

    pub fn rotate_y(&self, angle_rad: f64) -> Vector3D {
        let x = self.x * angle_rad.cos() + self.z * angle_rad.sin();
        let z = -self.x * angle_rad.sin() + self.z * angle_rad.cos();
        Vector3D { x, y: self.y, z }
    }

    pub fn rotate_z(&self, angle_rad: f64) -> Vector3D {
        let x = self.x * angle_rad.cos() - self.y * angle_rad.sin();
        let y = self.x * angle_rad.sin() + self.y * angle_rad.cos();
        Vector3D { x, y, z: self.z }
    }

    pub fn scale(&self, factor: f64) -> Vector3D {
        Vector3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn rotate_around_axis(&self, axis_direction: &Vector3D, angle_rad: f64) -> Vector3D {
        let rotated_v = self.scale(angle_rad.cos()) +
                        axis_direction.cross(self).scale(angle_rad.sin()) +
                        axis_direction.scale(axis_direction.dot(self));
        rotated_v
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line3D {
    pub origin: Point3D,
    pub direction: Vector3D,
}

impl Line3D {
    pub fn new(origin: Point3D, direction: Vector3D) -> Self {
        Line3D { origin, direction: direction.normalize() }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Plane3D {
    pub origin: Point3D,
    pub normal: Vector3D,
}

impl Plane3D {
    pub fn new(origin: Point3D, normal: Vector3D) -> Self {
        Plane3D { origin, normal: normal.normalize() }
    }
}

// Topological representations

#[derive(Debug, PartialEq, Clone)]
pub struct Vertex {
    pub point: Point3D,
}

impl Vertex {
    pub fn new(point: Point3D) -> Self {
        Vertex { point }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    pub start_vertex: Vertex,
    pub end_vertex: Vertex,
    pub curve: Line3D, // For simplicity, assuming linear edges for now
}

impl Edge {
    pub fn new(start_vertex: Vertex, end_vertex: Vertex, curve: Line3D) -> Self {
        Edge { start_vertex, end_vertex, curve }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Face {
    pub boundary_edges: Vec<Edge>,
    pub surface: Plane3D, // For simplicity, assuming planar faces for now
}

impl Face {
    pub fn new(boundary_edges: Vec<Edge>, surface: Plane3D) -> Self {
        Face { boundary_edges, surface }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Shell {
    pub faces: Vec<Face>,
}

impl Shell {
    pub fn new(faces: Vec<Face>) -> Self {
        Shell { faces }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solid {
    pub shell: Shell,
}

impl Solid {
    pub fn new(shell: Shell) -> Self {
        Solid { shell }
    }
}

pub fn create_box(width: f64, height: f64, depth: f64) -> Solid {
    // Define vertices
    let p0 = Point3D::new(0.0, 0.0, 0.0);
    let p1 = Point3D::new(width, 0.0, 0.0);
    let p2 = Point3D::new(width, height, 0.0);
    let p3 = Point3D::new(0.0, height, 0.0);
    let p4 = Point3D::new(0.0, 0.0, depth);
    let p5 = Point3D::new(width, 0.0, depth);
    let p6 = Point3D::new(width, height, depth);
    let p7 = Point3D::new(0.0, height, depth);

    let v0 = Vertex::new(p0.clone());
    let v1 = Vertex::new(p1.clone());
    let v2 = Vertex::new(p2.clone());
    let v3 = Vertex::new(p3.clone());
    let v4 = Vertex::new(p4.clone());
    let v5 = Vertex::new(p5.clone());
    let v6 = Vertex::new(p6.clone());
    let v7 = Vertex::new(p7.clone());

    // Define edges
    let e0 = Edge::new(v0.clone(), v1.clone(), Line3D::new(p0.clone(), p0.vector_to(&p1)));
    let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2)));
    let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), p2.vector_to(&p3)));
    let e3 = Edge::new(v3.clone(), v0.clone(), Line3D::new(p3.clone(), p3.vector_to(&p0)));

    let e4 = Edge::new(v4.clone(), v5.clone(), Line3D::new(p4.clone(), p4.vector_to(&p5)));
    let e5 = Edge::new(v5.clone(), v6.clone(), Line3D::new(p5.clone(), p5.vector_to(&p6)));
    let e6 = Edge::new(v6.clone(), v7.clone(), Line3D::new(p6.clone(), p6.vector_to(&p7)));
    let e7 = Edge::new(v7.clone(), v4.clone(), Line3D::new(p7.clone(), p7.vector_to(&p4)));

    let e8 = Edge::new(v0.clone(), v4.clone(), Line3D::new(p0.clone(), p0.vector_to(&p4)));
    let e9 = Edge::new(v1.clone(), v5.clone(), Line3D::new(p1.clone(), p1.vector_to(&p5)));
    let e10 = Edge::new(v2.clone(), v6.clone(), Line3D::new(p2.clone(), p2.vector_to(&p6)));
    let e11 = Edge::new(v3.clone(), v7.clone(), Line3D::new(p3.clone(), p3.vector_to(&p7)));

    // Define faces
    let normal_xy_pos = Vector3D::new(0.0, 0.0, 1.0);
    let normal_xy_neg = Vector3D::new(0.0, 0.0, -1.0);
    let normal_xz_pos = Vector3D::new(0.0, 1.0, 0.0);
    let normal_xz_neg = Vector3D::new(0.0, -1.0, 0.0);
    let normal_yz_pos = Vector3D::new(1.0, 0.0, 0.0);
    let normal_yz_neg = Vector3D::new(-1.0, 0.0, 0.0);

    let face_bottom = Face::new(vec![e0.clone(), e3.clone(), e2.clone(), e1.clone()], Plane3D::new(p0.clone(), normal_xy_neg));
    let face_top = Face::new(vec![e4.clone(), e5.clone(), e6.clone(), e7.clone()], Plane3D::new(p4.clone(), normal_xy_pos));
    let face_front = Face::new(vec![e0.clone(), e9.clone(), e4.clone(), e8.clone()], Plane3D::new(p0.clone(), normal_xz_neg));
    let face_back = Face::new(vec![e2.clone(), e11.clone(), e6.clone(), e10.clone()], Plane3D::new(p2.clone(), normal_xz_pos));
    let face_left = Face::new(vec![e3.clone(), e8.clone(), e7.clone(), e11.clone()], Plane3D::new(p3.clone(), normal_yz_neg));
    let face_right = Face::new(vec![e1.clone(), e10.clone(), e5.clone(), e9.clone()], Plane3D::new(p1.clone(), normal_yz_pos));

    let faces = vec![face_bottom, face_top, face_front, face_back, face_left, face_right];
    let shell = Shell::new(faces);
    Solid::new(shell)
}

const CYLINDER_SEGMENTS: usize = 32;

pub fn create_cylinder(radius: f64, height: f64) -> Solid {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    // Create bottom and top vertices
    let bottom_center = Point3D::new(0.0, 0.0, 0.0);
    let top_center = Point3D::new(0.0, 0.0, height);
    let v_bottom_center = Vertex::new(bottom_center.clone());
    let v_top_center = Vertex::new(top_center.clone());

    for i in 0..CYLINDER_SEGMENTS {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (CYLINDER_SEGMENTS as f64);
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        vertices.push(Point3D::new(x, y, 0.0)); // Bottom circle points
        vertices.push(Point3D::new(x, y, height)); // Top circle points
    }

    // Create bottom face
    let mut bottom_edges = Vec::new();
    let bottom_normal = Vector3D::new(0.0, 0.0, -1.0);
    for i in 0..CYLINDER_SEGMENTS {
        let p1 = vertices[i * 2].clone();
        let p2 = vertices[((i + 1) % CYLINDER_SEGMENTS) * 2].clone();
        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        bottom_edges.push(Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2))));

        // Add triangular faces from center to form the bottom cap
        let edge1 = Edge::new(v_bottom_center.clone(), v1.clone(), Line3D::new(bottom_center.clone(), bottom_center.vector_to(&v1.point)));
        let edge2 = Edge::new(v1.clone(), v2.clone(), Line3D::new(v1.point.clone(), v1.point.vector_to(&v2.point)));
        let edge3 = Edge::new(v2.clone(), v_bottom_center.clone(), Line3D::new(v2.point.clone(), v2.point.vector_to(&v_bottom_center.point)));
        faces.push(Face::new(vec![edge1, edge2, edge3], Plane3D::new(bottom_center.clone(), bottom_normal.clone())));
    }

    // Create top face
    let mut top_edges = Vec::new();
    let top_normal = Vector3D::new(0.0, 0.0, 1.0);
    for i in 0..CYLINDER_SEGMENTS {
        let p1 = vertices[i * 2 + 1].clone();
        let p2 = vertices[((i + 1) % CYLINDER_SEGMENTS) * 2 + 1].clone();
        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        top_edges.push(Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2))));

        // Add triangular faces from center to form the top cap
        let edge1 = Edge::new(v_top_center.clone(), v1.clone(), Line3D::new(top_center.clone(), top_center.vector_to(&v1.point)));
        let edge2 = Edge::new(v1.clone(), v2.clone(), Line3D::new(v1.point.clone(), v1.point.vector_to(&v2.point)));
        let edge3 = Edge::new(v2.clone(), v_top_center.clone(), Line3D::new(v2.point.clone(), v2.point.vector_to(&v_top_center.point)));
        faces.push(Face::new(vec![edge1, edge2, edge3], Plane3D::new(top_center.clone(), top_normal.clone())));
    }

    // Create side faces
    for i in 0..CYLINDER_SEGMENTS {
        let p_bottom_curr = vertices[i * 2].clone();
        let p_top_curr = vertices[i * 2 + 1].clone();
        let p_bottom_next = vertices[((i + 1) % CYLINDER_SEGMENTS) * 2].clone();
        let p_top_next = vertices[((i + 1) % CYLINDER_SEGMENTS) * 2 + 1].clone();

        let v_bottom_curr = Vertex::new(p_bottom_curr.clone());
        let v_top_curr = Vertex::new(p_top_curr.clone());
        let v_bottom_next = Vertex::new(p_bottom_next.clone());
        let v_top_next = Vertex::new(p_top_next.clone());

        let e1 = Edge::new(v_bottom_curr.clone(), v_bottom_next.clone(), Line3D::new(p_bottom_curr.clone(), p_bottom_curr.vector_to(&p_bottom_next)));
        let e2 = Edge::new(v_bottom_next.clone(), v_top_next.clone(), Line3D::new(p_bottom_next.clone(), p_bottom_next.vector_to(&p_top_next)));
        let e3 = Edge::new(v_top_next.clone(), v_top_curr.clone(), Line3D::new(p_top_next.clone(), p_top_next.vector_to(&p_top_curr)));
        let e4 = Edge::new(v_top_curr.clone(), v_bottom_curr.clone(), Line3D::new(p_top_curr.clone(), p_top_curr.vector_to(&p_bottom_curr)));

        // Calculate normal for the side face
        let side_normal = Vector3D::new(p_bottom_curr.x, p_bottom_curr.y, 0.0).normalize();
        faces.push(Face::new(vec![e1, e2, e3, e4], Plane3D::new(p_bottom_curr.clone(), side_normal)));
    }

    let shell = Shell::new(faces);
    Solid::new(shell)
}

const SPHERE_LATITUDE_SEGMENTS: usize = 16;
const SPHERE_LONGITUDE_SEGMENTS: usize = 32;

pub fn create_sphere(radius: f64) -> Solid {
    let mut faces = Vec::new();

    // Create vertices
    let mut vertices: Vec<Vec<Point3D>> = Vec::new();
    for i in 0..=SPHERE_LATITUDE_SEGMENTS {
        let lat = std::f64::consts::PI * (i as f64) / (SPHERE_LATITUDE_SEGMENTS as f64);
        let y = radius * lat.cos();
        let current_radius = radius * lat.sin();

        let mut row_vertices = Vec::new();
        for j in 0..SPHERE_LONGITUDE_SEGMENTS {
            let lon = 2.0 * std::f64::consts::PI * (j as f64) / (SPHERE_LONGITUDE_SEGMENTS as f64);
            let x = current_radius * lon.cos();
            let z = current_radius * lon.sin();
            row_vertices.push(Point3D::new(x, y, z));
        }
        vertices.push(row_vertices);
    }

    // Create faces
    for i in 0..SPHERE_LATITUDE_SEGMENTS {
        for j in 0..SPHERE_LONGITUDE_SEGMENTS {
            let p1 = vertices[i][j].clone();
            let p2 = vertices[i][(j + 1) % SPHERE_LONGITUDE_SEGMENTS].clone();
            let p3 = vertices[i + 1][(j + 1) % SPHERE_LONGITUDE_SEGMENTS].clone();
            let p4 = vertices[i + 1][j].clone();

            let v1 = Vertex::new(p1.clone());
            let v2 = Vertex::new(p2.clone());
            let v3 = Vertex::new(p3.clone());
            let v4 = Vertex::new(p4.clone());

            let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2)));
            let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), p2.vector_to(&p3)));
            let e3 = Edge::new(v3.clone(), v4.clone(), Line3D::new(p3.clone(), p3.vector_to(&p4)));
            let e4 = Edge::new(v4.clone(), v1.clone(), Line3D::new(p4.clone(), p4.vector_to(&p1)));

            let normal = Vector3D::new(p1.x, p1.y, p1.z).normalize(); // Normal pointing outwards from the center
            faces.push(Face::new(vec![e1, e2, e3, e4], Plane3D::new(p1.clone(), normal)));
        }
    }

    let shell = Shell::new(faces);
    Solid::new(shell)
}

pub fn extrude(face: &Face, extrusion_vector: &Vector3D) -> Solid {
    let mut new_faces: Vec<Face> = Vec::new();

    // 1. Original face (bottom cap of extrusion)
    new_faces.push(face.clone());

    // 2. Extruded face (top cap of extrusion)
    let mut extruded_boundary_edges: Vec<Edge> = Vec::new();
    for edge in &face.boundary_edges {
        let new_start_point = edge.start_vertex.point.translate(extrusion_vector);
        let new_end_point = edge.end_vertex.point.translate(extrusion_vector);
        let new_start_vertex = Vertex::new(new_start_point.clone());
        let new_end_vertex = Vertex::new(new_end_point.clone());
        let new_line = Line3D::new(new_start_point.clone(), new_start_point.vector_to(&new_end_point));
        extruded_boundary_edges.push(Edge::new(new_start_vertex, new_end_vertex, new_line));
    }
    let extruded_surface_origin = face.surface.origin.translate(extrusion_vector);
    let extruded_face = Face::new(extruded_boundary_edges, Plane3D::new(extruded_surface_origin, face.surface.normal.clone()));
    new_faces.push(extruded_face);

    // 3. Side faces
    for i in 0..face.boundary_edges.len() {
        let current_edge = &face.boundary_edges[i];

        let p_orig_start = current_edge.start_vertex.point.clone();
        let p_orig_end = current_edge.end_vertex.point.clone();
        let p_extr_start = current_edge.start_vertex.point.translate(extrusion_vector);
        let p_extr_end = current_edge.end_vertex.point.translate(extrusion_vector);

        let v_orig_start = Vertex::new(p_orig_start.clone());
        let v_orig_end = Vertex::new(p_orig_end.clone());
        let v_extr_start = Vertex::new(p_extr_start.clone());
        let v_extr_end = Vertex::new(p_extr_end.clone());

        // Edges for the side face (quadrilateral)
        let e1 = Edge::new(v_orig_start.clone(), v_orig_end.clone(), Line3D::new(p_orig_start.clone(), p_orig_start.vector_to(&p_orig_end)));
        let e2 = Edge::new(v_orig_end.clone(), v_extr_end.clone(), Line3D::new(p_orig_end.clone(), p_orig_end.vector_to(&p_extr_end)));
        let e3 = Edge::new(v_extr_end.clone(), v_extr_start.clone(), Line3D::new(p_extr_end.clone(), p_extr_end.vector_to(&p_extr_start)));
        let e4 = Edge::new(v_extr_start.clone(), v_orig_start.clone(), Line3D::new(p_extr_start.clone(), p_extr_start.vector_to(&p_orig_start)));

        // Calculate normal for the side face.
        let edge_direction = p_orig_start.vector_to(&p_orig_end);
        let side_normal = edge_direction.cross(extrusion_vector).normalize();

        new_faces.push(Face::new(vec![e1, e2, e3, e4], Plane3D::new(p_orig_start.clone(), side_normal)));
    }

    let shell = Shell::new(new_faces);
    Solid::new(shell)
}

const REVOLUTION_SEGMENTS: usize = 32;

pub fn revolution(face: &Face, axis: &Line3D, angle_rad: f64) -> Solid {
    let mut all_faces: Vec<Face> = Vec::new();

    // Add the original face as one cap
    all_faces.push(face.clone());

    let angle_increment = angle_rad / (REVOLUTION_SEGMENTS as f64);

    let mut previous_face_points: Vec<Point3D> = face.boundary_edges.iter().map(|edge| edge.start_vertex.point.clone()).collect();

    for i in 1..=REVOLUTION_SEGMENTS {
        let current_angle = angle_increment * (i as f64);
        let mut current_face_points: Vec<Point3D> = Vec::new();

        // Rotate each point of the original face
        for p in &previous_face_points {
            current_face_points.push(p.rotate_around_axis(&axis.origin, &axis.direction, current_angle));
        }

        // Create side faces connecting previous and current revolution steps
        for j in 0..face.boundary_edges.len() {
            let p1 = previous_face_points[j].clone();
            let p2 = previous_face_points[(j + 1) % face.boundary_edges.len()].clone();
            let p3 = current_face_points[(j + 1) % face.boundary_edges.len()].clone();
            let p4 = current_face_points[j].clone();

            let v1 = Vertex::new(p1.clone());
            let v2 = Vertex::new(p2.clone());
            let v3 = Vertex::new(p3.clone());
            let v4 = Vertex::new(p4.clone());

            let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2)));
            let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), p2.vector_to(&p3)));
            let e3 = Edge::new(v3.clone(), v4.clone(), Line3D::new(p3.clone(), p3.vector_to(&p4)));
            let e4 = Edge::new(v4.clone(), v1.clone(), Line3D::new(p4.clone(), p4.vector_to(&p1)));

            // Calculate normal for the side face (approximate for now)
            let normal = (p2.vector_to(&p1)).cross(&p4.vector_to(&p1)).normalize();
            all_faces.push(Face::new(vec![e1, e2, e3, e4], Plane3D::new(p1.clone(), normal)));
        }
        previous_face_points = current_face_points;
    }

    // Add the final rotated face as the other cap
    let mut final_boundary_edges: Vec<Edge> = Vec::new();
    for j in 0..face.boundary_edges.len() {
        let p1 = previous_face_points[j].clone();
        let p2 = previous_face_points[(j + 1) % face.boundary_edges.len()].clone();
        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        final_boundary_edges.push(Edge::new(v1, v2, Line3D::new(p1.clone(), p1.vector_to(&p2))));
    }
    let final_surface_origin = face.surface.origin.rotate_around_axis(&axis.origin, &axis.direction, angle_rad);
    let final_face = Face::new(final_boundary_edges, Plane3D::new(final_surface_origin, face.surface.normal.rotate_around_axis(&axis.direction, angle_rad)));
    all_faces.push(final_face);

    let shell = Shell::new(all_faces);
    Solid::new(shell)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-9;

    fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        assert!((a - b).abs() < epsilon, "{} is not approximately equal to {}", a, b);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let expected = Vector3D::new(5.0, 7.0, 9.0);
        assert_eq!(v1.clone().add(v2.clone()), expected);
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector3D::new(5.0, 7.0, 9.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let expected = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v1.sub(&v2), expected);
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let expected = Vector3D::new(2.0, 4.0, 6.0);
        assert_eq!(v.scale(2.0), expected);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let expected = 32.0;
        assert_eq!(v1.dot(&v2), expected);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let expected = Vector3D::new(0.0, 0.0, 1.0);
        assert_eq!(v1.cross(&v2), expected);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector3D::new(3.0, 0.0, 4.0);
        let expected = Vector3D::new(0.6, 0.0, 0.8);
        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn test_point_translate() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let expected = Point3D::new(2.0, 3.0, 4.0);
        assert_eq!(p.translate(&v), expected);
    }

    #[test]
    fn test_line3d_new() {
        let origin = Point3D::new(0.0, 0.0, 0.0);
        let direction = Vector3D::new(3.0, 0.0, 4.0);
        let line = Line3D::new(origin, direction);
        assert_eq!(line.direction, Vector3D::new(0.6, 0.0, 0.8));
    }

    #[test]
    fn test_plane3d_new() {
        let origin = Point3D::new(0.0, 0.0, 0.0);
        let normal = Vector3D::new(0.0, 1.0, 0.0);
        let plane = Plane3D::new(origin, normal);
        assert_eq!(plane.normal, Vector3D::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_point_rotate_x() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let rotated_p = p.rotate_x(PI / 2.0);
        assert_approx_eq(rotated_p.x, 0.0, EPSILON);
        assert_approx_eq(rotated_p.y, 0.0, EPSILON);
        assert_approx_eq(rotated_p.z, 1.0, EPSILON);
    }

    #[test]
    fn test_point_rotate_y() {
        let p = Point3D::new(1.0, 0.0, 0.0);
        let rotated_p = p.rotate_y(PI / 2.0);
        assert_approx_eq(rotated_p.x, 0.0, EPSILON);
        assert_approx_eq(rotated_p.y, 0.0, EPSILON);
        assert_approx_eq(rotated_p.z, -1.0, EPSILON);
    }

    #[test]
    fn test_point_rotate_z() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let rotated_p = p.rotate_z(PI / 2.0);
        assert_approx_eq(rotated_p.x, -1.0, EPSILON);
        assert_approx_eq(rotated_p.y, 0.0, EPSILON);
        assert_approx_eq(rotated_p.z, 0.0, EPSILON);
    }

    #[test]
    fn test_point_scale() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        let scaled_p = p.scale(2.0);
        assert_eq!(scaled_p, Point3D::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_point_rotate_around_axis() {
        let p = Point3D::new(1.0, 0.0, 0.0);
        let axis_origin = Point3D::new(0.0, 0.0, 0.0);
        let axis_direction = Vector3D::new(0.0, 0.0, 1.0);
        let rotated_p = p.rotate_around_axis(&axis_origin, &axis_direction, PI / 2.0);
        assert_approx_eq(rotated_p.x, 0.0, EPSILON);
        assert_approx_eq(rotated_p.y, 1.0, EPSILON);
        assert_approx_eq(rotated_p.z, 0.0, EPSILON);

        let p2 = Point3D::new(1.0, 1.0, 0.0);
        let axis_origin2 = Point3D::new(0.0, 1.0, 0.0);
        let axis_direction2 = Vector3D::new(0.0, 1.0, 0.0);
        let rotated_p2 = p2.rotate_around_axis(&axis_origin2, &axis_direction2, PI / 2.0);
        assert_approx_eq(rotated_p2.x, 0.0, EPSILON);
        assert_approx_eq(rotated_p2.y, 1.0, EPSILON);
        assert_approx_eq(rotated_p2.z, -1.0, EPSILON);
    }

    #[test]
    fn test_vector_rotate_x() {
        let v = Vector3D::new(0.0, 1.0, 0.0);
        let rotated_v = v.rotate_x(PI / 2.0);
        assert_approx_eq(rotated_v.x, 0.0, EPSILON);
        assert_approx_eq(rotated_v.y, 0.0, EPSILON);
        assert_approx_eq(rotated_v.z, 1.0, EPSILON);
    }

    #[test]
    fn test_vector_rotate_y() {
        let v = Vector3D::new(1.0, 0.0, 0.0);
        let rotated_v = v.rotate_y(PI / 2.0);
        assert_approx_eq(rotated_v.x, 0.0, EPSILON);
        assert_approx_eq(rotated_v.y, 0.0, EPSILON);
        assert_approx_eq(rotated_v.z, -1.0, EPSILON);
    }

    #[test]
    fn test_vector_rotate_z() {
        let v = Vector3D::new(0.0, 1.0, 0.0);
        let rotated_v = v.rotate_z(PI / 2.0);
        assert_approx_eq(rotated_v.x, -1.0, EPSILON);
        assert_approx_eq(rotated_v.y, 0.0, EPSILON);
        assert_approx_eq(rotated_v.z, 0.0, EPSILON);
    }

    #[test]
    fn test_vector_scale() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let scaled_v = v.scale(2.0);
        assert_eq!(scaled_v, Vector3D::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vector_rotate_around_axis() {
        let v = Vector3D::new(1.0, 0.0, 0.0);
        let axis_direction = Vector3D::new(0.0, 0.0, 1.0);
        let rotated_v = v.rotate_around_axis(&axis_direction, PI / 2.0);
        assert_approx_eq(rotated_v.x, 0.0, EPSILON);
        assert_approx_eq(rotated_v.y, 1.0, EPSILON);
        assert_approx_eq(rotated_v.z, 0.0, EPSILON);
    }

    // Tests for topological representations
    #[test]
    fn test_vertex_new() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        let v = Vertex::new(p);
        assert_eq!(v.point, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_edge_new() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        let line = Line3D::new(p1, Vector3D::new(1.0, 0.0, 0.0));
        let edge = Edge::new(v1, v2, line);
        assert_eq!(edge.start_vertex.point, Point3D::new(0.0, 0.0, 0.0));
        assert_eq!(edge.end_vertex.point, Point3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_face_new() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 1.0, 0.0);
        let p4 = Point3D::new(0.0, 1.0, 0.0);

        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        let v3 = Vertex::new(p3.clone());
        let v4 = Vertex::new(p4.clone());

        let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2)));
        let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), p2.vector_to(&p3)));
        let e3 = Edge::new(v3.clone(), v4.clone(), Line3D::new(p3.clone(), p3.vector_to(&p4)));
        let e4 = Edge::new(v4.clone(), v1.clone(), Line3D::new(p4.clone(), p4.vector_to(&p1)));

        let boundary_edges = vec![e1, e2, e3, e4];
        let surface = Plane3D::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let face = Face::new(boundary_edges, surface);

        assert_eq!(face.boundary_edges.len(), 4);
        assert_eq!(face.surface.normal, Vector3D::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_shell_new() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 1.0, 0.0);
        let p4 = Point3D::new(0.0, 1.0, 0.0);

        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        let v3 = Vertex::new(p3.clone());
        let v4 = Vertex::new(p4.clone());

        let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), Vector3D::new(1.0, 0.0, 0.0)));
        let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), Vector3D::new(0.0, 1.0, 0.0)));
        let e3 = Edge::new(v3.clone(), v4.clone(), Line3D::new(p3.clone(), Vector3D::new(-1.0, 0.0, 0.0)));
        let e4 = Edge::new(v4.clone(), v1.clone(), Line3D::new(p4.clone(), Vector3D::new(0.0, -1.0, 0.0)));

        let boundary_edges = vec![e1, e2, e3, e4];
        let surface = Plane3D::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let face = Face::new(boundary_edges, surface);

        let shell = Shell::new(vec![face]);
        assert_eq!(shell.faces.len(), 1);
    }

    #[test]
    fn test_solid_new() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 1.0, 0.0);
        let p4 = Point3D::new(0.0, 1.0, 0.0);

        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        let v3 = Vertex::new(p3.clone());
        let v4 = Vertex::new(p4.clone());

        let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), Vector3D::new(1.0, 0.0, 0.0)));
        let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), Vector3D::new(0.0, 1.0, 0.0)));
        let e3 = Edge::new(v3.clone(), v4.clone(), Line3D::new(p3.clone(), Vector3D::new(-1.0, 0.0, 0.0)));
        let e4 = Edge::new(v4.clone(), v1.clone(), Line3D::new(p4.clone(), Vector3D::new(0.0, -1.0, 0.0)));

        let boundary_edges = vec![e1, e2, e3, e4];
        let surface = Plane3D::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let face = Face::new(boundary_edges, surface);

        let shell = Shell::new(vec![face]);
        let solid = Solid::new(shell);
        assert_eq!(solid.shell.faces.len(), 1);
    }

    #[test]
    fn test_create_box() {
        let box_solid = create_box(1.0, 2.0, 3.0);
        assert_eq!(box_solid.shell.faces.len(), 6);
        // Further assertions could check the vertices and edges of the faces
    }

    #[test]
    fn test_create_cylinder() {
        let cylinder_solid = create_cylinder(1.0, 2.0);
        // A cylinder has 2 end caps + CYLINDER_SEGMENTS side faces
        assert_eq!(cylinder_solid.shell.faces.len(), 2 * CYLINDER_SEGMENTS + CYLINDER_SEGMENTS);
    }

    #[test]
    fn test_create_sphere() {
        let sphere_solid = create_sphere(1.0);
        // A sphere has (SPHERE_LATITUDE_SEGMENTS) * SPHERE_LONGITUDE_SEGMENTS faces
        assert_eq!(sphere_solid.shell.faces.len(), SPHERE_LATITUDE_SEGMENTS * SPHERE_LONGITUDE_SEGMENTS);
    }

    #[test]
    fn test_extrude_face() {
        // Create a simple square face in the XY plane
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 1.0, 0.0);
        let p4 = Point3D::new(0.0, 1.0, 0.0);

        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        let v3 = Vertex::new(p3.clone());
        let v4 = Vertex::new(p4.clone());

        let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2)));
        let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), p2.vector_to(&p3)));
        let e3 = Edge::new(v3.clone(), v4.clone(), Line3D::new(p3.clone(), p3.vector_to(&p4)));
        let e4 = Edge::new(v4.clone(), v1.clone(), Line3D::new(p4.clone(), p4.vector_to(&p1)));

        let boundary_edges = vec![e1, e2, e3, e4];
        let surface = Plane3D::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let face_to_extrude = Face::new(boundary_edges, surface);

        let extrusion_vector = Vector3D::new(0.0, 0.0, 2.0);
        let extruded_solid = extrude(&face_to_extrude, &extrusion_vector);

        // An extruded square face should result in a solid with 6 faces (original, extruded, and 4 side faces)
        assert_eq!(extruded_solid.shell.faces.len(), 6);

        // Check the normal of the extruded top face
        let top_face = &extruded_solid.shell.faces[1]; // Assuming the extruded face is the second one added
        assert_eq!(top_face.surface.normal, Vector3D::new(0.0, 0.0, 1.0));

        // Check a side face normal (e.g., the one from e1)
        let side_face_1 = &extruded_solid.shell.faces[2]; // Assuming side faces start from the third one
        let expected_side_normal = Vector3D::new(1.0, 0.0, 0.0).cross(&extrusion_vector).normalize();
        assert_approx_eq(side_face_1.surface.normal.x, expected_side_normal.x, EPSILON);
        assert_approx_eq(side_face_1.surface.normal.y, expected_side_normal.y, EPSILON);
        assert_approx_eq(side_face_1.surface.normal.z, expected_side_normal.z, EPSILON);
    }

    #[test]
    fn test_revolution() {
        // Create a simple square face in the XY plane, offset from Y axis
        let p1 = Point3D::new(1.0, 0.0, 0.0);
        let p2 = Point3D::new(2.0, 0.0, 0.0);
        let p3 = Point3D::new(2.0, 1.0, 0.0);
        let p4 = Point3D::new(1.0, 1.0, 0.0);

        let v1 = Vertex::new(p1.clone());
        let v2 = Vertex::new(p2.clone());
        let v3 = Vertex::new(p3.clone());
        let v4 = Vertex::new(p4.clone());

        let e1 = Edge::new(v1.clone(), v2.clone(), Line3D::new(p1.clone(), p1.vector_to(&p2)));
        let e2 = Edge::new(v2.clone(), v3.clone(), Line3D::new(p2.clone(), p2.vector_to(&p3)));
        let e3 = Edge::new(v3.clone(), v4.clone(), Line3D::new(p3.clone(), p3.vector_to(&p4)));
        let e4 = Edge::new(v4.clone(), v1.clone(), Line3D::new(p4.clone(), p4.vector_to(&p1)));

        let boundary_edges = vec![e1, e2, e3, e4];
        let surface = Plane3D::new(Point3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let face_to_revolve = Face::new(boundary_edges, surface);

        // Revolve around the Y axis
        let axis = Line3D::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));
        let revolved_solid = revolution(&face_to_revolve, &axis, 2.0 * PI);

        // A revolved square should result in a solid with 2 caps + REVOLUTION_SEGMENTS * 4 side faces
        assert_eq!(revolved_solid.shell.faces.len(), 2 + REVOLUTION_SEGMENTS * 4);
    }
}