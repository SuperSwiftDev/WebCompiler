pub trait Effectful: Clone + Default {
    fn extend(&mut self, other: Self);
}

/// A monadic-like effect-tracking computation structure.
pub struct IO<Value, Effect: Effectful> {
    value: Value,
    effect: Effect,
}


impl<Value, Effect: Effectful> IO<Value, Effect> {
    pub fn wrap(value: Value) -> IO<Value, Effect> {
        IO { value: value, effect: Effect::default() }
    }
    pub fn map<Result>(self, apply: impl FnOnce(Value) -> Result) -> IO<Result, Effect> {
        let IO { value, effect } = self;
        let value = apply(value);
        IO { value, effect }
    }
    pub fn map_with_context<Result>(self, apply: impl FnOnce(Value, &Effect) -> Result) -> IO<Result, Effect> {
        let IO { value, effect } = self;
        let value = apply(value, &effect);
        IO { value, effect }
    }
    pub fn map_with_context_mut<Result>(self, apply: impl FnOnce(Value, &mut Effect) -> Result) -> IO<Result, Effect> {
        let IO { value, mut effect } = self;
        let value = apply(value, &mut effect);
        IO { value, effect }
    }
    pub fn and_then<Result>(self, apply: impl FnOnce(Value) -> IO<Result, Effect>) -> IO<Result, Effect> {
        let IO { value, mut effect } = self;
        let IO { value, effect: effect2 } = apply(value);
        effect.extend(effect2);
        IO { value, effect }
    }
    pub fn and_then_with_context<Result>(self, apply: impl FnOnce(Value, &Effect) -> IO<Result, Effect>) -> IO<Result, Effect> {
        let IO { value, mut effect } = self;
        let IO { value, effect: effect2 } = apply(value, &effect);
        effect.extend(effect2);
        IO { value, effect }
    }
    pub fn and_then_with_context_mut<Result>(self, apply: impl FnOnce(Value, &mut Effect) -> IO<Result, Effect>) -> IO<Result, Effect> {
        let IO { value, mut effect } = self;
        let IO { value, effect: effect2 } = apply(value, &mut effect);
        effect.extend(effect2);
        IO { value, effect }
    }
    pub fn and_modify_context(self, apply: impl FnOnce(&mut Effect) -> ()) -> IO<Value, Effect> {
        let IO { value, mut effect } = self;
        apply(&mut effect);
        IO { value, effect }
    }
    pub fn flatten(size_hint: usize, items: impl IntoIterator<Item=IO<Value, Effect>>) -> IO<Vec<Value>, Effect> {
        let initial_state = IO::<Vec<Value>, Effect>::wrap(Vec::with_capacity(size_hint));
        items
            .into_iter()
            .fold(initial_state, |mut acc, item| {
                let IO { value, effect } = item;
                acc.value.push(value);
                acc.effect.extend(effect);
                acc
            })
    }
    pub fn flatten_vec(items: Vec<IO<Value, Effect>>) -> IO<Vec<Value>, Effect> {
        Self::flatten(items.len(), items)
    }
    pub fn flatten_vec_deep(items: Vec<IO<Vec<Value>, Effect>>) -> IO<Vec<Value>, Effect> {
        let initial_state = IO::<Vec<Value>, Effect>::wrap(Vec::with_capacity(items.len()));
        items
            .into_iter()
            .fold(initial_state, |mut acc, item| {
                let IO { value, effect } = item;
                acc.value.extend(value);
                acc.effect.extend(effect);
                acc
            })
    }
    pub fn collapse(self) -> (Value, Effect) {
        let IO { value, effect } = self;
        (value, effect)
    }
}

pub fn io_iter_map<Input, Output, Effect: Effectful>(
    size_hint: usize,
    items: impl IntoIterator<Item=Input>,
    apply: impl Fn(Input) -> IO<Output, Effect>,
) -> IO<Vec<Output>, Effect> {
    let outputs = items
        .into_iter()
        .map(|x| apply(x));
    IO::<Output, Effect>::flatten(size_hint, outputs)
}

pub fn io_iter_map_mut<Input, Output, Effect: Effectful>(
    size_hint: usize,
    items: impl IntoIterator<Item=Input>,
    mut apply: impl FnMut(Input) -> IO<Output, Effect>,
) -> IO<Vec<Output>, Effect> {
    let outputs = items
        .into_iter()
        .map(|x| apply(x));
    IO::<Output, Effect>::flatten(size_hint, outputs)
}

pub fn io_vec_map<Input, Output, Effect: Effectful>(
    items: Vec<Input>,
    apply: impl Fn(Input) -> IO<Output, Effect>,
) -> IO<Vec<Output>, Effect> {
    let items_len = items.len();
    io_iter_map(items_len, items, apply)
}

