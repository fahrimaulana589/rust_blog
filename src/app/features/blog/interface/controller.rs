use crate::app::features::blog::interface::dto::{
    BlogResponseDto, CategoryResponseDto, CreateBlogRequestDto, CreateCategoryRequestDto,
    CreateTagRequestDto, PaginationRequestDto, TagResponseDto, UpdateBlogRequestDto,
    UpdateCategoryRequestDto, UpdateTagRequestDto,
};
use crate::utils::di::Container;
use crate::utils::error_response::{ErrorResponse, map_string_error, map_validation_error};
use crate::utils::success_response::{map_success_response, map_success_with_data};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use validator::Validate;

#[utoipa::path(
    path = "/app/categories",
    tag = "Blog",
    request_body = CreateCategoryRequestDto,
    responses(
        (status = 201, description = "Category created", body = crate::utils::success_response::SuccessResponse<CategoryResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/categories")]
pub async fn create_category(
    container: web::Data<Container>,
    payload: web::Json<CreateCategoryRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .create_category_usecase
        .execute(payload.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(map_success_response(
            "Category created successfully".to_string(),
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/categories",
    tag = "Blog",
    responses(
        (status = 200, description = "List categories", body = crate::utils::success_response::SuccessResponse<Vec<CategoryResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/categories")]
pub async fn get_categories(
    container: web::Data<Container>,
    query: web::Query<PaginationRequestDto>,
) -> impl Responder {
    match container
        .get_categories_usecase
        .execute(query.into_inner())
        .await
    {
        Ok(categories) => HttpResponse::Ok().json(map_success_with_data(
            "Categories fetched successfully".to_string(),
            categories,
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/categories/{id}",
    tag = "Blog",
    params(
        ("id", description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category found", body = crate::utils::success_response::SuccessResponse<CategoryResponseDto>),
        (status = 404, description = "Category not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/categories/{id}")]
pub async fn get_category(container: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    match container
        .get_category_usecase
        .execute(id.into_inner())
        .await
    {
        Ok(category) => HttpResponse::Ok().json(map_success_with_data(
            "Category fetched successfully".to_string(),
            category,
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(map_string_error(e))
            } else {
                HttpResponse::InternalServerError().json(map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    path = "/app/categories/{id}",
    tag = "Blog",
    params(
        ("id", description = "Category ID")
    ),
    request_body = UpdateCategoryRequestDto,
    responses(
        (status = 200, description = "Category updated", body = crate::utils::success_response::SuccessResponse<CategoryResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/categories/{id}")]
pub async fn update_category(
    container: web::Data<Container>,
    id: web::Path<i32>,
    payload: web::Json<UpdateCategoryRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .update_category_usecase
        .execute(id.into_inner(), payload.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(map_success_response(
            "Category updated successfully".to_string(),
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/categories/{id}",
    tag = "Blog",
    params(
        ("id", description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category deleted", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 404, description = "Category not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/categories/{id}")]
pub async fn delete_category(
    container: web::Data<Container>,
    id: web::Path<i32>,
) -> impl Responder {
    match container
        .delete_category_usecase
        .execute(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(map_success_response(
            "Category deleted successfully".to_string(),
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(map_string_error(e))
            } else {
                HttpResponse::InternalServerError().json(map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    path = "/app/tags",
    tag = "Blog",
    request_body = CreateTagRequestDto,
    responses(
        (status = 200, description = "Tag created", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/tags")]
pub async fn create_tag(
    container: web::Data<Container>,
    payload: web::Json<CreateTagRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .create_tag_usecase
        .execute(payload.into_inner())
        .await
    {
        Ok(_) => {
            HttpResponse::Ok().json(map_success_response("Tag created successfully".to_string()))
        }
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/tags",
    tag = "Blog",
    responses(
        (status = 200, description = "List tags", body = crate::utils::success_response::SuccessResponse<Vec<TagResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/tags")]
pub async fn get_tags(
    container: web::Data<Container>,
    query: web::Query<PaginationRequestDto>,
) -> impl Responder {
    match container.get_tags_usecase.execute(query.into_inner()).await {
        Ok(tags) => HttpResponse::Ok().json(map_success_with_data(
            "Tags fetched successfully".to_string(),
            tags,
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/tags/{id}",
    tag = "Blog",
    params(
        ("id", description = "Tag ID")
    ),
    responses(
        (status = 200, description = "Tag found", body = crate::utils::success_response::SuccessResponse<TagResponseDto>),
        (status = 404, description = "Tag not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/tags/{id}")]
pub async fn get_tag(container: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    match container.get_tag_usecase.execute(id.into_inner()).await {
        Ok(tag) => HttpResponse::Ok().json(map_success_with_data(
            "Tag fetched successfully".to_string(),
            tag,
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(map_string_error(e))
            } else {
                HttpResponse::InternalServerError().json(map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    path = "/app/tags/{id}",
    tag = "Blog",
    params(
        ("id", description = "Tag ID")
    ),
    request_body = UpdateTagRequestDto,
    responses(
        (status = 200, description = "Tag updated", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/tags/{id}")]
pub async fn update_tag(
    container: web::Data<Container>,
    id: web::Path<i32>,
    payload: web::Json<UpdateTagRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .update_tag_usecase
        .execute(id.into_inner(), payload.into_inner())
        .await
    {
        Ok(_) => {
            HttpResponse::Ok().json(map_success_response("Tag updated successfully".to_string()))
        }
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/tags/{id}",
    tag = "Blog",
    params(
        ("id", description = "Tag ID")
    ),
    responses(
        (status = 200, description = "Tag deleted", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 404, description = "Tag not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/tags/{id}")]
pub async fn delete_tag(container: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    match container.delete_tag_usecase.execute(id.into_inner()).await {
        Ok(_) => {
            HttpResponse::Ok().json(map_success_response("Tag deleted successfully".to_string()))
        }
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(map_string_error(e))
            } else {
                HttpResponse::InternalServerError().json(map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    path = "/app/blogs",
    tag = "Blog",
    request_body = CreateBlogRequestDto,
    responses(
        (status = 200, description = "Blog created", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/blogs")]
pub async fn create_blog(
    container: web::Data<Container>,
    payload: web::Json<CreateBlogRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .create_blog_usecase
        .execute(payload.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(map_success_response(
            "Blog created successfully".to_string(),
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/blogs",
    tag = "Blog",
    responses(
        (status = 200, description = "List blogs", body = crate::utils::success_response::SuccessResponse<Vec<BlogResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/blogs")]
pub async fn get_blogs(
    container: web::Data<Container>,
    query: web::Query<PaginationRequestDto>,
) -> impl Responder {
    match container
        .get_blogs_usecase
        .execute(query.into_inner())
        .await
    {
        Ok(blogs) => HttpResponse::Ok().json(map_success_with_data(
            "Blogs fetched successfully".to_string(),
            blogs,
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/blogs/{id}",
    tag = "Blog",
    params(
        ("id", description = "Blog ID")
    ),
    responses(
        (status = 200, description = "Blog found", body = crate::utils::success_response::SuccessResponse<BlogResponseDto>),
        (status = 404, description = "Blog not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/blogs/{id}")]
pub async fn get_blog(container: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    match container.get_blog_usecase.execute(id.into_inner()).await {
        Ok(blog) => HttpResponse::Ok().json(map_success_with_data(
            "Blog fetched successfully".to_string(),
            blog,
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(map_string_error(e))
            } else {
                HttpResponse::InternalServerError().json(map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    path = "/app/blogs/{id}",
    tag = "Blog",
    params(
        ("id", description = "Blog ID")
    ),
    request_body = UpdateBlogRequestDto,
    responses(
        (status = 200, description = "Blog updated", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/blogs/{id}")]
pub async fn update_blog(
    container: web::Data<Container>,
    id: web::Path<i32>,
    payload: web::Json<UpdateBlogRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .update_blog_usecase
        .execute(id.into_inner(), payload.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(map_success_response(
            "Blog updated successfully".to_string(),
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/blogs/{id}",
    tag = "Blog",
    params(
        ("id", description = "Blog ID")
    ),
    responses(
        (status = 200, description = "Blog deleted", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 404, description = "Blog not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/blogs/{id}")]
pub async fn delete_blog(container: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    match container.delete_blog_usecase.execute(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(map_success_response(
            "Blog deleted successfully".to_string(),
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(map_string_error(e))
            } else {
                HttpResponse::InternalServerError().json(map_string_error(e))
            }
        }
    }
}
