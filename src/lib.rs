struct Field (usize);

struct Crasherini {
    field: Field,
}

impl Crasherini {
    pub fn write_to_immut_ref(&mut self){
        for value in vec![&self.field] {
            value.0 = 10;
        } 
    }
}