use bambangshop::{Result, compose_error_response};
use rocket::http::Status;

use crate::model::product::Product;
use crate::repository::product::ProductRepository;
use crate::service::notification::NotificationService;

pub struct ProductService;

impl ProductService {
    pub fn create(mut product: Product) -> Result<Product> {
        product.product_type = product.product_type.to_uppercase();
        let product_result = ProductRepository::add(product);

        NotificationService::notify(
            &product_result.product_type,
            "CREATED",
            product_result.clone()
        );

        Ok(product_result)
    }

    pub fn list() -> Result<Vec<Product>> {
        Ok(ProductRepository::get_all())
    }

    pub fn read(id: usize) -> Result<Product> {
        let product_opt = ProductRepository::get_by_id(id);

        if product_opt.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Product not found.")
            ));
        }

        Ok(product_opt.unwrap())
    }

    pub fn delete(id: usize) -> Result<Product> {
        let product_opt = ProductRepository::delete(id);

        if product_opt.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Product not found.")
            ));
        }

        let product = product_opt.unwrap();

        NotificationService::notify(
            &product.product_type,
            "DELETED",
            product.clone()
        );

        Ok(product)
    }

    pub fn publish(id: usize) -> Result<Product> {
        let product_opt = ProductRepository::get_by_id(id);

        if product_opt.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Product not found.")
            ));
        }

        let product = product_opt.unwrap();

        NotificationService::notify(
            &product.product_type,
            "PROMOTION",
            product.clone()
        );

        Ok(product)
    }
}