use crate::error::Error;
use threemf::Model;

#[allow(dead_code)]
pub fn load_model(path: &str) -> Result<Model, Error> {
    let model = Model::from_file(path)?;
    Ok(model)
}
