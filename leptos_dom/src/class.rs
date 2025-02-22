use leptos_reactive::Scope;

pub enum Class {
    Value(bool),
    Fn(Box<dyn Fn() -> bool>),
}

pub trait IntoClass {
    fn into_class(self, cx: Scope) -> Class;
}

impl IntoClass for bool {
    fn into_class(self, _cx: Scope) -> Class {
        Class::Value(self)
    }
}

impl<T> IntoClass for T
where
    T: Fn() -> bool + 'static,
{
    fn into_class(self, _cx: Scope) -> Class {
        let modified_fn = Box::new(self);
        Class::Fn(modified_fn)
    }
}

impl Class {
    pub fn as_value_string(&self, class_name: &'static str) -> &'static str {
        match self {
            Class::Value(value) => {
                if *value {
                    class_name
                } else {
                    ""
                }
            }
            Class::Fn(f) => {
                let value = f();
                if value {
                    class_name
                } else {
                    ""
                }
            }
        }
    }
}
