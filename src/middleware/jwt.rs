use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    HttpMessage,
    HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};
use actix_web::http::StatusCode;
use crate::utils::jwt::{validate_token, Claims};
use crate::utils::response::GenericResponse;
use crate::utils::errors::AppError;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers();
        let auth_header = match headers.get("Authorization") {
            Some(header) => header.to_str().unwrap_or(""),
            None => {
                let response = HttpResponse::Unauthorized()
                    .json(GenericResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        data: None::<()>,
                        message: "Missing Authorization header".to_string(),
                    });
                
                return Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0,
                        response.map_into_right_body(),
                    ))
                });
            },
        };

        let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
        match validate_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
            Err(_) => {
                let response = HttpResponse::Unauthorized()
                    .json(GenericResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        data: None::<()>,
                        message: "Invalid or expired token".to_string(),
                    });
                
                Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0,
                        response.map_into_right_body(),
                    ))
                })
            },
        }
    }
}