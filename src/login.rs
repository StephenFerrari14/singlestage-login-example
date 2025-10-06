use leptos::prelude::*;
use singlestage::{Button, Checkbox, Input, Label, Link};

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <body class="min-h-screen flex items-center justify-center">
            <div class="max-w-md w-full">
                <div class="text-center">
                    <div class="mx-auto h-16 w-16 flex items-center justify-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.8" d="M12 15v2m0-8a3 3 0 00-3 3v1h6v-1a3 3 0 00-3-3zm6 2v7a2 2 0 01-2 2H6a2 2 0 01-2-2v-7a2 2 0 012-2h12a2 2 0 012 2zM8 9V7a4 4 0 118 0v2" />
                        </svg>
                    </div>

                    <h1 class="mt-4 text-2xl font-semibold">"Sign in to your account"</h1>
                    <p class="mt-2 text-sm text-gray-400">"Welcome back — please enter your details."</p>
                </div>

                <form class="mt-8 space-y-6" action="#" method="POST" novalidate>
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div class="mb-4">
                            <Label class="grid gap-2">"Email" <Input input_type="email" /></Label>
                        </div>
                        <div>
                            <Label class="grid gap-2">"Password" <Input input_type="password" /></Label>
                        </div>
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="flex items-center">
                            <Label>
                                <Checkbox />
                                "Remember Me"
                            </Label>
                        </div>
                        <div class="text-sm">
                        <Link href="#">"Forgot password?"</Link>
                        </div>
                    </div>

                    <div class="text-center">
                        <Button>"Sign In"</Button>
                    </div>

                    <div class="text-center text-sm text-gray-400">
                        "Don't have an account? "
                        <Link href="#">"Sign up"</Link>
                    </div>
                </form>
                <p class="mt-6 text-xs text-center text-gray-400">"By signing in you agree to our "<a href="#" class="text-indigo-500 hover:underline">terms</a> " and " <a href="#" class="text-indigo-500 hover:underline">privacy policy</a>.</p>
            </div>
        </body>
    }
}
