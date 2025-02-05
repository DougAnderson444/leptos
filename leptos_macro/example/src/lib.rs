use leptos::*;

#[component]
pub fn TestComponent(
    _cx: Scope,
    /// Rust code
    /// ```
    /// assert_eq!("hello", stringify!(hello));
    /// ```
    /// View containing rust code
    /// ```view
    /// assert!(true);
    /// ```
    /// View containing rsx
    /// ```view
    /// # use example::TestComponent;
    /// <TestComponent key="hello"/>
    /// ```
    /// View containing rsx
    /// ```view compile_fail
    /// # use example::TestComponent;
    /// <TestComponent/>
    /// ```
    #[prop(into)]
    key: String,
    /// rsx unclosed
    /// ```view
    /// # use example::TestComponent;
    /// <TestComponent key="hello"/>
    #[prop(optional)]
    another:usize,
    /// rust unclosed
    /// ```view
    /// use example::TestComponent;
    #[prop(optional)]
    and_another: usize,
) -> impl IntoView {
    _ = (key, another, and_another);
}

