use std::marker::PhantomData;
use crate::material::{Ceramic, MaterialTag};

/// Marker trait for [`crate::registry::kitchenware::KitchenwareRegistry`].
pub trait KitchenwareTag {
}

pub trait Kitchenware : KitchenwareTag {
    type MadeOf: MaterialTag;
}

/// 熱源
pub trait HeatSource {}

/// まな板
pub struct CuttingBoard<M> {
    __variance: PhantomData<fn(M) -> M>,
}

impl<M> Default for CuttingBoard<M> {
    fn default() -> Self {
        Self {
            __variance: PhantomData,
        }
    }
}

impl<M: MaterialTag> KitchenwareTag for CuttingBoard<M> {
}

impl<M: MaterialTag> Kitchenware for CuttingBoard<M> {
    type MadeOf = M;
}

#[derive(Default)]
/// コップ
pub struct Cup;

impl KitchenwareTag for Cup {
}

impl Kitchenware for Cup {
    type MadeOf = Ceramic;
}

/// キッチンナイフ
pub struct KitchenKnife;

impl KitchenwareTag for KitchenKnife {}

/// フライパン
pub struct FlyingPan;

impl KitchenwareTag for FlyingPan {}

/// 圧力鍋
pub struct PressureCooker;

impl KitchenwareTag for PressureCooker {}

/// 蒸し器
pub struct Steamer;

impl KitchenwareTag for Steamer {}

/// おたま
pub struct Ladle;

impl KitchenwareTag for Ladle {}

/// フライ返し
pub struct Spatula;

impl KitchenwareTag for Spatula {}

/// トング
pub struct Tongs;

impl KitchenwareTag for Tongs {}

/// ざる
pub struct Colander;

impl KitchenwareTag for Colander {}

/// ピーラー
pub struct Peeler;

impl KitchenwareTag for Peeler {}

/// 泡立て器
pub struct Whisk;

impl KitchenwareTag for Whisk {}

/// おろし金
pub struct Grater;

impl KitchenwareTag for Grater {}

/// スライサー
pub struct Slicer;

impl KitchenwareTag for Slicer {}

/// 箸
pub struct Chopsticks;

impl KitchenwareTag for Chopsticks {}

/// 計量スプーン
pub struct MeasuringSpoon;

impl KitchenwareTag for MeasuringSpoon {}

/// 計量カップ
pub struct MeasuringCup;

impl KitchenwareTag for MeasuringCup {}

/// 麺棒
pub struct RollingPin;

impl KitchenwareTag for RollingPin {}

/// トレイ
pub struct Tray;

impl KitchenwareTag for Tray {}

/// ボウル
pub struct MixingBowl;

impl KitchenwareTag for MixingBowl {}

/// キッチンコンロ
pub struct KitchenStove;

impl KitchenwareTag for KitchenStove {}

impl HeatSource for KitchenStove {}

/// 携帯熱源
pub struct PortableStove;

impl KitchenwareTag for PortableStove {}

impl HeatSource for PortableStove {}

/// キャンプファイヤー
pub struct CampFire;

impl KitchenwareTag for CampFire {}

impl HeatSource for CampFire {}

/// 缶切り
pub struct CanOpener;

impl KitchenwareTag for CanOpener {}
