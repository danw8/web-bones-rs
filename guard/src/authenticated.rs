use rocket::Outcome::{Success, Forward};
use rocket::request::{Outcome, FromRequest};
use rocket::Request;
use service::UserService;

pub struct Authenticated(bool);

impl<'a, 'r> FromRequest<'a, 'r> for Authenticated {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let outcome = UserService::from_request(request);
        if outcome.is_success(){
            let user_service = outcome.unwrap();
            if user_service.is_authenticated() {
                return Success(Authenticated(true));
            }
        }
        return Forward(());
	}
}