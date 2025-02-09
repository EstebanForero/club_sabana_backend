use async_trait::async_trait;
use use_cases::category_service::err::Result;
use use_cases::category_service::repository_trait::CategoryRepository;

use crate::TursoDb;

#[async_trait]
impl CategoryRepository for TursoDb {
    async fn create_category(&self, category: &Category) -> Result<()> {}

    fn get_category_by_id(&self, id: Uuid) -> Result<Option<Category>> {
        todo!()
    }

    fn update_category(&self, category: &Category) -> Result<()> {
        todo!()
    }

    fn delete_category(&self, id: Uuid) -> Result<()> {
        todo!()
    }

    fn list_categories(&self) -> Result<Vec<Category>> {
        todo!()
    }
    // add code here
}
