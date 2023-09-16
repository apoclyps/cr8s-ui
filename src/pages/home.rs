use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::{contexts::CurrentUserContext, Route};

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    match &current_user_ctx.user {
        Some(user) => {
            html! {
                <div class="row">
                    <div class="col">
                        <Sidebar />
                    </div>
                    <div class="col">
                        <p class="text-center">
                            <Header />
                        </p>
                    </div>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}
