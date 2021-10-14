use crate::controllers::product_controller::get_products;

#[get("/products")]
pub fn get_products_route() -> String {
    get_products()
}