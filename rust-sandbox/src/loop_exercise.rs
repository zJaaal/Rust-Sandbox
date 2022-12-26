use rand::Rng;
use std::collections::HashMap;
#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}
#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

fn car_quality(miles: u32) -> (Age, u32) {
    let age = if miles == 0 { Age::New } else { Age::Used };

    (age, miles)
}

fn car_factory(order: u32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver", "Black"];
    let age = car_quality(miles);

    let color = (((order % 5) + 5) % 5) as usize;

    let mut motor = Transmission::Manual;
    let mut roof = true;

    if order % 3 == 0 {
        motor = Transmission::Automatic
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }

    Car {
        color: String::from(colors[color]),
        motor,
        roof,
        age,
    }
}

pub fn main() {
    let mut orders: HashMap<u32, Car> = HashMap::new();

    for i in 1..11 {
        let miles = rand::thread_rng().gen_range(0..100000);
        let car = car_factory(i, miles);
        orders.insert(i, car);

        println!("Order #{}: {:?}", i, orders.get(&i))
    }
}
