extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;
mod sphere;
mod scene;

fn _random_position() -> vector::Vector {
    vector::Vector(
        (rand::random::<f32>() * 9.0) - 4.5,
        (rand::random::<f32>() * 9.0) - 4.5, 
        (rand::random::<f32>() * 8.0) + 3.0
    )
}

fn _random_sphere(p: &vector::Vector, r: f32) -> sphere::Sphere {
    let diffuse = vector::Vector(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>());
    let ambient = (rand::random::<f32>() / 3.0) * diffuse;
    let specular = vector::Vector(1.0, 1.0, 1.0);
    let shine = rand::random::<f32>() * 100.0;
    let reflectiveness = rand::random::<f32>() * 0.95;

    return sphere::Sphere::new(p, r, &ambient, &diffuse, &specular, shine, reflectiveness);
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;

    let c = camera::Camera::new(0.0, 0.0, -1.0);

    let mut objects: Vec<sphere::Sphere> = Vec::new();

    let main_sphere = sphere::Sphere::new(
        &vector::Vector(0.0, -2.0, 15.0),
        3.0,
        &vector::Vector(0.0, 0.1, 0.1),
        &vector::Vector(0.0, 0.3, 0.3),
        &vector::Vector(1.0, 1.0, 1.0),
        20.0,
        0.9
    );
    
    objects.push(main_sphere);
    
    objects.push(sphere::Sphere::new(
        &vector::Vector(0.0, 2.0, -5.0),
        4.0,
        &vector::Vector(0.0, 0.1, 0.0),
        &vector::Vector(0.0, 0.4, 0.2),
        &vector::Vector(1.0, 1.0, 1.0),
        80.0,
        0.9
    ));

    for x in -8 .. 8 {
        for z in 5 .. 15 {
            let x_coord = x as f32;
            let z_coord = z as f32;
            let pos = vector::Vector(x_coord, 2.4, z_coord);
            let col = vector::Vector(
                (x_coord + 8.0) / 16.0,
                0.5,
                (z_coord - 5.0) / 10.0,
            );

            objects.push(sphere::Sphere::new(
                &pos,
                0.2,
                &(0.2 * col),
                &col,
                &vector::Vector(1.0, 1.0, 1.0),
                60.0,
                0.0
            ));
        }
    }

    // TODO: actually make the multiple lights work :(
    let mut lights = Vec::new();
    lights.push(vector::Vector(5.0, -5.0, 5.0));

    let scene = scene::Scene::new(
        c,
        objects,
        HEIGHT,
        WIDTH,
        &lights,
        &vector::Vector(1.0, 1.0, 1.0),
        &vector::Vector(1.0, 1.0, 1.0),
        &vector::Vector(1.0, 1.0, 1.0),
    );

    scene.render();
}
