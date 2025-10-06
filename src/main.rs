use leptos::prelude::*;
use singlestage::ThemeProvider;

use crate::login::Login;

mod login;

fn main() {
    leptos::mount::mount_to_body(|| view! { 
        <ThemeProvider>
            <Login/> 
        </ThemeProvider>
    })
}
