use tabled::{Tabled, Table};

// Using HUF currency and centimeter for diameter
// Because we are comparing Hungarian made pizzas
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Tabled)]
struct Pizza {
    name: String,
    diameter_cm: u16,
    price_huf: u16,
    area_to_price: f32
}

fn calculate_pizza_sizes(pizzas: Vec<Pizza>) -> Vec<Pizza> {
    let mut calc_pizzas = Vec::new();
    for pizza in pizzas {
        let pizza_price =
            (pizza.price_huf as f32) /
            (((pizza.diameter_cm as f32)*(pizza.diameter_cm  as f32)*3.1415)/4.0);
        let pizza_item = Pizza {
            name: pizza.name,
            diameter_cm: pizza.diameter_cm,
            price_huf: pizza.price_huf,
            area_to_price: pizza_price,
        };
        calc_pizzas.push(pizza_item);
    }
    calc_pizzas
}

fn main() {
    println!("Pizza Size Calculator");
    println!("");
    let pizzas = vec!(
        { 
            Pizza {
                name: String::from("quattro formaggi"),
                diameter_cm: 24,
                price_huf: 1995,
                area_to_price: 0.0,
            }
        },
        {
            Pizza {
                name: String::from("quattro formaggi"),
                diameter_cm: 32,
                price_huf: 3195,
                area_to_price: 0.0,
            }
        },
        {
            Pizza {
                name: String::from("quattro formaggi"),
                diameter_cm: 45,
                price_huf: 5395,
                area_to_price: 0.0,
            }
        },
    );
    let calc_pizzas = calculate_pizza_sizes(pizzas);
    let pizzas_table = Table::new(calc_pizzas).to_string();
    println!("{}", pizzas_table);
}
