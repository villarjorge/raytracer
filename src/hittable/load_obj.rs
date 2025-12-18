use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

use crate::bvh::BVHNode;
use crate::hittable::Hittable;
use crate::hittable::parallelogram::Parallelogram;
use crate::hittable::triangle::Triangle;
use crate::material::Material;
use crate::point3::{Point3, Vector3};

/// Load a BVHNode of polygons from a .obj file.
/// Code from: https://www.justinthein.dev/ray_tracer/2021/07/21/ray_tracer_parser.html, extended a bit by me
// To do: make use of the normals in the file, if they exist. In the creation of a polygon, is n normalized?
// To do: triangulate polygons with more than three vertices https://en.wikipedia.org/wiki/Fan_triangulation
// To do: support loading materials if provided. You would have to deal with .mtl, converting them to a Material
pub fn load_model(model_path: &str, scale: f64, material: Arc<dyn Material>) -> BVHNode {
    let file: File = File::open(model_path).unwrap();
    let lines: std::io::Lines<BufReader<File>> = BufReader::new(file).lines();

    let mut vertex_coords: Vec<Vec<f64>> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    // let mut vertex_normals: Vec<Vec<f64>> = Vec::new();

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
                    if vertices.len() != 3 && vertices.len() != 4 {
                        panic!("Only triangles and parallelogram polygons are supported")
                    }

                    faces.push(vertices);
                }
                // "vn" => {
                //     let normals = line_iter
                //     .map(|s| s.parse::<f64>().unwrap())
                //     .collect();
                //     vertex_normals.push(normals);
                // },
                "vn" => eprintln!("Ignoring normals"), // ignore normals
                "#" => (),                             // ignore comment line
                "vt" => eprintln!("Ignoring texture coordinates"), // ignore texture coordinates
                "s" => eprintln!("Smooth shading is not supported, ignoring"),
                "o" => eprintln!("Loading object with name {}", line_iter.collect::<String>()),
                "vp" => eprintln!("Free form geometries are not supported"),
                "usemtl" => eprintln!("Ignoring use material"),
                "mtllib" => eprintln!("Ignoring material definition"),
                _ => panic!("Unexpected line in `.obj` file."),
            }
        }
    }

    let point_from_vec = |coord: &Vec<f64>| -> Vector3 {
        Vector3::new(scale * coord[0], scale * coord[1], scale * coord[2])
    };

    // To do: Instead of implementing a general polygon, triangulate non triangle polygons

    let get_triangle = |face: &Vec<usize>| -> Arc<dyn Hittable> {
        let v1: Point3 = point_from_vec(&vertex_coords[face[0]]);
        let v2: Point3 = point_from_vec(&vertex_coords[face[1]]);
        let v3: Point3 = point_from_vec(&vertex_coords[face[2]]);
        Arc::new(Triangle::from_vertex_locations(
            v1,
            v2,
            v3,
            material.clone(),
        ))
    };

    let get_quadrilateral = |face: &Vec<usize>| -> Arc<dyn Hittable> {
        let v1: Point3 = point_from_vec(&vertex_coords[face[0]]);
        let v2: Point3 = point_from_vec(&vertex_coords[face[1]]);
        let v3: Point3 = point_from_vec(&vertex_coords[face[2]]);
        Arc::new(Parallelogram::from_vertex_locations(
            v1,
            v2,
            v3,
            material.clone(),
        ))
    };

    let get_polygon = |face: &Vec<usize>| -> Arc<dyn Hittable> {
        // Ignore the last face value
        if face.len() == 3 {
            get_triangle(face)
        } else if face.len() == 4 {
            get_quadrilateral(face)
        } else {
            panic!()
        }
    };

    let mut polygons: Vec<Arc<dyn Hittable>> = Vec::new();

    for face in faces {
        polygons.push(get_polygon(&face));
    }

    BVHNode::from_vec(polygons)

    // Remember how to get a slice https://stackoverflow.com/questions/39785597/how-do-i-get-a-slice-of-a-vect-in-rust
    // And how to flatten the two zips https://stackoverflow.com/questions/29669287/how-can-i-zip-more-than-two-iterators
}
