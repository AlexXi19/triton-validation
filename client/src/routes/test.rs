use leptos::*;

#[component]
pub fn Test(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="p-6">
            <h2 class="text-4xl">"Testing Route"</h2>
            <h3 class="text-4xl text-blue-400">"Testing Route Sub"</h3>
        </div>
    }
}
