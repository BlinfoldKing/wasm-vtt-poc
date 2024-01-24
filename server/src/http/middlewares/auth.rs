use std::future::{ready, Ready};

use crate::{
    config::Cfg,
    entities::jwt::JWTToken,
    error::{Error as AppError, ErrorKind},
    http::Response,
};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use tracing::debug;
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        req.request().headers().get("Authorization");
        if req.path() == "/v1/auth/login" {
            let fut = self.service.call(req);
            return Box::pin(async { fut.await.map(ServiceResponse::map_into_left_body) });
        }

        match req.headers().get("Authorization") {
            None => {
                return Box::pin(async move {
                    let (request, _) = req.into_parts();

                    let error = AppError::new(
                        "no Bearer token found".to_owned(),
                        ErrorKind::Unauthenticated,
                    );

                    Ok(ServiceResponse::new(
                        request,
                        Response::err(error).map_into_right_body(),
                    ))
                });
            }
            Some(token) => {
                let tokens: Vec<&str> = token.to_str().unwrap().split(" ").collect();

                if tokens.len() < 2 || tokens[0] != "Bearer" {
                    return Box::pin(async move {
                        let (request, _) = req.into_parts();

                        let error = AppError::new(
                            "invalid Bearer token found".to_owned(),
                            ErrorKind::Unauthenticated,
                        );

                        Ok(ServiceResponse::new(
                            request,
                            Response::err(error).map_into_right_body(),
                        ))
                    });
                }

                let config = req.app_data::<web::Data<Cfg>>().unwrap();

                let res = JWTToken::from(tokens[1]).verify(config.general.secret.clone());
                if let Ok(user) = res {
                    req.extensions_mut().insert(user);
                    let fut = self.service.call(req);
                    Box::pin(async { fut.await.map(ServiceResponse::map_into_left_body) })
                } else {
                    return Box::pin(async move {
                        let (request, _) = req.into_parts();

                        let error = AppError::new(
                            "invalid Bearer token found".to_owned(),
                            ErrorKind::Unauthenticated,
                        );

                        Ok(ServiceResponse::new(
                            request,
                            Response::err(error).map_into_right_body(),
                        ))
                    });
                }
            }
        }
    }
}
