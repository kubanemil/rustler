/*
You want to impement Drop trait to specify how resources and data in memory will be managed
after the variable goes out of scope.
 */
pub fn main() {
    let _ptr1 = CustomPointer { data: 11 };
    {
        let _ptr2 = CustomPointer { data: 22 };
        CustomPointer { data: 33 }; // will be dropped first
    }
}

struct CustomPointer {
    data: u8,
}

impl Drop for CustomPointer {
    fn drop(&mut self) {
        //can't call this explicitly, instead call `std::mem::drop(value)`
        println!("Trying to drop {}", &self.data);
    }
}
