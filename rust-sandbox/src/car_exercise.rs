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

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    let age = car_quality(miles);

    let car_type = if age.1 > 0 { "used car" } else { "new car" };

    let roof_type = if roof { "Hard Top" } else { "Convertible" };

    println!(
        "Prepare a {}: {:?}, {}, {}, {} miles\n",
        car_type, motor, color, roof_type, miles
    );

    Car {
        color,
        motor,
        roof,
        age,
    }
}

pub fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut car: Car;
    let mut engine: Transmission = Transmission::Manual;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}
