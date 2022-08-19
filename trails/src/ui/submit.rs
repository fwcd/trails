use druid::{Widget, Size, Env, BoxConstraints, LifeCycle, Event, PaintCtx, LayoutCtx, UpdateCtx, LifeCycleCtx, EventCtx, Data, Code};

/// A simple container handles enter events.
pub struct Submit<W, D> {
    child: W,
    on_enter: Option<Box<dyn Fn(&mut D)>>,
}

impl<W, D> Submit<W, D> {
    pub fn new(child: W) -> Self {
        Self { child, on_enter: None }
    }

    pub fn on_enter(self, on_enter: impl Fn(&mut D) + 'static) -> Self {
        Self { on_enter: Some(Box::new(on_enter)), ..self }
    }
}

impl<W, D> Widget<D> for Submit<W, D> where D: Data, W: Widget<D> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        match event {
            Event::KeyDown(e) if e.code == Code::Enter => {
                if let Some(on_enter) = &self.on_enter {
                    on_enter(data);
                    ctx.set_handled();
                }
            },
            _ => self.child.event(ctx, event, data, env)
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &D, data: &D, env: &Env) {
        self.child.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &D, env: &Env) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        self.child.paint(ctx, data, env);
    }
}
