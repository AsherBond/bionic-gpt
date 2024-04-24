use super::super::{Authentication, CustomError};
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::queries::documents;
use db::Pool;
use web_pages::{render_with_props};

pub async fn row(
    Path((team_id, document_id)): Path<(i32, i32)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let document = documents::document()
        .bind(&transaction, &document_id)
        .one()
        .await?;

    let html = render_with_props(
        web_pages::documents::index::Row,
        web_pages::documents::index::RowProps {
            team_id,
            document,
            first_time: false,
        },
    );

    Ok(Html(html))
}