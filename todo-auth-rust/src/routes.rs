use actix_web::web;
use actix_web::web::{delete, get, post, put};
use actix_web_middleware_keycloak_auth::{ AlwaysReturnPolicy, DecodingKey, KeycloakAuth, Role, StandardKeycloakClaims };
use crate::health_check::health_check_api::get_health_check;
use crate::todo::todo_api::{create_todo, delete_todo, get_page_todo, get_todo_by_id, get_todo_list, update_todo};

// FROM KEYCLOAK check for correct <<kid>>
const KEYCLOAK_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIICrzCCAZcCBgGJfodLqTANBgkqhkiG9w0BAQsFADAbMRkwFwYDVQQDDBBsZWFybi1ydXN0LXJlYWxtMB4XDTIzMDcyMjE2NTQwM1oXDTMzMDcyMjE2NTU0M1owGzEZMBcGA1UEAwwQbGVhcm4tcnVzdC1yZWFsbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALicSu3bGqJIdzG/SBvAXRLb0eJAQUczxR4Ilfsiz+5BMH5vez4PG4RKybZprksDBuH1xQ5iR8UR3QvenjXQ025hAKh3+cPRtDUd+eQP6pDoxCa83CzCMX4gspDEUEhAjnhxGZ63UJG07exEUxqEYeZ8GSmOhraZNt9qI2h77/fOh75L2W2joCAXh2Lwquh87jRDJHqop8nF/wOqpU6mkcwLvFD4/B579zi3MAr8LykWuYVgedhcJSdlEQBisKMhR4GEsZaPuePk6PVPRYsaHRxQppzxYJB/aOWlvNQ6QLPC2MAs6BTleKguqgHGTBv8wEYmLixU8IwT59Hy/xRtqxUCAwEAATANBgkqhkiG9w0BAQsFAAOCAQEARIEhEifJebMxOFVtYlUD1Dhf7KhxFMTjMTUUcAQ+S036gZrr9s9S5XKRHLMknGq1VJe/kjfClRR97VxDOWKdz/E1VdhjXB6UvGG92vWfjGtmsHfc54D2Qx1bc587GoTql4WWDsp5H+25u32A89NCzkxLfnCOqnvRg74LRH2a02JpOReYtwywQFMy4fv2YIltaavwet+bv5pwbOv/fenFMWVPxSZmUqkcLQkYx1emRIjD28QFmKDXjo+y6moNmVoM4zbRJYgcniODYRPIpgjJnH0IhucwrarnbFxkkUyh8q9UJTzgZo/nTG+AvSec937HOZ7KrCY+sBBeXj5z9niPrQ==
-----END PUBLIC KEY-----";
pub fn routes(cfg: &mut web::ServiceConfig) {

    let keycloak_auth = KeycloakAuth::default_with_pk(DecodingKey::from_rsa_pem(KEYCLOAK_PUBLIC_KEY.as_bytes()).unwrap());

    /****
    let keycloak_auth = KeycloakAuth {
        detailed_responses: true,
        passthrough_policy: AlwaysReturnPolicy,
        keycloak_oid_public_key: DecodingKey::from_rsa_pem(KEYCLOAK_PUBLIC_KEY.as_bytes()).unwrap(),
        required_roles: vec![Role::Realm {
            role: "test".to_owned(),
        }],
    };
    ****/

    cfg.service(
        web::scope("/api")
            .service(web::scope("/healthcheck")
                .route("", get().to(get_health_check))
            )
            .service(
                web::scope("/todo")
                    .wrap(keycloak_auth)
                    .route("", get().to(get_todo_list))
                    .route("/params", get().to(get_page_todo))
                    .route("/{id}", get().to(get_todo_by_id))
                    .route("", post().to(create_todo))
                    .route("", put().to(update_todo))
                    .route("/{id}", delete().to(delete_todo)),
            )
    );
}