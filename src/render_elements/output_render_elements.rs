
// smithay::backend::renderer::element::render_elements! {
//     pub OutputRenderElements<R, E> where 
//     R: ImportAll + ImportMem;
//     Space=SpaceRenderElements<R, E>,
//     Window=Wrap<E>,
//     Custom=CustomRenderElements<R>,
//     Preview=CropRenderElement<RelocateRenderElement<RescaleRenderElement<WindowRenderElement<R>>>>,
// }

// Recursive expansion of render_elements! macro
// ==============================================

use core::panicking;

use smithay::{desktop::space::SpaceRenderElements, backend::renderer::{element::{utils::{CropRenderElement, RelocateRenderElement, RescaleRenderElement}, Wrap, RenderElement}, ImportAll, ImportMem}};

use crate::{shell::WindowRenderElement, skia_drawing::SkiaElement};

use super::custom_render_elements::CustomRenderElements;

pub enum OutputRenderElements<'frame, R, E>
where
    R: smithay::backend::renderer::Renderer,
    E: smithay::backend::renderer::element::RenderElement<R>,
{
    Space(SpaceRenderElements<R, E>),
    Window(Wrap<E>),
    Custom(CustomRenderElements<'frame, R>),
    Preview(CropRenderElement<RelocateRenderElement<RescaleRenderElement<WindowRenderElement<R>>>>),
    #[doc(hidden)]
    _GenericCatcher((std::marker::PhantomData<R>, std::convert::Infallible, &'frame ())),
}
impl<'frame, R, E> smithay::backend::renderer::element::Element for OutputRenderElements<'frame, R, E>
where
    R: smithay::backend::renderer::Renderer,
    <R as smithay::backend::renderer::Renderer>::TextureId: 'static,
    E: smithay::backend::renderer::element::RenderElement<R>
        + smithay::backend::renderer::element::Element,
    R: ImportAll + ImportMem,
{
    fn id(&self) -> &smithay::backend::renderer::element::Id {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::id(x),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::id(x),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::id(x),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::id(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn location(
        &self,
        scale: smithay::utils::Scale<f64>,
    ) -> smithay::utils::Point<i32, smithay::utils::Physical> {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::location(x, scale),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::location(x, scale),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::location(x, scale),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::location(x, scale),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn src(&self) -> smithay::utils::Rectangle<f64, smithay::utils::Buffer> {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::src(x),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::src(x),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::src(x),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::src(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn transform(&self) -> smithay::utils::Transform {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::transform(x),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::transform(x),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::transform(x),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::transform(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn geometry(
        &self,
        scale: smithay::utils::Scale<f64>,
    ) -> smithay::utils::Rectangle<i32, smithay::utils::Physical> {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn current_commit(&self) -> smithay::backend::renderer::utils::CommitCounter {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::current_commit(x),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::current_commit(x),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::current_commit(x),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::current_commit(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn damage_since(
        &self,
        scale: smithay::utils::Scale<f64>,
        commit: Option<smithay::backend::renderer::utils::CommitCounter>,
    ) -> Vec<smithay::utils::Rectangle<i32, smithay::utils::Physical>> {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => {
                smithay::backend::renderer::element::Element::damage_since(x, scale, commit)
            }
            #[allow(unused_doc_comments)]
            Self::Window(x) => {
                smithay::backend::renderer::element::Element::damage_since(x, scale, commit)
            }
            #[allow(unused_doc_comments)]
            Self::Custom(x) => {
                smithay::backend::renderer::element::Element::damage_since(x, scale, commit)
            }
            #[allow(unused_doc_comments)]
            Self::Preview(x) => {
                smithay::backend::renderer::element::Element::damage_since(x, scale, commit)
            }
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn opaque_regions(
        &self,
        scale: smithay::utils::Scale<f64>,
    ) -> Vec<smithay::utils::Rectangle<i32, smithay::utils::Physical>> {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::opaque_regions(x, scale),
            #[allow(unused_doc_comments)]
            Self::Window(x) => {
                smithay::backend::renderer::element::Element::opaque_regions(x, scale)
            }
            #[allow(unused_doc_comments)]
            Self::Custom(x) => {
                smithay::backend::renderer::element::Element::opaque_regions(x, scale)
            }
            #[allow(unused_doc_comments)]
            Self::Preview(x) => {
                smithay::backend::renderer::element::Element::opaque_regions(x, scale)
            }
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn alpha(&self) -> f32 {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::alpha(x),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::alpha(x),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::alpha(x),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::alpha(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn kind(&self) -> smithay::backend::renderer::element::Kind {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::Element::kind(x),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::Element::kind(x),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::Element::kind(x),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::Element::kind(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
}
impl<'frame, R, E> smithay::backend::renderer::element::RenderElement<R> for OutputRenderElements<'frame, R, E>
where
    R: ImportAll + ImportMem,
    R: smithay::backend::renderer::Renderer + 'frame,
    <R as smithay::backend::renderer::Renderer>::TextureId: 'static,
    SkiaElement: smithay::backend::renderer::element::RenderElement<R>,
    E: RenderElement<R>,
{
    fn draw(
        &self,
        frame: &mut <R as smithay::backend::renderer::Renderer>::Frame<'_>,
        src: smithay::utils::Rectangle<f64, smithay::utils::Buffer>,
        dst: smithay::utils::Rectangle<i32, smithay::utils::Physical>,
        damage: &[smithay::utils::Rectangle<i32, smithay::utils::Physical>],
    ) -> Result<(), <R as smithay::backend::renderer::Renderer>::Error> where {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
                x, frame, src, dst, damage,
            ),
            #[allow(unused_doc_comments)]
            Self::Window(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
                x, frame, src, dst, damage,
            ),
            #[allow(unused_doc_comments)]
            Self::Custom(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
                x, frame, src, dst, damage,
            ),
            #[allow(unused_doc_comments)]
            Self::Preview(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
                x, frame, src, dst, damage,
            ),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn underlying_storage(
        &self,
        renderer: &mut R,
    ) -> Option<smithay::backend::renderer::element::UnderlyingStorage> {
        match self {
            #[allow(unused_doc_comments)]
            Self::Space(x) => {
                smithay::backend::renderer::element::RenderElement::<R>::underlying_storage(
                    x, renderer,
                )
            }
            #[allow(unused_doc_comments)]
            Self::Window(x) => {
                smithay::backend::renderer::element::RenderElement::<R>::underlying_storage(
                    x, renderer,
                )
            }
            #[allow(unused_doc_comments)]
            Self::Custom(x) => {
                smithay::backend::renderer::element::RenderElement::<R>::underlying_storage(
                    x, renderer,
                )
            }
            #[allow(unused_doc_comments)]
            Self::Preview(x) => {
                smithay::backend::renderer::element::RenderElement::<R>::underlying_storage(
                    x, renderer,
                )
            }
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
}
impl<'frame, R, E> From<SpaceRenderElements<R, E>> for OutputRenderElements<'frame, R, E>
where
    R: smithay::backend::renderer::Renderer,
    E: smithay::backend::renderer::element::RenderElement<R>
        + smithay::backend::renderer::element::Element,
{
    fn from(field: SpaceRenderElements<R, E>) -> OutputRenderElements<'frame, R, E> {
        OutputRenderElements::Space(field)
    }
}
impl<'frame, R, E> From<Wrap<E>> for OutputRenderElements<'frame, R, E>
where
    R: smithay::backend::renderer::Renderer,
    E: smithay::backend::renderer::element::RenderElement<R>
        + smithay::backend::renderer::element::Element,
{
    fn from(field: Wrap<E>) -> OutputRenderElements<'frame, R, E> {
        OutputRenderElements::Window(field)
    }
}
impl<'frame, R, E> From<CustomRenderElements<'frame, R>> for OutputRenderElements<'frame, R, E>
where
    R: smithay::backend::renderer::Renderer,
    E: smithay::backend::renderer::element::RenderElement<R>
        + smithay::backend::renderer::element::Element,
{
    fn from(field: CustomRenderElements<'frame, R>) -> OutputRenderElements<'frame, R, E> {
        OutputRenderElements::Custom(field)
    }
}
impl<'frame, R, E>
    From<CropRenderElement<RelocateRenderElement<RescaleRenderElement<WindowRenderElement<R>>>>>
    for OutputRenderElements<'frame, R, E>
where
    R: smithay::backend::renderer::Renderer,
    E: smithay::backend::renderer::element::RenderElement<R>
        + smithay::backend::renderer::element::Element,
{
    fn from(
        field: CropRenderElement<
            RelocateRenderElement<RescaleRenderElement<WindowRenderElement<R>>>,
        >,
    ) -> OutputRenderElements<'frame, R, E> {
        OutputRenderElements::Preview(field)
    }
}