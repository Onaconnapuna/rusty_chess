// mutability of a struct is in its binding, not declaration, so when I create an instance, thats when I use mut;

pub struct Board {
    pub grid: [ [u8; 8]; 8],
}