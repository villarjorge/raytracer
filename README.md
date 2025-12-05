# Overview

A CPU raytracer written in Rust based on the one descrived in [_Ray Tracing in One Weekend_][one_weekend] and [_Ray Tracing: The Next Week_](next_week).

## Images

![Three big spheres surraunded by many smaller ones.](images\success2.png)

![Two cylinders, one thick and the other thin in the classic cornell box](images\cornell_box_two_cylinders_medium_quality.png)

![A busy scene showcasing: Arbitrary textures in spheres, Perlin noise textures, a glass sphere creating a caustic, a sphere with subsurface scattering, a floor made of boxes with random hights and a cube composed of small white spheres. The scene is iluminated by a square from above and the whole scene is suffused by a slight fog.](images\final_image_low_quality2.png)

## Primitives

Primitives are implemented through a trait called Hittable. The currently implemented primitives are those detailed in the book:

- HittableList
- Sphere
- Parallelogram
- Triangle: Copies the code from parallelogram changing the function that determines if the point is inside the primitive

And the one that I have added:

- Quadric: Can represent a wide range of primitives. Currently only cylinder, sphere and cone

[one_weekend]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[next_week]: https://raytracing.github.io/books/RayTracingTheNextWeek.html