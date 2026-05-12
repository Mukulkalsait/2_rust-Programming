mod data_structs;
mod serde_operations;
mod test;

fn main() {
    serde_operations::serilise_data();
    serde_operations::deserilise_data();
    test::test_serde_values();
}
