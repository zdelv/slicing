use crate::error::Error;
use crate::threemf::Model;

#[allow(dead_code)]
pub fn load_model(path: &str) -> Result<Model, Error> {
    let model = Model::from_file(path)?;
    Ok(model)
}

#[test]
fn test_load_model() {
    let path = "data/test.3mf";
    match load_model(path) {
        Ok(m) => {
            println!("{:?}", m);
            assert!(true)
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false)
        }
    }
}
