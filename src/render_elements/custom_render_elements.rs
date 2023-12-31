// smithay::backend::renderer::element::render_elements! {
//     pub CustomRenderElements<R> where
//         R: ImportAll + ImportMem;
//     Pointer=PointerRenderElement<R>,
//     Surface=WaylandSurfaceRenderElement<R>,
//     #[cfg(feature = "debug")]
//     // Note: We would like to borrow this element instead, but that would introduce
//     // a feature-dependent lifetime, which introduces a lot more feature bounds
//     // as the whole type changes and we can't have an unused lifetime (for when "debug" is disabled)
//     // in the declaration.
//     Fps=FpsElement<<R as Renderer>::TextureId>,
//     Skia=SkiaElement,
// }
// Recursive expansion of render_elements! macro
// ==============================================

use core::panicking;

use smithay::backend::renderer::{element::surface::WaylandSurfaceRenderElement, ImportAll, ImportMem, Renderer};

use crate::{drawing::{PointerRenderElement, FpsElement}, skia_drawing::SkiaElement};

pub enum CustomRenderElements<'frame, R>
where
    R: smithay::backend::renderer::Renderer + 'frame,
{
    Pointer(PointerRenderElement<R>),
    Surface(WaylandSurfaceRenderElement<R>),
    #[cfg(feature = "debug")]
    Fps(FpsElement<<R as Renderer>::TextureId>),
    Skia(SkiaElement),
    #[doc(hidden)]
    _GenericCatcher((std::marker::PhantomData<R>, std::convert::Infallible, &'frame ())),
}

impl<'frame, R> smithay::backend::renderer::element::Element for CustomRenderElements<'frame, R>
where
    R: smithay::backend::renderer::Renderer,
    <R as smithay::backend::renderer::Renderer>::TextureId: 'static,
    R: ImportAll + ImportMem,
{
    fn id(&self) -> &smithay::backend::renderer::element::Id {
        match self {
            #[allow(unused_doc_comments)]
            Self::Pointer(x) => smithay::backend::renderer::element::Element::id(x),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::id(x),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::id(x),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::id(x),
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
            Self::Pointer(x) => smithay::backend::renderer::element::Element::location(x, scale),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::location(x, scale),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::location(x, scale),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::location(x, scale),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }

        }
    }
    fn src(&self) -> smithay::utils::Rectangle<f64, smithay::utils::Buffer> {
        match self {
            #[allow(unused_doc_comments)]
            Self::Pointer(x) => smithay::backend::renderer::element::Element::src(x),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::src(x),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::src(x),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::src(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn transform(&self) -> smithay::utils::Transform {
        match self {
            #[allow(unused_doc_comments)]
            Self::Pointer(x) => smithay::backend::renderer::element::Element::transform(x),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::transform(x),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::transform(x),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::transform(x),
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
            Self::Pointer(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::geometry(x, scale),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn current_commit(&self) -> smithay::backend::renderer::utils::CommitCounter {
        match self {
            #[allow(unused_doc_comments)]
            Self::Pointer(x) => smithay::backend::renderer::element::Element::current_commit(x),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::current_commit(x),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::current_commit(x),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::current_commit(x),
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
            Self::Pointer(x) => {
                smithay::backend::renderer::element::Element::damage_since(x, scale, commit)
            }
            #[allow(unused_doc_comments)]
            Self::Surface(x) => {
                smithay::backend::renderer::element::Element::damage_since(x, scale, commit)
            }
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => {
                smithay::backend::renderer::element::Element::damage_since(x, scale, commit)
            }
            #[allow(unused_doc_comments)]
            Self::Skia(x) => {
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
            Self::Pointer(x) => {
                smithay::backend::renderer::element::Element::opaque_regions(x, scale)
            }
            #[allow(unused_doc_comments)]
            Self::Surface(x) => {
                smithay::backend::renderer::element::Element::opaque_regions(x, scale)
            }
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::opaque_regions(x, scale),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::opaque_regions(x, scale),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn alpha(&self) -> f32 {
        match self {
            #[allow(unused_doc_comments)]
            Self::Pointer(x) => smithay::backend::renderer::element::Element::alpha(x),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::alpha(x),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::alpha(x),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::alpha(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
    fn kind(&self) -> smithay::backend::renderer::element::Kind {
        match self {
            #[allow(unused_doc_comments)]
            Self::Pointer(x) => smithay::backend::renderer::element::Element::kind(x),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::Element::kind(x),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::Element::kind(x),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::Element::kind(x),
            Self::_GenericCatcher(_) => {
                panicking::panic("internal error: entered unreachable code")
            }
        }
    }
}

impl<'frame, R> smithay::backend::renderer::element::RenderElement<R> for CustomRenderElements<'frame, R>
where
    R: smithay::backend::renderer::Renderer,
    R: ImportAll + ImportMem,
    <R as smithay::backend::renderer::Renderer>::TextureId: 'static,
    SkiaElement: smithay::backend::renderer::element::RenderElement<R>,
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
            Self::Pointer(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
                x, frame, src, dst, damage,
            ),
            #[allow(unused_doc_comments)]
            Self::Surface(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
                x, frame, src, dst, damage,
            ),
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
                x, frame, src, dst, damage,
            ),
            #[allow(unused_doc_comments)]
            Self::Skia(x) => smithay::backend::renderer::element::RenderElement::<R>::draw(
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
            Self::Pointer(x) => {
                smithay::backend::renderer::element::RenderElement::<R>::underlying_storage(
                    x, renderer,
                )
            }
            #[allow(unused_doc_comments)]
            Self::Surface(x) => {
                smithay::backend::renderer::element::RenderElement::<R>::underlying_storage(
                    x, renderer,
                )
            }
            #[allow(unused_doc_comments)]
            #[cfg(feature = "debug")]
            Self::Fps(x) => {
                smithay::backend::renderer::element::RenderElement::<R>::underlying_storage(
                    x, renderer,
                )
            }
            #[allow(unused_doc_comments)]
            Self::Skia(x) => {
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
impl<'frame, R> From<PointerRenderElement<R>> for CustomRenderElements<'frame, R>
where
    R: smithay::backend::renderer::Renderer,
    // <R as smithay::backend::renderer::Renderer>::Frame<'frame>: std::convert::AsMut<SkiaFrame>,
    // R: AsSkiaFrame<'frame>,
{
    fn from(field: PointerRenderElement<R>) -> CustomRenderElements<'frame, R> {
        CustomRenderElements::Pointer(field)
    }
}
impl<'frame, R> From<WaylandSurfaceRenderElement<R>> for CustomRenderElements<'frame, R>
where
    R: smithay::backend::renderer::Renderer,
    // <R as smithay::backend::renderer::Renderer>::Frame<'frame>: std::convert::AsMut<SkiaFrame>,
    // R: AsSkiaFrame<'frame>,
{
    fn from(field: WaylandSurfaceRenderElement<R>) -> CustomRenderElements<'frame, R> {
        CustomRenderElements::Surface(field)
    }
}
#[cfg(feature = "debug")]
impl<'frame, R> From<FpsElement<<R as Renderer>::TextureId>> for CustomRenderElements<'frame, R>
where
    R: smithay::backend::renderer::Renderer,
{
    fn from(field: FpsElement<<R as Renderer>::TextureId>) -> CustomRenderElements<'frame, R> {
        CustomRenderElements::Fps(field)
    }
}
impl<'frame, R> From<SkiaElement> for CustomRenderElements<'frame, R>
where
    R: smithay::backend::renderer::Renderer,
{
    fn from(field: SkiaElement) -> CustomRenderElements<'frame, R> {
        CustomRenderElements::Skia(field)
    }
}