use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    HttpMessage,
    HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use actix_web::http::StatusCode;
use crate::utils::jwt::Claims;
use crate::utils::response::GenericResponse;
use crate::utils::roles::Role;

pub struct RbacMiddleware {
    pub allowed_roles: Vec<Role>,
}

impl<S, B> Transform<S, ServiceRequest> for RbacMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RbacMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RbacMiddlewareService {
            service,
            allowed_roles: self.allowed_roles.clone(),
        })
    }
}

pub struct RbacMiddlewareService<S> {
    service: S,
    allowed_roles: Vec<Role>,
}

impl<S, B> Service<ServiceRequest> for RbacMiddlewareService<S>
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
        // Check if claims exist in the request extensions
        let claims_opt = req.extensions().get::<Claims>().cloned();
        
        if claims_opt.is_none() {
            let response = HttpResponse::Unauthorized()
                .json(GenericResponse {
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    data: None::<()>,
                    message: "Unauthorized".to_string(),
                });
            
            let (http_req, _) = req.into_parts();
            return Box::pin(async move {
                Ok(ServiceResponse::new(
                    http_req,
                    response.map_into_right_body(),
                ))
            });
        }
        
        let claims = claims_opt.unwrap();
        let role_result = Role::from_str(&claims.role);
        
        if role_result.is_err() {
            let response = HttpResponse::Forbidden()
                .json(GenericResponse {
                    status: StatusCode::FORBIDDEN.as_u16(),
                    data: None::<()>,
                    message: "Invalid role".to_string(),
                });
            
            let (http_req, _) = req.into_parts();
            return Box::pin(async move {
                Ok(ServiceResponse::new(
                    http_req,
                    response.map_into_right_body(),
                ))
            });
        }
        
        let user_role = role_result.unwrap();
        
        if !self.allowed_roles.contains(&user_role) {
            let response = HttpResponse::Forbidden()
                .json(GenericResponse {
                    status: StatusCode::FORBIDDEN.as_u16(),
                    data: None::<()>,
                    message: format!(
                        "One of roles {:?} required",
                        self.allowed_roles.iter().map(|r| r.as_str()).collect::<Vec<_>>()
                    ),
                });
            
            let (http_req, _) = req.into_parts();
            return Box::pin(async move {
                Ok(ServiceResponse::new(
                    http_req,
                    response.map_into_right_body(),
                ))
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}