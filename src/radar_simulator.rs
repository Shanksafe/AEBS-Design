use rand::Rng;
use std::{thread, time::Duration};

#[derive(Debug)]
enum ObjectType {
    Pedestrian,
    Vehicle,
    Cyclist,
    Animal,
}

#[derive(Debug)]
struct DetectedObject {
    id: u32,
    object_type: ObjectType,
    distance_m: f64,
    speed_kmh: f64,
}

fn generate_random_object(id: u32) -> DetectedObject {
    let mut rng = rand::thread_rng();
    let obj_type = match rng.gen_range(0..4) {
        0 => ObjectType::Pedestrian,
        1 => ObjectType::Vehicle,
        2 => ObjectType::Cyclist,
        _ => ObjectType::Animal,
    };

    DetectedObject {
        id,
        object_type: obj_type,
        distance_m: rng.gen_range(0.5..=200.0),
        speed_kmh: rng.gen_range(1.0..=50.0),  // only approaching objects
    }
}

pub fn run_simulation() {
    let update_interval = Duration::from_millis(100);
    let mut object_id = 0;
    let num_iterations = 30;

    println!("Starting radar simulation...");

    for _ in 0..num_iterations {
        let num_objects = rand::thread_rng().gen_range(1..=5);
        let mut objects = Vec::new();

        for _ in 0..num_objects {
            let obj = generate_random_object(object_id);
            objects.push(obj);
            object_id += 1;
        }

        println!("Detected {} object(s):", num_objects);
        for obj in &objects {
            println!("{:?}", obj);
        }

        println!("---");
        thread::sleep(update_interval);
    }

}
