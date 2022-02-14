use crate::http::assembly::Assembly;
use crate::http::product::Product;
use async_graphql::Union;

#[derive(Union, Debug)]
pub enum Item {
    Assembly(Assembly),
    Product(Product),
}
