use leptos::{prelude::*, html::ElementChild};
use crate::components::button::{Button, ButtonVariant};
use crate::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn StyleguidePage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8 fade-in">
            <h1 class="text-3xl font-bold mb-8">"Styleguide"</h1>
            <section class="mb-12">
                <h2 class="text-2xl font-semibold mb-4">"Buttons"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="flex flex-col items-start gap-2">
                        <Button variant=ButtonVariant::Default>"Default Button"</Button>
                        <span class="text-sm text-gray-600">"Default - Standard button"</span>
                    </div>
                    <div class="flex flex-col items-start gap-2">
                        <Button variant=ButtonVariant::Destructive>"Destructive Button"</Button>
                        <span class="text-sm text-gray-600">"Destructive - Button for destructive actions"</span>
                    </div>
                    <div class="flex flex-col items-start gap-2">
                        <Button variant=ButtonVariant::Outline>"Outline Button"</Button>
                        <span class="text-sm text-gray-600">"Outline - Button with outline style"</span>
                    </div>
                    <div class="flex flex-col items-start gap-2">
                        <Button variant=ButtonVariant::Secondary>"Secondary Button"</Button>
                        <span class="text-sm text-gray-600">"Secondary - Alternative action"</span>
                    </div>
                    <div class="flex flex-col items-start gap-2">
                        <Button variant=ButtonVariant::Ghost>"Ghost Button"</Button>
                        <span class="text-sm text-gray-600">"Ghost - Button with ghost style"</span>
                    </div>
                    <div class="flex flex-col items-start gap-2">
                        <Button variant=ButtonVariant::Link>"Link Button"</Button>
                        <span class="text-sm text-gray-600">"Link - Button styled as a link"</span>
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
                        <div class="w-full h-20 bg-destructive rounded"></div>
                        <span class="text-sm font-medium">"Destructive"</span>
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-muted rounded"></div>
                        <span class="text-sm font-medium">"Muted"</span>
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-accent rounded"></div>
                        <span class="text-sm font-medium">"Accent"</span>
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-popover rounded"></div>
                        <span class="text-sm font-medium">"Popover"</span>
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="w-full h-20 bg-card rounded"></div>
                        <span class="text-sm font-medium">"Card"</span>
                    </div>
                </div>
            </section>
            <section class="mb-12">
                <h2 class="text-2xl font-semibold mb-4">"Badges"</h2>
                <div class="flex flex-wrap gap-4">
                    <Badge variant=BadgeVariant::Default>"Default Badge"</Badge>
                    <Badge variant=BadgeVariant::Secondary>"Secondary Badge"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Destructive Badge"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline Badge"</Badge>
                </div>
            </section>
        </div>
    }
}
