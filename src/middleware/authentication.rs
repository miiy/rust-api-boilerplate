// https://github.com/actix/examples/blob/master/middleware/various/src/read_request_body.rs
// https://github.com/actix/examples/blob/master/middleware/rate-limit/src/rate_limit.rs
use crate::{
    auth::provider::AuthenticationProvider,
    error::{APIError, ErrorEntity},
    AppState,
};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ResponseError,
    http::{header::Header, StatusCode},
    web, Error, HttpMessage, HttpResponse,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        // parse the authorization header
        let schema_result = Authorization::<Bearer>::parse(&req);
        let schema = match schema_result {
            Ok(schema) => schema,
            Err(e) => {
                // invalid Header provided
                // println!("{}", e);
                let resp = error_response(StatusCode::UNAUTHORIZED, Box::new(e));
                return Box::pin(async move { Ok(req.into_response(resp.map_into_right_body())) });
            }
        };

        let service = Rc::clone(&self.service);

        // parse the token
        Box::pin(async move {
            // parse the token
            let token = schema.as_ref().token();
            let app_state = req.app_data::<web::Data<AppState>>().unwrap();
            let token_data = match app_state.jwt.decode(token) {
                Ok(token_data) => token_data,
                Err(e) => {
                    let resp = error_response(StatusCode::UNAUTHORIZED, Box::new(e));
                    return Ok(req.into_response(resp.map_into_right_body()));
                }
            };

            // extract the user from the token
            let sub = token_data.claims.sub;
            let user_result = app_state.auth_provider.get_user(sub).await;
            let user = match user_result {
                Ok(user) => user,
                Err(e) => {
                    let resp = error_response(StatusCode::UNAUTHORIZED, Box::new(e));
                    return Ok(req.into_response(resp.map_into_right_body()));
                }
            };

            // insert authenticated user into the request extensions
            req.extensions_mut().insert(user);

            // println!("Authorization from start. You requested: {}", req.path());
            let res = service.call(req).await?;

            // println!("response: {:?}", res.headers());
            Ok(res.map_into_left_body())
        })
    }
}

fn error_response(code: StatusCode, e: Box<dyn std::error::Error>) -> HttpResponse {
    let err = match code {
        StatusCode::BAD_REQUEST => APIError::BadRequest(ErrorEntity {
            code: 400,
            message: e.to_string(),
        }),
        StatusCode::UNAUTHORIZED => APIError::Unauthorized(ErrorEntity {
            code: 401,
            message: e.to_string(),
        }),
        _ => APIError::InternalError(ErrorEntity {
            code: 500,
            message: e.to_string(),
        }),
    };
    err.error_response()
}
