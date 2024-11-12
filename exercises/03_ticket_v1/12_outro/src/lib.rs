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
    product_name:String,
    quantity:u16,
    unit_price:u16
}
impl Order {



// getters

pub fn product_name(&self) -> &String {
    return &self.product_name
}

pub fn quantity(&self) -> &u16 {
    return &self.quantity
}

pub fn unit_price(&self) -> &u16 {
    return &self.unit_price
}

pub fn total(&self) -> u16 {
    return &self.unit_price * &self.quantity
}

// validation

fn name_val(name:&String) -> bool {
    if name.is_empty() == false && name.len() < 300 {
        return true
    } else {
        return false
    }
}

fn quant_val(qant:&u16) -> bool {
    return qant > &0
}

fn unit_val(unit:&u16) -> bool {
    return unit>&0
}

// setters

pub fn set_product_name(&mut self, new_name:String) { 
    if Self::name_val(&new_name) == false {
        panic!("Some shit here")
    } else {
        self.product_name = new_name;
    }
}

pub fn set_quantity(&mut self, new_quant:u16) {
    if Self::quant_val(&new_quant) == false {
        panic!("Do some shit here")

    } else {
        self.quantity = new_quant;
    }
}
pub fn set_unit_price(&mut self, new_price:u16) {
    if Self::unit_val(&new_price) == false {
        panic!("Do some shit here")
    } else {
        self.unit_price = new_price;
    }
}
// new

pub fn new(product_name:String, quantity:u16, unit_price:u16) -> Order {
    if Self::name_val(&product_name) == false {
        panic!("Some shit here")
    }
    if Self::quant_val(&quantity) == false {
        panic!("Do some shit here")

    }
    if Self::unit_val(&unit_price) == false {
        panic!("Do some shit here")
    }

    Order {
        product_name,
        quantity,
        unit_price
    }
}
}