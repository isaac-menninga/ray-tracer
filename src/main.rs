extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;
mod sphere;
mod scene;

fn random_position() -> vector::Vector {
    vector::Vector(
        (rand::random::<f32>() * 9.0) - 4.5,
        (rand::random::<f32>() * 9.0) - 4.5, 
        (rand::random::<f32>() * 8.0) + 3.0
    )
}

fn random_sphere(p: &vector::Vector, r: f32) -> sphere::Sphere {
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

    // let main_sphere = sphere::Sphere::new(
    //     &vector::Vector(0.0, 0.0, 15.0),
    //     3.0,
    //     &vector::Vector(0.0, 0.2, 0.2),
    //     &vector::Vector(0.0, 0.8, 0.8),
    //     &vector::Vector(1.0, 1.0, 1.0),
    //     60.0,
    //     0.3
    // );
    
    // objects.push(main_sphere);

    for _ in 0 .. 10 {
        let pos = random_position();
        let r = (rand::random::<f32>() * 0.5) + 0.5;
        objects.push(random_sphere(&pos, r));
    }

    let scene = scene::Scene::new(
        c,
        objects,
        HEIGHT,
        WIDTH,
        5.0,
        -5.0,
        5.0,
        &vector::Vector(1.0, 1.0, 1.0),
        &vector::Vector(1.0, 1.0, 1.0),
        &vector::Vector(1.0, 1.0, 1.0),
    );

    scene.render();
}
