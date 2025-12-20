use crate::utils::di::Container;
use crate::utils::error_response::map_string_error;
use crate::utils::token::verify_token;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web,
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_str) = auth_header.and_then(|h| h.to_str().ok()) {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if let Some(container) = req.app_data::<web::Data<Container>>() {
                    if let Ok(_) = verify_token(token, &container.config.jwt_secret) {
                        // Token is valid
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                }
            }
        }

        Box::pin(async move {
            let error_response = map_string_error("Unauthorized".to_string());
            Err(actix_web::error::InternalError::from_response(
                "Unauthorized",
                actix_web::HttpResponse::Unauthorized().json(error_response),
            )
            .into())
        })
    }
}
