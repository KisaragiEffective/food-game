pub trait AnimalTag {
}

/// 鶏
/// ## See also
/// - <https://www.maff.go.jp/j/pr/aff/2009/pdf/aff2009_02_poster06.pdf>
pub struct Chicken;

impl AnimalTag for Chicken {}

/// せせり
pub struct ChickenNeck;

/// 手羽さき
pub struct ChickenWing;

/// 手羽なか
pub struct ChickenMiddleWing;

/// 手羽もと
pub struct ChickenWingStick;

/// 鶏胸肉
pub struct ChickenBreast;

/// ふりそで
pub struct ChickenShoulder;

/// 鶏皮
pub struct ChickenSkin;

/// 鳥のささみ
pub struct ChickenTender;

/// 鶏もも肉
pub struct ChickenThigh;

/// 鶏ハツ
pub struct ChickenHeart;

/// 鶏ハツモト
pub struct ChickenVentricle;

/// 鶏レバー
pub struct ChickenLiver;

/// 鶏ボンジリ
pub struct ChichenTail;

/// 鶏ヤゲン
pub struct ChickenBreastCartilage;

/// 鶏砂肝
pub struct ChichenGizzard;

/// 鶏の軟骨
pub struct ChickenCartliage;

/// 豚
/// ## See also
/// - <https://www.maff.go.jp/j/pr/aff/2009/pdf/aff2009_02_poster04.pdf>
pub struct Pig;

/// トントロ
pub struct PigNeck;

/// 豚の肩ロース
pub struct PigButt;

/// 豚ロース
pub struct PorkLoin;

/// 豚の肩
pub struct PigPicnic;

/// 豚ヒレ肉
pub struct PigTenderloin;

/// 豚のそともも
pub struct PigSilverside;

/// 豚バラ肉
pub struct PigBelly;

/// 豚もも肉
pub struct PigTopside;

/// チークミート
pub struct PigCheekMeat;

/// 豚の腎臓
pub struct PigKidney;

/// 豚ガツ
pub struct PigStomach;

/// 豚タン
pub struct PigTongue;

/// 豚ハツ
pub struct PigHeart;

/// 豚レバー
pub struct PigLiver;

/// 小腸
pub struct PigSmallIntestine;
/// 大腸
pub struct PigLargeIntestine;

pub type 豚モツ = (PigSmallIntestine, PigLargeIntestine);

/// コブクロ
pub struct PigUteri;

/// トンソク
pub struct PigTrotters;

impl AnimalTag for Pig {}

/// ## See also
/// - <https://www.maff.go.jp/j/pr/aff/2009/pdf/aff2009_02_poster02.pdf>
pub struct Cow;

impl AnimalTag for Cow {}

/// 牛ネック
pub struct CowNeck;

/// 牛ランプ
pub struct CowRump;

/// 牛ラムシン
pub struct CowRumpRoast;

/// イチボ
pub struct CowHbone;

/// 牛ヒレ肉
pub struct CowTenderloin;

pub struct CowBottomRound;

pub struct CowTopRound;

pub struct CowChuckFlapTail;

pub struct CowTriTip;

pub struct CowShoulderClod;

pub struct CowChunkTender;

pub struct CowBrisket;

/// 牛カルビ
pub struct CowShortRib;

pub struct CowInsideSkirt;

pub struct CowFlankenRib;

pub struct CowBottomFlap;

pub struct CowChunkRib;

pub struct CowRibFinger;

pub struct CowShank;

/// 牛タン
pub struct CowTongue;

/// ツラミ
pub struct CowCheek;

/// 牛レバー
pub struct CowLiver;

/// 牛サガリ
pub struct CowHangingTender;

pub struct CowKidney;

/// ミノ
pub struct CowMountainChainTripe;

/// ミノサンド
pub struct CowMountainChainTripe2;

pub struct CowHeart;

/// 牛ハラミ
pub struct CowOutsideSkirt;

pub struct CowHoneycombTripe;

pub struct CowBookTripe;

pub struct CowAbmasum;

pub struct CowSmallIntestine;

pub struct CowLargeIntestine;
