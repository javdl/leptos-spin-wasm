use leptos::{prelude::*, html::ElementChild};
use crate::components::button::{Button, ButtonVariant};

#[component]
pub fn StyleguidePage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-8">"Styleguide"</h1>
            <section class="mb-12">
                <h2 class="text-2xl font-semibold mb-4">"Buttons"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="flex flex-col items-start gap-2">
                        // <Button variant=ButtonVariant::Primary>"Primary Button"</Button>
                        <span class="text-sm text-gray-600">"Primary - Main call to action"</span>
                    </div>
                    <div class="flex flex-col items-start gap-2">
                        // <Button variant=ButtonVariant::Secondary>"Secondary Button"</Button>
                        <span class="text-sm text-gray-600">"Secondary - Alternative action"</span>
                    </div>
                </div>
            </section>
            <section class="mb-12">
                <h2 class="text-2xl font-semibold mb-4">"Typography"</h2>
                <div class="space-y-4">
                    <div>
                        <h1 class="text-4xl font-bold">"Heading 1"</h1>
                        <span class="text-sm text-gray-600">"text-4xl font-bold"</span>
                    </div>
                    <div>
                        <h2 class="text-3xl font-semibold">"Heading 2"</h2>
                        <span class="text-sm text-gray-600">"text-3xl font-semibold"</span>
                    </div>
                    <div>
                        <h3 class="text-2xl font-medium">"Heading 3"</h3>
                        <span class="text-sm text-gray-600">"text-2xl font-medium"</span>
                    </div>
                    <div>
                        <p class="text-base">"Regular paragraph text. Lorem ipsum dolor sit amet, consectetur adipiscing elit."</p>
                        <span class="text-sm text-gray-600">"text-base"</span>
                    </div>
                    <div>
                        <p class="text-sm">"Small text. Lorem ipsum dolor sit amet, consectetur adipiscing elit."</p>
                        <span class="text-sm text-gray-600">"text-sm"</span>
                    </div>
                </div>
            </section>
            <section class="mb-12">
                <h2 class="text-2xl font-semibold mb-4">"Colors"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-primary rounded"></div>
                        <span class="text-sm font-medium">"Primary"</span>
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-secondary rounded"></div>
                        <span class="text-sm font-medium">"Secondary"</span>
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-gray-200 rounded"></div>
                        <span class="text-sm font-medium">"Gray 200"</span>
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-gray-800 rounded"></div>
                        <span class="text-sm font-medium">"Gray 800"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}