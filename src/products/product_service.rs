use crate::products::{
    product_model::{CreateProduct, Product},
    product_repository,
};

pub async fn get_all_products() -> Vec<Product> {
    product_repository::Repo::new()
        .await
        .find_all()
        .await
        .expect("error get data")
}

pub async fn create_product(product: CreateProduct) -> String {
    product_repository::Repo::new()
        .await
        .insert(product)
        .await
        .expect("error create product");

    "category create success".to_string()
}
