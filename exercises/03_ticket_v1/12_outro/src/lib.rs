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

pub struct Order {
   pub(crate)  product_name: String,
   pub(crate) quantity: u128,
   pub(crate)  unit_price: u128,
}

impl Order {
    pub fn new(product_name: String, quantity: u128, unit_price: u128) -> Order{
        if product_name.is_empty(){
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300{
            panic!("Product name cannot be longer than 300 bytes");
        }
        if quantity == 0 {
            panic!("Quantity cannot be less than 0");
        }
        if unit_price == 0 {
            panic!("Unit price cannot be less than 0");
        }

        Order{
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn product_name(&self) -> &String{
        &self.product_name
    }

    pub fn quantity(&self) -> &u128{
        &self.quantity
    }

    pub fn unit_price(&self) -> &u128{
        &self.unit_price
    }

    pub fn total(&self) -> u128{
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, new_product_name: String){
        self.valid_product_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u128){
        self.valid_quantity(new_quantity);
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: u128){
        self.valid_unit_price(new_unit_price);
        self.unit_price = new_unit_price;
    }

    pub fn valid_product_name(&self, product_name: &String){
        if product_name.is_empty(){
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300{
            panic!("Product name cannot be longer than 300 bytes");
        }
    }

    pub fn valid_quantity(&self, quantity: u128){
        if quantity == 0 {
            panic!("Quantity cannot be less than 0");
        }
    }

    pub fn valid_unit_price(&self, unit_price: u128){
        if unit_price == 0 {
            panic!("Unit price cannot be less than 0");
        }
    }




}
