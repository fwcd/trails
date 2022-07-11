use druid::{Widget, Size, Env, BoxConstraints, LifeCycle, Event, PaintCtx, LayoutCtx, UpdateCtx, LifeCycleCtx, EventCtx, RenderContext, Rect, Color, Point, piet::{Text, TextLayoutBuilder, TextLayout}, FontFamily, FontWeight, Data};

pub struct Tighten<W> {
    child: W
}

impl<W> Tighten<W> {
    pub fn new(child: W) -> Self {
        Self { child }
    }
}

impl<W, D> Widget<D> for Tighten<W> where D: Data, W: Widget<D> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &D, data: &D, env: &Env) {
        self.child.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &D, env: &Env) -> Size {
        let child_bc = BoxConstraints::tight(bc.max());
        self.child.layout(ctx, &child_bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        self.child.paint(ctx, data, env);
    }
}
