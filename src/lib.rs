use sea_orm::{Database, DbErr, EntityTrait, PaginatorTrait};

pub async fn query_with_generic_type<EntityTraitType: EntityTrait>() -> Result<(), DbErr> {

    let db = Database::connect("CONNECTION").await?;

    // find works with all()
    let mut query_all = EntityTraitType::find().all(&db).await?;

    // compiler complains about this line: `method not found in `sea_orm::Select<EntityTraitType>`
    let mut query = EntityTraitType::find().paginate(&db, 50);

    Ok(())
}