use crate::categories::{
    category_model::{Category, CreateCategory},
    category_repository,
};

pub async fn get_all_categories() -> Vec<Category> {
    category_repository::Repo::new()
        .await
        .find_all()
        .await
        .expect("error get data")
}

pub async fn create_category(category: CreateCategory) -> String {
    category_repository::Repo::new()
        .await
        .insert(category)
        .await
        .expect("error create category");

    "category create success".to_string()
}
