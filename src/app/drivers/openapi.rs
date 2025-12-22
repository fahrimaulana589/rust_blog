use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        crate::app::features::auth::interface::controller::login,
        crate::app::features::auth::interface::controller::forgot_password,
        crate::app::features::auth::interface::controller::reset_password,
        // Home
        crate::app::features::home::interface::controllers::count,
        crate::app::features::home::interface::controllers::send_email,
        // Blog Categories
        crate::app::features::blog::interface::controller::create_category,
        crate::app::features::blog::interface::controller::get_categories,
        crate::app::features::blog::interface::controller::get_category,
        crate::app::features::blog::interface::controller::update_category,
        crate::app::features::blog::interface::controller::delete_category,
        // Blog Tags
        crate::app::features::blog::interface::controller::create_tag,
        crate::app::features::blog::interface::controller::get_tags,
        crate::app::features::blog::interface::controller::get_tag,
        crate::app::features::blog::interface::controller::update_tag,
        crate::app::features::blog::interface::controller::delete_tag,
        // Blogs
        crate::app::features::blog::interface::controller::create_blog,
        crate::app::features::blog::interface::controller::get_blogs,
        crate::app::features::blog::interface::controller::get_blog,
        crate::app::features::blog::interface::controller::update_blog,
        crate::app::features::blog::interface::controller::delete_blog,
        // Projects
        crate::app::features::projects::interface::controller::create_project,
        crate::app::features::projects::interface::controller::get_all_projects,
        crate::app::features::projects::interface::controller::get_project,
        crate::app::features::projects::interface::controller::update_project,
        crate::app::features::projects::interface::controller::delete_project,
        // Stacks
        crate::app::features::projects::interface::controller::create_stack,
        crate::app::features::projects::interface::controller::get_all_stacks,
        crate::app::features::projects::interface::controller::get_stack,
        crate::app::features::projects::interface::controller::update_stack,
        crate::app::features::projects::interface::controller::delete_stack,
    ),
    components(
        schemas(
            // Utility
            crate::utils::error_response::ErrorResponse,
            // Auth
            crate::app::features::auth::interface::dto::LoginRequestDto,
            crate::app::features::auth::interface::dto::LoginResponseDto,
            crate::app::features::auth::interface::dto::ForgotPasswordRequestDto,
            crate::app::features::auth::interface::dto::ResetPasswordRequestDto,
            // Blog
            crate::app::features::blog::interface::dto::CreateCategoryRequestDto,
            crate::app::features::blog::interface::dto::CategoryResponseDto,
            crate::app::features::blog::interface::dto::CreateTagRequestDto,
            crate::app::features::blog::interface::dto::TagResponseDto,
            crate::app::features::blog::interface::dto::CreateBlogRequestDto,
            crate::app::features::blog::interface::dto::BlogResponseDto,
            // Projects
            crate::app::features::projects::interface::dto::CreateProjectRequestDto,
            crate::app::features::projects::interface::dto::UpdateProjectRequestDto,
            crate::app::features::projects::interface::dto::ProjectResponseDto,
            crate::app::features::projects::interface::dto::CreateStackRequestDto,
            crate::app::features::projects::interface::dto::UpdateStackRequestDto,
            crate::app::features::projects::interface::dto::StackResponseDto,
        )
    ),
    modifiers(&SecurityAddon),
    security(
        ("jwt" = [])
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Home", description = "Home endpoints"),
        (name = "Blog", description = "Blog management"),
        (name = "Projects", description = "Project portfolio management"),
    ),
    info(
        title = "MyBlog API",
        version = "0.1.0",
        description = "API documentation for MyBlog application"
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
