use models::account::Account;

pub fn create_account(_new_account: actix_web::web::Json<Account>)
-> impl futures::Future<Item = actix_web::HttpResponse, Error = actix_web::Error> {
    let mut account = _new_account.into_inner();
    account.id = Some(1);
    let response = serde_json::to_string(&account).unwrap();
    log::info!("{:?}",response);

    futures::future::ok(
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(response))
}