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
    quantity: usize,
    unit_price: usize,
}

impl Order {
    pub fn new(name: String, q: usize, price: usize) -> Self {
        Self::validate_name(&name);
        Self::validate_quantity(q);
        Self::validate_unit_price(price);

        Self {
            product_name: name,
            quantity: q,
            unit_price: price,
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &usize {
        &self.quantity
    }

    pub fn unit_price(&self) -> &usize {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, v: String) {
        Self::validate_name(&v);
        self.product_name = v;
    }

    fn validate_name(name: &str) {
        if name.is_empty() || name.len() > 300 {
            panic!("The product name can't be empty and it can't be longer than 300 bytes.")
        }
    }

    pub fn set_quantity(&mut self, v: usize) {
        Self::validate_quantity(v);
        self.quantity = v;
    }

    fn validate_quantity(v: usize) {
        if v < 1 {
            panic!("The quantity must be strictly greater than zero.");
        }
    }

    pub fn set_unit_price(&mut self, v: usize) {
        Self::validate_unit_price(v);
        self.unit_price = v;
    }

    fn validate_unit_price(v: usize) {
        if v < 1 {
            panic!("The unit price is in cents and must be strictly greater than zero.")
        }
    }

    pub fn total(&self) -> usize {
        self.quantity * self.unit_price
    }
}
