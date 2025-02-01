// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order{
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order{
        validiate_product_name(&product_name);
        validiate_quantity(&quantity);
        validiate_unit_price(&unit_price);

        Order{
            product_name,
            quantity,
            unit_price
        }
    }
}

fn validiate_product_name(product_name: &String){
    if product_name.is_empty(){
        panic!("Prodcut name should not be empty.")
    }
    if product_name.len() >=300{
        panic!("Product name should not be more then 300 bytes.")
    }
}

fn validiate_quantity(quantity: &u32){
    if quantity == &0{
        panic!("Quantity should not be zero.")
    }
}

fn validiate_unit_price(unit_price: &u32){
    if unit_price == &0 {
        panic!("Unit price should not be zero")
    }
}