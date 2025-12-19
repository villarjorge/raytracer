use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

use crate::bvh::BVHNode;
use crate::hittable::Hittable;
use crate::hittable::triangle::Triangle;
use crate::material::Material;
use crate::point3::{Point3, Vector3};

/// Load a BVHNode of polygons from a .obj file.
/// Code from: https://www.justinthein.dev/ray_tracer/2021/07/21/ray_tracer_parser.html, extended a bit by me
// To do: make use of the normals in the file, if they exist. In the creation of a polygon, is n normalized?
// To do: support loading materials if provided. You would have to deal with .mtl, converting them to a Material
pub fn load_model(model_path: &str, scale: f64, material: Arc<dyn Material>) -> BVHNode {
    let file: File = File::open(model_path).unwrap();
    let lines: std::io::Lines<BufReader<File>> = BufReader::new(file).lines();

    let mut vertex_coords: Vec<Vec<f64>> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    // let mut vertex_normals: Vec<Vec<f64>> = Vec::new();

    let mut seen_normals: bool = true;
    let mut seen_texture_coords: bool = true;

    for line in lines.map_while(Result::ok) {
        let mut line_iter = line.split_ascii_whitespace();
        if let Some(first_word) = line_iter.next() {
            match first_word {
                "v" => {
                    let coords: Vec<f64> = line_iter.map(|s| s.parse::<f64>().unwrap()).collect();
                    if coords.len() != 3 {
                        panic!("unable to parse non 3d coordinates");
                    }
                    vertex_coords.push(coords);
                }
                "f" => {
                    // the format can be like this: f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3 so only get first, which is the vertex
                    let vertices: Vec<usize> = line_iter
                        .map(|s| s.split("/").next().unwrap().parse::<usize>().unwrap())
                        .map(|i| i - 1) // normalize into 0 index
                        .collect();
                    // if vertices.len() != 3 && vertices.len() != 4 {
                    //     panic!("Only triangles and parallelogram polygons are supported")
                    // }

                    faces.push(vertices);
                }
                // "vn" => {
                //     let normals = line_iter
                //     .map(|s| s.parse::<f64>().unwrap())
                //     .collect();
                //     vertex_normals.push(normals);
                // },
                "vn" => {
                    if seen_normals {
                        eprintln!("Ignoring normals");
                        seen_normals = false;
                    }
                }, // ignore normals
                "#" => (),                             // ignore comment line
                "vt" => {
                    if seen_texture_coords {
                        eprintln!("Ignoring texture coordinates");
                        seen_texture_coords = false;
                    }
                }, // ignore texture coordinates
                "s" => eprintln!("Smooth shading is not supported, ignoring"),
                "o" => eprintln!("Loading object with name {}", line_iter.collect::<String>()),
                "vp" => eprintln!("Free form geometries are not supported"),
                "usemtl" => eprintln!("Ignoring use material"),
                "mtllib" => eprintln!("Ignoring material definition"),
                _ => panic!("Unexpected line in `.obj` file."),
            }
        }
    }
    // To do: move this to point3/mod.rs
    let point_from_vec = |coord: &Vec<f64>| -> Vector3 {
        Vector3::new(scale * coord[0], scale * coord[1], scale * coord[2])
    };

    // triangulate polygons with more than three vertices by supposing that they are convex and going around in a fan https://en.wikipedia.org/wiki/Fan_triangulation
    let get_triangles = |face: &Vec<usize>| -> Vec<Arc<dyn Hittable>> {
        let v1: Point3 = point_from_vec(&vertex_coords[face[0]]);

        let mut triangles_vec: Vec<Arc<dyn Hittable>> = Vec::new();

        // Weirldly, .windows returns slices, not something with a garanteed size
        for slice in face[1..].windows(2) {
            let v2: Point3 = point_from_vec(&vertex_coords[slice[0]]);
            let v3: Point3 = point_from_vec(&vertex_coords[slice[1]]);

            triangles_vec.push(Arc::new(Triangle::from_vertex_locations(
                v1,
                v2,
                v3,
                material.clone(),
            )))
        }
        triangles_vec
    };

    let mut triangles: Vec<Arc<dyn Hittable>> = Vec::new();

    for face in faces {
        triangles.extend(get_triangles(&face));
    }

    BVHNode::from_vec(triangles)
}
