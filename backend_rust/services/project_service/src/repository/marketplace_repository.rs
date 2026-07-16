use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MarketplaceCategory {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MarketplaceProduct {
    pub id: Uuid,
    pub name: String,
    pub category_id: Uuid,
    pub descriptions: Option<String>,
    pub visual: Option<String>,
    pub price: i32,
    pub html_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProjectCategory {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

#[derive(Clone)]
pub struct MarketplaceRepository {
    pool: PgPool,
}

impl MarketplaceRepository {
    pub fn new(pool: PgPool) -> Self {
        MarketplaceRepository { pool }
    }

    // --- Marketplace Categories ---

    pub async fn list_categories(&self, query: &PaginationQuery) -> Result<PaginatedResponse<MarketplaceCategory>> {
        let (has_search, search_param) = if let Some(ref s) = query.search {
            let s = s.trim();
            if !s.is_empty() {
                (true, format!("%{}%", s))
            } else {
                (false, String::new())
            }
        } else {
            (false, String::new())
        };

        let base = if has_search {
            format!("SELECT * FROM marketplace_categories WHERE name ILIKE $1")
        } else {
            "SELECT * FROM marketplace_categories".to_string()
        };

        let count = if has_search {
            "SELECT COUNT(*) FROM marketplace_categories WHERE name ILIKE $1".to_string()
        } else {
            "SELECT COUNT(*) FROM marketplace_categories".to_string()
        };

        let total: i64 = if has_search {
            sqlx::query_scalar(&count).bind(&search_param).fetch_one(&self.pool).await?
        } else {
            sqlx::query_scalar(&count).fetch_one(&self.pool).await?
        };

        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10).max(1).min(100);
        let offset = (page - 1) * page_size;

        let data = if has_search {
            let q = format!("{} ORDER BY created_at DESC LIMIT $2 OFFSET $3", base);
            sqlx::query_as::<_, MarketplaceCategory>(&q)
                .bind(&search_param)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool).await?
        } else {
            let q = format!("{} ORDER BY created_at DESC LIMIT $1 OFFSET $2", base);
            sqlx::query_as::<_, MarketplaceCategory>(&q)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool).await?
        };

        let total_pages = ((total as f64) / (page_size as f64)).ceil() as i32;

        Ok(PaginatedResponse { data, total, page, page_size, total_pages })
    }

    pub async fn create_category(&self, name: &str) -> Result<MarketplaceCategory> {
        let cat = sqlx::query_as::<_, MarketplaceCategory>(
            r#"INSERT INTO marketplace_categories (name) VALUES ($1) RETURNING *"#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        Ok(cat)
    }

    pub async fn update_category(&self, id: Uuid, name: &str) -> Result<Option<MarketplaceCategory>> {
        let cat = sqlx::query_as::<_, MarketplaceCategory>(
            r#"UPDATE marketplace_categories SET name = $1, updated_at = NOW() WHERE id = $2 RETURNING *"#,
        )
        .bind(name)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(cat)
    }

    pub async fn delete_category(&self, id: Uuid) -> Result<bool> {
        let r = sqlx::query("DELETE FROM marketplace_categories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn get_category_by_id(&self, id: Uuid) -> Result<Option<MarketplaceCategory>> {
        let cat = sqlx::query_as::<_, MarketplaceCategory>(
            "SELECT * FROM marketplace_categories WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(cat)
    }

    // --- Marketplace Products ---

    pub async fn list_products(&self, query: &PaginationQuery, category_id: Option<Uuid>) -> Result<PaginatedResponse<MarketplaceProduct>> {
        let (has_search, search_param) = if let Some(ref s) = query.search {
            let s = s.trim();
            if !s.is_empty() {
                (true, format!("%{}%", s))
            } else {
                (false, String::new())
            }
        } else {
            (false, String::new())
        };

        let mut conditions = vec!["1=1".to_string()];
        let mut params: Vec<String> = vec![];
        let mut param_idx = 1;

        if has_search {
            conditions.push(format!("name ILIKE ${}", param_idx));
            params.push(search_param.clone());
            param_idx += 1;
        }

        if let Some(cat_id) = category_id {
            conditions.push(format!("category_id = ${}::uuid", param_idx));
            params.push(cat_id.to_string());
            param_idx += 1;
        }

        let where_clause = conditions.join(" AND ");
        let count_sql = format!("SELECT COUNT(*) FROM marketplace_products WHERE {}", where_clause);

        let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
        for p in &params {
            count_q = count_q.bind(p.clone());
        }
        let total: i64 = count_q.fetch_one(&self.pool).await?;

        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10).max(1).min(100);
        let offset = (page - 1) * page_size;

        let data_sql = format!(
            "SELECT * FROM marketplace_products WHERE {} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
            where_clause, param_idx, param_idx + 1
        );
        let mut data_q = sqlx::query_as::<_, MarketplaceProduct>(&data_sql);
        for p in &params {
            data_q = data_q.bind(p.clone());
        }
        let data: Vec<MarketplaceProduct> = data_q.bind(page_size).bind(offset).fetch_all(&self.pool).await?;

        let total_pages = ((total as f64) / (page_size as f64)).ceil() as i32;

        Ok(PaginatedResponse { data, total, page, page_size, total_pages })
    }

    pub async fn create_product(
        &self,
        name: &str,
        category_id: Uuid,
        descriptions: &str,
        visual: &str,
        price: i32,
        html_code: Option<&str>,
    ) -> Result<MarketplaceProduct> {
        let prod = sqlx::query_as::<_, MarketplaceProduct>(
            r#"INSERT INTO marketplace_products (name, category_id, descriptions, visual, price, html_code)
               VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
        )
        .bind(name)
        .bind(category_id)
        .bind(descriptions)
        .bind(visual)
        .bind(price)
        .bind(html_code)
        .fetch_one(&self.pool)
        .await?;
        Ok(prod)
    }

    pub async fn get_product_by_id(&self, id: Uuid) -> Result<Option<MarketplaceProduct>> {
        let prod = sqlx::query_as::<_, MarketplaceProduct>(
            "SELECT * FROM marketplace_products WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(prod)
    }

    pub async fn update_product(
        &self,
        id: Uuid,
        name: &str,
        category_id: Uuid,
        descriptions: &str,
        visual: &str,
        price: i32,
        html_code: Option<&str>,
    ) -> Result<Option<MarketplaceProduct>> {
        let prod = sqlx::query_as::<_, MarketplaceProduct>(
            r#"UPDATE marketplace_products
               SET name = $1, category_id = $2, descriptions = $3, visual = $4, price = $5, html_code = $6, updated_at = NOW()
               WHERE id = $7 RETURNING *"#,
        )
        .bind(name)
        .bind(category_id)
        .bind(descriptions)
        .bind(visual)
        .bind(price)
        .bind(html_code)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(prod)
    }

    pub async fn delete_product(&self, id: Uuid) -> Result<bool> {
        let r = sqlx::query("DELETE FROM marketplace_products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    // --- Project Categories ---

    pub async fn list_project_categories(&self, query: &PaginationQuery) -> Result<PaginatedResponse<ProjectCategory>> {
        let (has_search, search_param) = if let Some(ref s) = query.search {
            let s = s.trim();
            if !s.is_empty() {
                (true, format!("%{}%", s))
            } else {
                (false, String::new())
            }
        } else {
            (false, String::new())
        };

        let base = if has_search {
            format!("SELECT * FROM project_categories WHERE name ILIKE $1")
        } else {
            "SELECT * FROM project_categories".to_string()
        };

        let count = if has_search {
            "SELECT COUNT(*) FROM project_categories WHERE name ILIKE $1".to_string()
        } else {
            "SELECT COUNT(*) FROM project_categories".to_string()
        };

        let total: i64 = if has_search {
            sqlx::query_scalar(&count).bind(&search_param).fetch_one(&self.pool).await?
        } else {
            sqlx::query_scalar(&count).fetch_one(&self.pool).await?
        };

        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10).max(1).min(100);
        let offset = (page - 1) * page_size;

        let data = if has_search {
            let q = format!("{} ORDER BY created_at DESC LIMIT $2 OFFSET $3", base);
            sqlx::query_as::<_, ProjectCategory>(&q)
                .bind(&search_param)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool).await?
        } else {
            let q = format!("{} ORDER BY created_at DESC LIMIT $1 OFFSET $2", base);
            sqlx::query_as::<_, ProjectCategory>(&q)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool).await?
        };

        let total_pages = ((total as f64) / (page_size as f64)).ceil() as i32;

        Ok(PaginatedResponse { data, total, page, page_size, total_pages })
    }

    pub async fn create_project_category(&self, name: &str) -> Result<ProjectCategory> {
        let cat = sqlx::query_as::<_, ProjectCategory>(
            r#"INSERT INTO project_categories (name) VALUES ($1) RETURNING *"#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        Ok(cat)
    }

    pub async fn update_project_category(&self, id: Uuid, name: &str) -> Result<Option<ProjectCategory>> {
        let cat = sqlx::query_as::<_, ProjectCategory>(
            r#"UPDATE project_categories SET name = $1, updated_at = NOW() WHERE id = $2 RETURNING *"#,
        )
        .bind(name)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(cat)
    }

    pub async fn delete_project_category(&self, id: Uuid) -> Result<bool> {
        let r = sqlx::query("DELETE FROM project_categories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn get_project_category_by_id(&self, id: Uuid) -> Result<Option<ProjectCategory>> {
        let cat = sqlx::query_as::<_, ProjectCategory>(
            "SELECT * FROM project_categories WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(cat)
    }
}
