mod wrong;
use wrong::something_wrong;


// This file is a parody. Please don't take anything seriously besides module organization.


// This is a public function. Other module can import this.
pub fn deposit(amount: i32) {
    invest(amount);
}

pub fn withdraw(amount: i32) {
    something_wrong();
}

// It can be only used internally.
fn invest(amount: i32) {}
