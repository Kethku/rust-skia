use crate::prelude::*;
use crate::{scalar, Point, Point3, RSXform, Rect, Scalar, Size, Vector};
use once_cell::sync::OnceCell;
use skia_bindings::{C_SkMatrix_SubscriptMut, SkMatrix, SkMatrix_ScaleToFit};
use std::mem;
use std::ops::{Index, IndexMut};

bitflags! {
    pub struct TypeMask: u32 {
        const IDENTITY = skia_bindings::SkMatrix_TypeMask_kIdentity_Mask as u32;
        const TRANSLATE = skia_bindings::SkMatrix_TypeMask_kTranslate_Mask as u32;
        const SCALE = skia_bindings::SkMatrix_TypeMask_kScale_Mask as u32;
        const AFFINE = skia_bindings::SkMatrix_TypeMask_kAffine_Mask as u32;
        const PERSPECTIVE = skia_bindings::SkMatrix_TypeMask_kPerspective_Mask as u32;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ScaleToFit {
    Fill = SkMatrix_ScaleToFit::kFill_ScaleToFit as _,
    Start = SkMatrix_ScaleToFit::kStart_ScaleToFit as _,
    Center = SkMatrix_ScaleToFit::kCenter_ScaleToFit as _,
    End = SkMatrix_ScaleToFit::kEnd_ScaleToFit as _,
}

impl NativeTransmutable<SkMatrix_ScaleToFit> for ScaleToFit {}
#[test]
fn test_matrix_scale_to_fit_layout() {
    ScaleToFit::test_layout()
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Matrix(SkMatrix);

impl NativeTransmutable<SkMatrix> for Matrix {}
#[test]
fn test_matrix_layout() {
    Matrix::test_layout()
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { skia_bindings::C_SkMatrix_Equals(self.native(), rhs.native()) }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Member {
    ScaleX = 0,
    SkewX = 1,
    TransX = 2,
    SkewY = 3,
    ScaleY = 4,
    TransY = 5,
    Persp0 = 6,
    Persp1 = 7,
    Persp2 = 8,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AffineMember {
    ScaleX = 0,
    SkewY = 1,
    SkewX = 2,
    ScaleY = 3,
    TransX = 4,
    TransY = 5,
}

impl Index<Member> for Matrix {
    type Output = scalar;

    fn index(&self, index: Member) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<AffineMember> for Matrix {
    type Output = scalar;

    fn index(&self, index: AffineMember) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<usize> for Matrix {
    type Output = scalar;

    fn index(&self, index: usize) -> &Self::Output {
        &self.native().fMat[index]
    }
}

impl IndexMut<Member> for Matrix {
    fn index_mut(&mut self, index: Member) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<AffineMember> for Matrix {
    fn index_mut(&mut self, index: AffineMember) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *C_SkMatrix_SubscriptMut(self.native_mut(), index) }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::new_identity()
    }
}

impl Matrix {
    pub fn new_scale((sx, sy): (scalar, scalar)) -> Matrix {
        Matrix::from_native(unsafe { SkMatrix::MakeScale(sx, sy) })
    }

    pub fn new_trans(d: impl Into<Vector>) -> Matrix {
        let d = d.into();
        Matrix::from_native(unsafe { SkMatrix::MakeTrans(d.x, d.y) })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_all(
        scale_x: scalar,
        skew_x: scalar,
        trans_x: scalar,
        skew_y: scalar,
        scale_y: scalar,
        trans_y: scalar,
        pers_0: scalar,
        pers_1: scalar,
        pers_2: scalar,
    ) -> Matrix {
        Matrix::from_native(unsafe {
            SkMatrix::MakeAll(
                scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, pers_0, pers_1, pers_2,
            )
        })
    }

    pub fn get_type(&self) -> TypeMask {
        TypeMask::from_bits_truncate(unsafe { self.native().getType() } as _)
    }

    pub fn is_identity(&self) -> bool {
        unsafe { self.native().isIdentity() }
    }

    pub fn is_scale_translate(&self) -> bool {
        unsafe { self.native().isScaleTranslate() }
    }

    pub fn is_translate(&self) -> bool {
        // isTranslate does not link
        (self.get_type() & !TypeMask::TRANSLATE).is_empty()
    }

    pub fn rect_stays_rect(&self) -> bool {
        unsafe { self.native().rectStaysRect() }
    }

    pub fn preserves_axis_alignment(&self) -> bool {
        unsafe { self.native().preservesAxisAlignment() }
    }

    pub fn has_perspective(&self) -> bool {
        unsafe { self.native().hasPerspective() }
    }

    pub fn is_similarity(&self) -> bool {
        unsafe { self.native().isSimilarity(scalar::NEARLY_ZERO) }
    }

    pub fn preserves_right_angles(&self) -> bool {
        unsafe { self.native().preservesRightAngles(scalar::NEARLY_ZERO) }
    }

    #[deprecated(since = "0.12.0", note = "use scale_x() ")]
    pub fn get_scale_x(&self) -> scalar {
        self.scale_x()
    }

    pub fn scale_x(&self) -> scalar {
        unsafe { self.native().getScaleX() }
    }

    #[deprecated(since = "0.12.0", note = "use scale_y()")]
    pub fn get_scale_y(&self) -> scalar {
        self.scale_y()
    }

    pub fn scale_y(&self) -> scalar {
        unsafe { self.native().getScaleY() }
    }

    #[deprecated(since = "0.12.0", note = "use skew_y()")]
    pub fn get_skew_y(&self) -> scalar {
        self.skew_y()
    }

    pub fn skew_y(&self) -> scalar {
        unsafe { self.native().getSkewY() }
    }

    #[deprecated(since = "0.12.0", note = "use skew_x()")]
    pub fn get_skew_x(&self) -> scalar {
        self.skew_x()
    }

    pub fn skew_x(&self) -> scalar {
        unsafe { self.native().getSkewX() }
    }

    #[deprecated(since = "0.12.0", note = "use translate_x()")]
    pub fn get_translate_x(&self) -> scalar {
        self.translate_x()
    }

    pub fn translate_x(&self) -> scalar {
        unsafe { self.native().getTranslateX() }
    }

    #[deprecated(since = "0.12.0", note = "use translate_y()")]
    pub fn get_translate_y(&self) -> scalar {
        self.translate_y()
    }

    pub fn translate_y(&self) -> scalar {
        unsafe { self.native().getTranslateY() }
    }

    #[deprecated(since = "0.12.0", note = "use persp_x()")]
    pub fn get_persp_x(&self) -> scalar {
        self.persp_x()
    }

    pub fn persp_x(&self) -> scalar {
        unsafe { self.native().getPerspX() }
    }

    #[deprecated(since = "0.12.0", note = "use persp_y()")]
    pub fn get_persp_y(&self) -> scalar {
        self.persp_y()
    }

    pub fn persp_y(&self) -> scalar {
        unsafe { self.native().getPerspY() }
    }

    pub fn set_scale_x(&mut self, v: scalar) -> &mut Self {
        self[Member::ScaleX] = v;
        self
    }

    pub fn set_scale_y(&mut self, v: scalar) -> &mut Self {
        self[Member::ScaleY] = v;
        self
    }

    pub fn set_skew_y(&mut self, v: scalar) -> &mut Self {
        self[Member::SkewY] = v;
        self
    }

    pub fn set_skew_x(&mut self, v: scalar) -> &mut Self {
        self[Member::SkewX] = v;
        self
    }

    pub fn set_translate_x(&mut self, v: scalar) -> &mut Self {
        self[Member::TransX] = v;
        self
    }

    pub fn set_translate_y(&mut self, v: scalar) -> &mut Self {
        self[Member::TransY] = v;
        self
    }

    pub fn set_persp_x(&mut self, v: scalar) -> &mut Self {
        self[Member::Persp0] = v;
        self
    }

    pub fn set_persp_y(&mut self, v: scalar) -> &mut Self {
        self[Member::Persp1] = v;
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_all(
        &mut self,
        scale_x: scalar,
        skew_x: scalar,
        trans_x: scalar,
        skew_y: scalar,
        scale_y: scalar,
        trans_y: scalar,
        persp_0: scalar,
        persp_1: scalar,
        persp_2: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut().setAll(
                scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, persp_0, persp_1, persp_2,
            )
        };
        self
    }

    pub fn get_9(&self, buffer: &mut [scalar; 9]) {
        unsafe { self.native().get9(buffer.as_mut_ptr()) }
    }

    pub fn set_9(&mut self, buffer: &[scalar; 9]) -> &mut Self {
        unsafe { self.native_mut().set9(buffer.as_ptr()) }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    pub fn set_identity(&mut self) -> &mut Self {
        unsafe { self.native_mut().setIdentity() }
        self
    }

    pub fn set_translate(&mut self, v: impl Into<Vector>) -> &mut Self {
        let v = v.into();
        unsafe { self.native_mut().setTranslate(v.x, v.y) }
        self
    }

    pub fn set_scale(
        &mut self,
        (sx, sy): (scalar, scalar),
        pivot: impl Into<Option<Point>>,
    ) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().setScale(sx, sy, pivot.x, pivot.y) }
        self
    }

    pub fn set_rotate(&mut self, degrees: scalar, pivot: impl Into<Option<Point>>) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().setRotate(degrees, pivot.x, pivot.y) }
        self
    }

    pub fn set_sin_cos(
        &mut self,
        (sin_value, cos_value): (scalar, scalar),
        pivot: impl Into<Option<Point>>,
    ) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe {
            self.native_mut()
                .setSinCos(sin_value, cos_value, pivot.x, pivot.y)
        }
        self
    }

    pub fn set_rsxform(&mut self, rsx_form: &RSXform) -> &mut Self {
        unsafe { self.native_mut().setRSXform(rsx_form.native()) };
        self
    }

    pub fn set_skew(
        &mut self,
        (kx, ky): (scalar, scalar),
        pivot: impl Into<Option<Point>>,
    ) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().setSkew(kx, ky, pivot.x, pivot.y) }
        self
    }

    pub fn set_concat(&mut self, a: &Matrix, b: &Matrix) -> &mut Self {
        unsafe { self.native_mut().setConcat(a.native(), b.native()) }
        self
    }

    pub fn pre_translate(&mut self, delta: impl Into<Vector>) -> &mut Self {
        let delta = delta.into();
        unsafe { self.native_mut().preTranslate(delta.x, delta.y) }
        self
    }

    pub fn pre_scale(
        &mut self,
        (sx, sy): (scalar, scalar),
        pivot: impl Into<Option<Point>>,
    ) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().preScale(sx, sy, pivot.x, pivot.y) }
        self
    }

    pub fn pre_rotate(&mut self, degrees: scalar, pivot: impl Into<Option<Point>>) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().preRotate(degrees, pivot.x, pivot.y) }
        self
    }

    pub fn pre_skew(
        &mut self,
        (kx, ky): (scalar, scalar),
        pivot: impl Into<Option<Point>>,
    ) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().preSkew(kx, ky, pivot.x, pivot.y) }
        self
    }

    pub fn pre_concat(&mut self, other: &Matrix) -> &mut Self {
        unsafe { self.native_mut().preConcat(other.native()) }
        self
    }

    pub fn post_translate(&mut self, delta: impl Into<Vector>) -> &mut Self {
        let delta = delta.into();
        unsafe { self.native_mut().postTranslate(delta.x, delta.y) }
        self
    }

    pub fn post_scale(
        &mut self,
        (sx, sy): (scalar, scalar),
        pivot: impl Into<Option<Point>>,
    ) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().postScale(sx, sy, pivot.x, pivot.y) }
        self
    }

    pub fn post_idiv(&mut self, (div_x, div_y): (i32, i32)) -> bool {
        unsafe { self.native_mut().postIDiv(div_x, div_y) }
    }

    pub fn post_rotate(&mut self, degrees: scalar, pivot: impl Into<Option<Point>>) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().postRotate(degrees, pivot.x, pivot.y) }
        self
    }

    pub fn post_skew(
        &mut self,
        (kx, ky): (scalar, scalar),
        pivot: impl Into<Option<Point>>,
    ) -> &mut Self {
        let pivot = pivot.into().unwrap_or_default();
        unsafe { self.native_mut().postSkew(kx, ky, pivot.x, pivot.y) }
        self
    }

    pub fn post_concat(&mut self, other: &Matrix) -> &mut Self {
        unsafe { self.native_mut().postConcat(other.native()) }
        self
    }

    pub fn set_rect_to_rect(
        &mut self,
        src: impl AsRef<Rect>,
        dst: impl AsRef<Rect>,
        stf: ScaleToFit,
    ) -> bool {
        unsafe {
            self.native_mut().setRectToRect(
                src.as_ref().native(),
                dst.as_ref().native(),
                stf.into_native(),
            )
        }
    }

    pub fn from_rect_to_rect(
        src: impl AsRef<Rect>,
        dst: impl AsRef<Rect>,
        stf: ScaleToFit,
    ) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        m.set_rect_to_rect(src, dst, stf).if_true_some(m)
    }

    pub fn set_poly_to_poly(&mut self, src: &[Point], dst: &[Point]) -> bool {
        if src.len() != dst.len() {
            return false;
        }
        unsafe {
            self.native_mut().setPolyToPoly(
                src.native().as_ptr(),
                dst.native().as_ptr(),
                src.len().try_into().unwrap(),
            )
        }
    }

    pub fn from_poly_to_poly(src: &[Point], dst: &[Point]) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        m.set_poly_to_poly(src, dst).if_true_some(m)
    }

    #[must_use]
    pub fn invert(&self) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        unsafe { self.native().invert(m.native_mut()) }.if_true_some(m)
    }

    pub fn set_affine_identity(affine: &mut [scalar; 6]) {
        unsafe { SkMatrix::SetAffineIdentity(affine.as_mut_ptr()) }
    }

    #[deprecated(since = "0.12.0", note = "use to_affine()")]
    #[must_use]
    pub fn as_affine(&self) -> Option<[scalar; 6]> {
        self.to_affine()
    }

    #[must_use]
    pub fn to_affine(&self) -> Option<[scalar; 6]> {
        let mut affine = [scalar::default(); 6];
        unsafe { self.native().asAffine(affine.as_mut_ptr()) }.if_true_some(affine)
    }

    pub fn set_affine(&mut self, affine: &[scalar; 6]) -> &mut Self {
        unsafe { self.native_mut().setAffine(affine.as_ptr()) };
        self
    }

    pub fn from_affine(affine: &[scalar; 6]) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { m.native_mut().setAffine(affine.as_ptr()) }
        m
    }

    pub fn map_points(&self, dst: &mut [Point], src: &[Point]) {
        assert!(dst.len() >= src.len());

        unsafe {
            self.native().mapPoints(
                dst.native_mut().as_mut_ptr(),
                src.native().as_ptr(),
                src.len().try_into().unwrap(),
            )
        };
    }

    pub fn map_points_inplace(&self, pts: &mut [Point]) {
        unsafe {
            self.native()
                .mapPoints1(pts.native_mut().as_mut_ptr(), pts.len().try_into().unwrap())
        };
    }

    pub fn map_homogeneous_points(&self, dst: &mut [Point3], src: &[Point3]) {
        assert!(dst.len() >= src.len());

        unsafe {
            self.native().mapHomogeneousPoints(
                dst.native_mut().as_mut_ptr(),
                src.native().as_ptr(),
                src.len().try_into().unwrap(),
            )
        };
    }

    pub fn map_xy(&self, x: scalar, y: scalar) -> Point {
        self.map_point((x, y))
    }

    pub fn map_point(&self, point: impl Into<Point>) -> Point {
        let point = point.into();
        Point::from_native(unsafe { self.native().mapXY1(point.x, point.y) })
    }

    pub fn map_vectors(&self, dst: &mut [Vector], src: &[Vector]) {
        assert!(dst.len() >= src.len());
        unsafe {
            self.native().mapVectors(
                dst.native_mut().as_mut_ptr(),
                src.native().as_ptr(),
                src.len().try_into().unwrap(),
            )
        }
    }

    pub fn map_vectors_inplace(&self, vecs: &mut [Vector]) {
        unsafe {
            self.native().mapVectors1(
                vecs.native_mut().as_mut_ptr(),
                vecs.len().try_into().unwrap(),
            )
        }
    }

    pub fn map_vector(&self, vec: impl Into<Vector>) -> Vector {
        let vec = vec.into();
        Vector::from_native(unsafe { self.native().mapVector1(vec.x, vec.y) })
    }

    pub fn map_rect(&self, rect: impl AsRef<Rect>) -> (Rect, bool) {
        let mut rect = rect.as_ref().into_native();
        let rect_stays_rect = unsafe { self.native().mapRect1(&mut rect) };
        (Rect::from_native(rect), rect_stays_rect)
    }

    pub fn map_rect_to_quad(&self, rect: impl AsRef<Rect>) -> [Point; 4] {
        let mut points = [Point::default(); 4];
        unsafe {
            self.native()
                .mapRectToQuad(points.native_mut().as_mut_ptr(), rect.as_ref().native())
        };
        points
    }

    pub fn map_rect_scale_translate(&self, src: impl AsRef<Rect>) -> Option<Rect> {
        if self.is_scale_translate() {
            let mut rect = Rect::default();
            unsafe {
                self.native()
                    .mapRectScaleTranslate(rect.native_mut(), src.as_ref().native())
            };
            Some(rect)
        } else {
            None
        }
    }

    pub fn map_radius(&self, radius: scalar) -> Option<scalar> {
        if !self.has_perspective() {
            Some(unsafe { self.native().mapRadius(radius) })
        } else {
            None
        }
    }

    pub fn is_fixed_step_in_x(&self) -> bool {
        unsafe { self.native().isFixedStepInX() }
    }

    pub fn fixed_step_in_x(&self, y: scalar) -> Option<Vector> {
        if self.is_fixed_step_in_x() {
            Some(Vector::from_native(unsafe {
                self.native().fixedStepInX(y)
            }))
        } else {
            None
        }
    }

    pub fn cheap_equal_to(&self, other: &Matrix) -> bool {
        unsafe { self.native().cheapEqualTo(other.native()) }
    }

    pub fn dump(&self) {
        unsafe { self.native().dump() }
    }

    pub fn min_scale(&self) -> scalar {
        unsafe { self.native().getMinScale() }
    }

    pub fn max_scale(&self) -> scalar {
        unsafe { self.native().getMaxScale() }
    }

    pub fn min_max_scales(&self) -> (scalar, scalar) {
        let mut r: [scalar; 2] = Default::default();
        unsafe { self.native().getMinMaxScales(r.as_mut_ptr()) };
        (r[0], r[1])
    }

    pub fn decompose_scale(&self, mut remaining: Option<&mut Matrix>) -> Option<Size> {
        let mut size = Size::default();
        unsafe {
            self.native()
                .decomposeScale(size.native_mut(), remaining.native_ptr_or_null_mut())
        }
        .if_true_some(size)
    }

    pub fn i() -> &'static Matrix {
        static IDENTITY: OnceCell<Matrix> = OnceCell::new();
        IDENTITY.get_or_init(|| Matrix::new_identity())
    }

    pub fn invalid_matrix() -> &'static Matrix {
        static INVALID: OnceCell<Matrix> = OnceCell::new();
        INVALID.get_or_init(|| Matrix::from_native(unsafe { *SkMatrix::InvalidMatrix() }))
    }

    pub fn concat(a: &Matrix, b: &Matrix) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { m.native_mut().setConcat(a.native(), b.native()) };
        m
    }

    pub fn dirty_matrix_type_cache(&mut self) {
        // does not link:
        // unsafe { self.native_mut().dirtyMatrixTypeCache() }
        self.native_mut().fTypeMask = 0x80;
    }

    pub fn set_scale_translate(
        &mut self,
        (sx, sy): (scalar, scalar),
        t: impl Into<Vector>,
    ) -> &mut Self {
        let t = t.into();
        unsafe { self.native_mut().setScaleTranslate(sx, sy, t.x, t.y) }
        self
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn new_identity() -> Matrix {
        // SkMatrix contains no C++ types, so this is safe:
        let mut m: SkMatrix = unsafe { mem::zeroed() };
        unsafe { m.reset() };
        Matrix::from_native(m)
    }
}

impl IndexGet for Matrix {}
impl IndexSet for Matrix {}

#[test]
fn test_get_set_trait_compilation() {
    let mut m = Matrix::new_identity();
    let _x = m.get(AffineMember::ScaleX);
    m.set(AffineMember::ScaleX, 1.0);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_tuple_to_vector() {
    let mut m = Matrix::new_identity();
    m.set_translate((10.0, 11.0));
    assert_eq!(10.0, m.translate_x());
    assert_eq!(11.0, m.translate_y());
}

#[test]
fn setting_a_matrix_component_recomputes_typemask() {
    let mut m = Matrix::default();
    assert_eq!(TypeMask::IDENTITY, m.get_type());
    m.set_persp_x(0.1);
    assert_eq!(
        TypeMask::TRANSLATE | TypeMask::SCALE | TypeMask::AFFINE | TypeMask::PERSPECTIVE,
        m.get_type()
    );
}

#[test]
fn static_identity_matrix() {
    let i = Matrix::i();
    let identity = Matrix::new_identity();
    assert_eq!(*i, identity)
}
