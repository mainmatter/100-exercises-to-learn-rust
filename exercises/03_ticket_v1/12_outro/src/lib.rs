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
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    fn assert_production_name(product_name: &String) {
        if product_name.is_empty() {
            panic!("Product name can not be empty");
        }

        if product_name.len() > 300 {
            panic!("Product name can not be longer than 300");
        }
    }

    fn assert_quantity(quantity: i32) {
        if quantity <= 0 {
            panic!("Quantity should be greater than zero");
        }
    }

    fn assert_unit_price(unit_price: i32) {
        if unit_price <= 0 {
            panic!("Unit price should be greater than zero");
        }
    }

    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Self {
        Order::assert_production_name(&product_name);
        Order::assert_unit_price(unit_price);
        Order::assert_quantity(quantity);

        Order {
            quantity,
            unit_price,
            product_name,
        }
    }

    pub fn total(&self) -> i32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        Order::assert_production_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn set_unit_price(&mut self, new_unit_price: i32) {
        Order::assert_unit_price(new_unit_price);
        self.unit_price = new_unit_price;
    }

    pub fn set_quantity(&mut self, new_quantity: i32) {
        Order::assert_quantity(new_quantity);
        self.quantity = new_quantity;
    }
}
