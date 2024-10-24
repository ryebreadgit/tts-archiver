use serde::{Deserialize, Serialize};
use serde_json::Value;

// Automatically generated TTS save schema

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TTSSave {
    #[serde(rename = "SaveName")]
    pub save_name: Option<String>,
    #[serde(rename = "EpochTime")]
    pub epoch_time: Option<i64>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<String>,
    #[serde(rename = "GameMode")]
    pub game_mode: Option<String>,
    #[serde(rename = "GameType")]
    pub game_type: Option<String>,
    #[serde(rename = "GameComplexity")]
    pub game_complexity: Option<String>,
    #[serde(rename = "PlayingTime")]
    pub playing_time: Option<Vec<i64>>,
    #[serde(rename = "PlayerCounts")]
    pub player_counts: Option<Vec<i64>>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "Gravity")]
    pub gravity: Option<f64>,
    #[serde(rename = "PlayArea")]
    pub play_area: Option<f64>,
    #[serde(rename = "Table")]
    pub table: Option<String>,
    #[serde(rename = "Sky")]
    pub sky: Option<String>,
    #[serde(rename = "Note")]
    pub note: Option<String>,
    #[serde(rename = "TabStates")]
    pub tab_states: Option<TabStates>,
    #[serde(rename = "MusicPlayer")]
    pub music_player: Option<MusicPlayer>,
    #[serde(rename = "Grid")]
    pub grid: Option<Grid>,
    #[serde(rename = "Lighting")]
    pub lighting: Option<Lighting>,
    #[serde(rename = "Hands")]
    pub hands: Option<Hands>,
    #[serde(rename = "ComponentTags")]
    pub component_tags: Option<ComponentTags>,
    #[serde(rename = "Turns")]
    pub turns: Option<Turns>,
    #[serde(rename = "CameraStates")]
    pub camera_states: Option<Vec<Option<CameraState>>>,
    #[serde(rename = "DecalPallet")]
    pub decal_pallet: Option<Vec<DecalPallet>>,
    #[serde(rename = "LuaScript")]
    pub lua_script: Option<String>,
    #[serde(rename = "LuaScriptState")]
    pub lua_script_state: Option<String>,
    #[serde(rename = "XmlUI")]
    pub xml_ui: Option<String>,
    #[serde(rename = "SnapPoints")]
    pub snap_points: Option<Vec<SnapPoint>>,
    #[serde(rename = "ObjectStates")]
    pub object_states: Option<Vec<ObjectState>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabStates {
    #[serde(rename = "0")]
    pub n0: Option<n0>,
    #[serde(rename = "1")]
    pub n1: Option<n1>,
    #[serde(rename = "2")]
    pub n2: Option<n2>,
    #[serde(rename = "3")]
    pub n3: Option<n3>,
    #[serde(rename = "4")]
    pub n4: Option<n4>,
    #[serde(rename = "5")]
    pub n5: Option<n5>,
    #[serde(rename = "6")]
    pub n6: Option<n6>,
    #[serde(rename = "7")]
    pub n7: Option<n7>,
    #[serde(rename = "8")]
    pub n8: Option<n8>,
    #[serde(rename = "9")]
    pub n9: Option<n9>,
    #[serde(rename = "10")]
    pub n10: Option<n10>,
    #[serde(rename = "11")]
    pub n11: Option<n11>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n0 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor2>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor2 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor3>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor3 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n3 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor4>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor4 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor5>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor5 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n5 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor6>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor6 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor7>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor7 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor8>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor8 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor9>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor9 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor10>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor10 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor11>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor11 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n11 {
    pub title: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub visible_color: Option<VisibleColor12>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleColor12 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlayer {
    #[serde(rename = "RepeatSong")]
    pub repeat_song: Option<bool>,
    #[serde(rename = "PlaylistEntry")]
    pub playlist_entry: Option<i64>,
    #[serde(rename = "CurrentAudioTitle")]
    pub current_audio_title: Option<String>,
    #[serde(rename = "CurrentAudioURL")]
    pub current_audio_url: Option<String>,
    #[serde(rename = "AudioLibrary")]
    pub audio_library: Option<Vec<AudioLibrary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioLibrary {
    #[serde(rename = "Item1")]
    pub item1: Option<String>,
    #[serde(rename = "Item2")]
    pub item2: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
    #[serde(rename = "Lines")]
    pub lines: Option<bool>,
    #[serde(rename = "Color")]
    pub color: Option<Color>,
    #[serde(rename = "Opacity")]
    pub opacity: Option<f64>,
    #[serde(rename = "ThickLines")]
    pub thick_lines: Option<bool>,
    #[serde(rename = "Snapping")]
    pub snapping: Option<bool>,
    #[serde(rename = "Offset")]
    pub offset: Option<bool>,
    #[serde(rename = "BothSnapping")]
    pub both_snapping: Option<bool>,
    pub x_size: Option<f64>,
    pub y_size: Option<f64>,
    #[serde(rename = "PosOffset")]
    pub pos_offset: Option<PosOffset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PosOffset {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lighting {
    #[serde(rename = "LightIntensity")]
    pub light_intensity: Option<f64>,
    #[serde(rename = "LightColor")]
    pub light_color: Option<LightColor>,
    #[serde(rename = "AmbientIntensity")]
    pub ambient_intensity: Option<f64>,
    #[serde(rename = "AmbientType")]
    pub ambient_type: Option<i64>,
    #[serde(rename = "AmbientSkyColor")]
    pub ambient_sky_color: Option<AmbientSkyColor>,
    #[serde(rename = "AmbientEquatorColor")]
    pub ambient_equator_color: Option<AmbientEquatorColor>,
    #[serde(rename = "AmbientGroundColor")]
    pub ambient_ground_color: Option<AmbientGroundColor>,
    #[serde(rename = "ReflectionIntensity")]
    pub reflection_intensity: Option<f64>,
    #[serde(rename = "LutIndex")]
    pub lut_index: Option<i64>,
    #[serde(rename = "LutContribution")]
    pub lut_contribution: Option<f64>,
    #[serde(rename = "LutURL")]
    pub lut_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightColor {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbientSkyColor {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbientEquatorColor {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbientGroundColor {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hands {
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,
    #[serde(rename = "DisableUnused")]
    pub disable_unused: Option<bool>,
    #[serde(rename = "Hiding")]
    pub hiding: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentTags {
    pub labels: Option<Vec<Value>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Turns {
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
    #[serde(rename = "TurnOrder")]
    pub turn_order: Option<Vec<Value>>,
    #[serde(rename = "Reverse")]
    pub reverse: Option<bool>,
    #[serde(rename = "SkipEmpty")]
    pub skip_empty: Option<bool>,
    #[serde(rename = "DisableInteractions")]
    pub disable_interactions: Option<bool>,
    #[serde(rename = "PassTurns")]
    pub pass_turns: Option<bool>,
    #[serde(rename = "TurnColor")]
    pub turn_color: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraState {
    #[serde(rename = "Position")]
    pub position: Option<Position>,
    #[serde(rename = "Rotation")]
    pub rotation: Option<Rotation>,
    #[serde(rename = "Distance")]
    pub distance: Option<f64>,
    #[serde(rename = "Zoomed")]
    pub zoomed: Option<bool>,
    #[serde(rename = "AbsolutePosition")]
    pub absolute_position: Option<AbsolutePosition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbsolutePosition {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecalPallet {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapPoint {
    #[serde(rename = "Position")]
    pub position: Option<Position2>,
    #[serde(rename = "Rotation")]
    pub rotation: Option<Rotation2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation2 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectState {
    #[serde(rename = "GUID")]
    pub guid: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Transform")]
    pub transform: Option<Transform>,
    #[serde(rename = "Nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "GMNotes")]
    pub gmnotes: Option<String>,
    #[serde(rename = "AltLookAngle")]
    pub alt_look_angle: Option<AltLookAngle>,
    #[serde(rename = "ColorDiffuse")]
    pub color_diffuse: Option<ColorDiffuse>,
    #[serde(rename = "LayoutGroupSortIndex")]
    pub layout_group_sort_index: Option<i64>,
    #[serde(rename = "Value")]
    pub value: Option<i64>,
    #[serde(rename = "Locked")]
    pub locked: Option<bool>,
    #[serde(rename = "Grid")]
    pub grid: Option<bool>,
    #[serde(rename = "Snap")]
    pub snap: Option<bool>,
    #[serde(rename = "IgnoreFoW")]
    pub ignore_fo_w: Option<bool>,
    #[serde(rename = "MeasureMovement")]
    pub measure_movement: Option<bool>,
    #[serde(rename = "DragSelectable")]
    pub drag_selectable: Option<bool>,
    #[serde(rename = "Autoraise")]
    pub autoraise: Option<bool>,
    #[serde(rename = "Sticky")]
    pub sticky: Option<bool>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<bool>,
    #[serde(rename = "GridProjection")]
    pub grid_projection: Option<bool>,
    #[serde(rename = "HideWhenFaceDown")]
    pub hide_when_face_down: Option<bool>,
    #[serde(rename = "Hands")]
    pub hands: Option<bool>,
    #[serde(rename = "CustomPDF")]
    pub custom_pdf: Option<CustomPdf>,
    #[serde(rename = "LuaScript")]
    pub lua_script: Option<String>,
    #[serde(rename = "LuaScriptState")]
    pub lua_script_state: Option<String>,
    #[serde(rename = "XmlUI")]
    pub xml_ui: Option<String>,
    #[serde(rename = "MaterialIndex")]
    pub material_index: Option<Option<i64>>,
    #[serde(rename = "MeshIndex")]
    pub mesh_index: Option<Option<i64>>,
    #[serde(rename = "Number")]
    pub number: Option<Option<i64>>,
    #[serde(rename = "CustomMesh")]
    pub custom_mesh: Option<CustomMesh>,
    #[serde(rename = "Bag")]
    pub bag: Option<Bag>,
    #[serde(rename = "PhysicsMaterial")]
    pub physics_material: Option<PhysicsMaterial>,
    #[serde(rename = "Rigidbody")]
    pub rigidbody: Option<Rigidbody>,
    #[serde(rename = "CardID")]
    pub card_id: Option<Option<i64>>,
    #[serde(rename = "SidewaysCard")]
    pub sideways_card: Option<Option<bool>>,
    #[serde(rename = "CustomDeck")]
    pub custom_deck: Option<CustomDeck>,
    #[serde(rename = "CustomImage")]
    pub custom_image: Option<CustomImage>,
    #[serde(rename = "AttachedSnapPoints")]
    #[serde(default)]
    pub attached_snap_points: Option<Vec<AttachedSnapPoint>>,
    #[serde(rename = "DeckIDs")]
    #[serde(default)]
    pub deck_ids: Option<Vec<i64>>,
    #[serde(rename = "ContainedObjects")]
    #[serde(default)]
    pub contained_objects: Option<Vec<ContainedObject>>,
    #[serde(rename = "FogColor")]
    pub fog_color: Option<Option<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
    pub pos_z: Option<f64>,
    pub rot_x: Option<f64>,
    pub rot_y: Option<f64>,
    pub rot_z: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltLookAngle {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorDiffuse {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
    pub a: Option<Option<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomPdf {
    #[serde(rename = "PDFUrl")]
    pub pdfurl: Option<String>,
    #[serde(rename = "PDFPassword")]
    pub pdfpassword: Option<String>,
    #[serde(rename = "PDFPage")]
    pub pdfpage: Option<i64>,
    #[serde(rename = "PDFPageOffset")]
    pub pdfpage_offset: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomMesh {
    #[serde(rename = "MeshURL")]
    pub mesh_url: Option<String>,
    #[serde(rename = "DiffuseURL")]
    pub diffuse_url: Option<String>,
    #[serde(rename = "NormalURL")]
    pub normal_url: Option<String>,
    #[serde(rename = "ColliderURL")]
    pub collider_url: Option<String>,
    #[serde(rename = "Convex")]
    pub convex: Option<bool>,
    #[serde(rename = "MaterialIndex")]
    pub material_index: Option<i64>,
    #[serde(rename = "TypeIndex")]
    pub type_index: Option<i64>,
    #[serde(rename = "CastShadows")]
    pub cast_shadows: Option<bool>,
    #[serde(rename = "CustomShader")]
    pub custom_shader: Option<CustomShader>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomShader {
    #[serde(rename = "SpecularColor")]
    pub specular_color: SpecularColor,
    #[serde(rename = "SpecularIntensity")]
    pub specular_intensity: Option<f64>,
    #[serde(rename = "SpecularSharpness")]
    pub specular_sharpness: Option<f64>,
    #[serde(rename = "FresnelStrength")]
    pub fresnel_strength: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecularColor {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bag {
    #[serde(rename = "Order")]
    pub order: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicsMaterial {
    #[serde(rename = "StaticFriction")]
    pub static_friction: Option<f64>,
    #[serde(rename = "DynamicFriction")]
    pub dynamic_friction: Option<f64>,
    #[serde(rename = "Bounciness")]
    pub bounciness: Option<f64>,
    #[serde(rename = "FrictionCombine")]
    pub friction_combine: Option<i64>,
    #[serde(rename = "BounceCombine")]
    pub bounce_combine: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rigidbody {
    #[serde(rename = "Mass")]
    pub mass: Option<f64>,
    #[serde(rename = "Drag")]
    pub drag: Option<f64>,
    #[serde(rename = "AngularDrag")]
    pub angular_drag: Option<f64>,
    #[serde(rename = "UseGravity")]
    pub use_gravity: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDeck {
    #[serde(rename = "781")]
    pub n781: Option<n781>,
    #[serde(rename = "986")]
    pub n986: Option<n986>,
    #[serde(rename = "782")]
    pub n782: Option<n782>,
    #[serde(rename = "783")]
    pub n783: Option<n783>,
    #[serde(rename = "987")]
    pub n987: Option<n987>,
    #[serde(rename = "988")]
    pub n988: Option<n988>,
    #[serde(rename = "784")]
    pub n784: Option<n784>,
    #[serde(rename = "989")]
    pub n989: Option<n989>,
    #[serde(rename = "785")]
    pub n785: Option<n785>,
    #[serde(rename = "970")]
    pub n970: Option<n970>,
    #[serde(rename = "985")]
    pub n985: Option<n985>,
    #[serde(rename = "984")]
    pub n984: Option<n984>,
    #[serde(rename = "983")]
    pub n983: Option<n983>,
    #[serde(rename = "982")]
    pub n982: Option<n982>,
    #[serde(rename = "851")]
    pub n851: Option<n851>,
    #[serde(rename = "764")]
    pub n764: Option<n764>,
    #[serde(rename = "763")]
    pub n763: Option<n763>,
    #[serde(rename = "981")]
    pub n981: Option<n981>,
    #[serde(rename = "850")]
    pub n850: Option<n850>,
    #[serde(rename = "849")]
    pub n849: Option<n849>,
    #[serde(rename = "848")]
    pub n848: Option<n848>,
    #[serde(rename = "847")]
    pub n847: Option<n847>,
    #[serde(rename = "1886")]
    pub n1886: Option<n1886>,
    #[serde(rename = "979")]
    pub n979: Option<n979>,
    #[serde(rename = "978")]
    pub n978: Option<n978>,
    #[serde(rename = "846")]
    pub n846: Option<n846>,
    #[serde(rename = "963")]
    pub n963: Option<n963>,
    #[serde(rename = "964")]
    pub n964: Option<n964>,
    #[serde(rename = "967")]
    pub n967: Option<n967>,
    #[serde(rename = "968")]
    pub n968: Option<n968>,
    #[serde(rename = "977")]
    pub n977: Option<n977>,
    #[serde(rename = "976")]
    pub n976: Option<n976>,
    #[serde(rename = "975")]
    pub n975: Option<n975>,
    #[serde(rename = "971")]
    pub n971: Option<n971>,
    #[serde(rename = "972")]
    pub n972: Option<n972>,
    #[serde(rename = "973")]
    pub n973: Option<n973>,
    #[serde(rename = "974")]
    pub n974: Option<n974>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n781 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n986 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n782 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n783 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n987 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n988 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n784 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n989 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n785 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n970 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n985 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n984 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n983 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n982 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n851 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n764 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n763 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n981 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n850 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n849 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n848 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n847 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1886 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n979 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n978 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n846 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n963 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n964 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n967 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n968 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n977 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n976 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n975 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n971 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n972 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n973 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n974 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomImage {
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "ImageSecondaryURL")]
    pub image_secondary_url: Option<String>,
    #[serde(rename = "ImageScalar")]
    pub image_scalar: Option<f64>,
    #[serde(rename = "WidthScale")]
    pub width_scale: Option<f64>,
    #[serde(rename = "CustomTile")]
    pub custom_tile: Option<CustomTile>,
    #[serde(rename = "CustomToken")]
    pub custom_token: Option<CustomToken>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomTile {
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "Stackable")]
    pub stackable: Option<bool>,
    #[serde(rename = "Stretch")]
    pub stretch: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomToken {
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "MergeDistancePixels")]
    pub merge_distance_pixels: Option<f64>,
    #[serde(rename = "StandUp")]
    pub stand_up: Option<bool>,
    #[serde(rename = "Stackable")]
    pub stackable: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedSnapPoint {
    #[serde(rename = "Position")]
    pub position: Option<Position3>,
    #[serde(rename = "Rotation")]
    pub rotation: Option<Rotation3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position3 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation3 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainedObject {
    #[serde(rename = "GUID")]
    pub guid: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Transform")]
    pub transform: Option<Transform2>,
    #[serde(rename = "Nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "GMNotes")]
    pub gmnotes: Option<String>,
    #[serde(rename = "AltLookAngle")]
    pub alt_look_angle: Option<AltLookAngle2>,
    #[serde(rename = "ColorDiffuse")]
    pub color_diffuse: Option<ColorDiffuse2>,
    #[serde(rename = "LayoutGroupSortIndex")]
    pub layout_group_sort_index: Option<i64>,
    #[serde(rename = "Value")]
    pub value: Option<i64>,
    #[serde(rename = "Locked")]
    pub locked: Option<bool>,
    #[serde(rename = "Grid")]
    pub grid: Option<bool>,
    #[serde(rename = "Snap")]
    pub snap: Option<bool>,
    #[serde(rename = "IgnoreFoW")]
    pub ignore_fo_w: Option<bool>,
    #[serde(rename = "MeasureMovement")]
    pub measure_movement: Option<bool>,
    #[serde(rename = "DragSelectable")]
    pub drag_selectable: Option<bool>,
    #[serde(rename = "Autoraise")]
    pub autoraise: Option<bool>,
    #[serde(rename = "Sticky")]
    pub sticky: Option<bool>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<bool>,
    #[serde(rename = "GridProjection")]
    pub grid_projection: Option<bool>,
    #[serde(rename = "HideWhenFaceDown")]
    pub hide_when_face_down: Option<bool>,
    #[serde(rename = "Hands")]
    pub hands: Option<bool>,
    #[serde(rename = "SidewaysCard")]
    pub sideways_card: Option<Option<bool>>,
    #[serde(rename = "DeckIDs")]
    #[serde(default)]
    pub deck_ids: Option<Vec<i64>>,
    #[serde(rename = "CustomDeck")]
    pub custom_deck: Option<CustomDeck2>,
    #[serde(rename = "LuaScript")]
    pub lua_script: Option<String>,
    #[serde(rename = "LuaScriptState")]
    pub lua_script_state: Option<String>,
    #[serde(rename = "XmlUI")]
    pub xml_ui: Option<String>,
    #[serde(rename = "ContainedObjects")]
    #[serde(default)]
    pub contained_objects: Option<Vec<ContainedObject2>>,
    #[serde(rename = "CardID")]
    pub card_id: Option<Option<i64>>,
    #[serde(rename = "CustomImage")]
    pub custom_image: Option<CustomImage3>,
    #[serde(rename = "AttachedDecals")]
    #[serde(default)]
    pub attached_decals: Option<Vec<AttachedDecal2>>,
    #[serde(rename = "MaterialIndex")]
    pub material_index: Option<Option<i64>>,
    #[serde(rename = "RotationValues")]
    pub rotation_values: Option<Vec<RotationValue>>,
    #[serde(rename = "MeshIndex")]
    pub mesh_index: Option<Option<i64>>,
    #[serde(rename = "CustomMesh")]
    pub custom_mesh: Option<CustomMesh2>,
    #[serde(rename = "Bag")]
    pub bag: Option<Bag2>,
    #[serde(rename = "PhysicsMaterial")]
    pub physics_material: Option<PhysicsMaterial2>,
    #[serde(rename = "Rigidbody")]
    pub rigidbody: Option<Rigidbody2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform2 {
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
    pub pos_z: Option<f64>,
    pub rot_x: Option<f64>,
    pub rot_y: Option<f64>,
    pub rot_z: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltLookAngle2 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorDiffuse2 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
    pub a: Option<Option<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDeck2 {
    #[serde(rename = "1887")]
    pub n1887: Option<n1887>,
    #[serde(rename = "1345")]
    pub n1345: Option<n1345>,
    #[serde(rename = "2158")]
    pub n2158: Option<n2158>,
    #[serde(rename = "2427")]
    pub n2427: Option<n2427>,
    #[serde(rename = "2159")]
    pub n2159: Option<n2159>,
    #[serde(rename = "2157")]
    pub n2157: Option<n2157>,
    #[serde(rename = "691")]
    pub n691: Option<n691>,
    #[serde(rename = "690")]
    pub n690: Option<n690>,
    #[serde(rename = "694")]
    pub n694: Option<n694>,
    #[serde(rename = "689")]
    pub n689: Option<n689>,
    #[serde(rename = "696")]
    pub n696: Option<n696>,
    #[serde(rename = "693")]
    pub n693: Option<n693>,
    #[serde(rename = "695")]
    pub n695: Option<n695>,
    #[serde(rename = "692")]
    pub n692: Option<n692>,
    #[serde(rename = "688")]
    pub n688: Option<n688>,
    #[serde(rename = "1985")]
    pub n1985: Option<n1985>,
    #[serde(rename = "1898")]
    pub n1898: Option<n1898>,
    #[serde(rename = "992")]
    pub n992: Option<n992>,
    #[serde(rename = "981")]
    pub n981: Option<n9812>,
    #[serde(rename = "1889")]
    pub n1889: Option<n1889>,
    #[serde(rename = "1888")]
    pub n1888: Option<n1888>,
    #[serde(rename = "857")]
    pub n857: Option<n857>,
    #[serde(rename = "1072")]
    pub n1072: Option<n1072>,
    #[serde(rename = "1071")]
    pub n1071: Option<n1071>,
    #[serde(rename = "1073")]
    pub n1073: Option<n1073>,
    #[serde(rename = "1673")]
    pub n1673: Option<n1673>,
    #[serde(rename = "1024")]
    pub n1024: Option<n1024>,
    #[serde(rename = "990")]
    pub n990: Option<n990>,
    #[serde(rename = "991")]
    pub n991: Option<n991>,
    #[serde(rename = "1357")]
    pub n1357: Option<n1357>,
    #[serde(rename = "1355")]
    pub n1355: Option<n1355>,
    #[serde(rename = "1068")]
    pub n1068: Option<n1068>,
    #[serde(rename = "1939")]
    pub n1939: Option<n1939>,
    #[serde(rename = "1358")]
    pub n1358: Option<n1358>,
    #[serde(rename = "1356")]
    pub n1356: Option<n1356>,
    #[serde(rename = "930")]
    pub n930: Option<n930>,
    #[serde(rename = "984")]
    pub n984: Option<n9842>,
    #[serde(rename = "989")]
    pub n989: Option<n9892>,
    #[serde(rename = "987")]
    pub n987: Option<n9872>,
    #[serde(rename = "978")]
    pub n978: Option<n9782>,
    #[serde(rename = "936")]
    pub n936: Option<n936>,
    #[serde(rename = "937")]
    pub n937: Option<n937>,
    #[serde(rename = "933")]
    pub n933: Option<n933>,
    #[serde(rename = "938")]
    pub n938: Option<n938>,
    #[serde(rename = "940")]
    pub n940: Option<n940>,
    #[serde(rename = "939")]
    pub n939: Option<n939>,
    #[serde(rename = "934")]
    pub n934: Option<n934>,
    #[serde(rename = "935")]
    pub n935: Option<n935>,
    #[serde(rename = "941")]
    pub n941: Option<n941>,
    #[serde(rename = "932")]
    pub n932: Option<n932>,
    #[serde(rename = "2229")]
    pub n2229: Option<n2229>,
    #[serde(rename = "2227")]
    pub n2227: Option<n2227>,
    #[serde(rename = "2226")]
    pub n2226: Option<n2226>,
    #[serde(rename = "2215")]
    pub n2215: Option<n2215>,
    #[serde(rename = "2214")]
    pub n2214: Option<n2214>,
    #[serde(rename = "2213")]
    pub n2213: Option<n2213>,
    #[serde(rename = "2212")]
    pub n2212: Option<n2212>,
    #[serde(rename = "1961")]
    pub n1961: Option<n1961>,
    #[serde(rename = "1962")]
    pub n1962: Option<n1962>,
    #[serde(rename = "1899")]
    pub n1899: Option<n1899>,
    #[serde(rename = "2211")]
    pub n2211: Option<n2211>,
    #[serde(rename = "728")]
    pub n728: Option<n728>,
    #[serde(rename = "726")]
    pub n726: Option<n726>,
    #[serde(rename = "1983")]
    pub n1983: Option<n1983>,
    #[serde(rename = "2216")]
    pub n2216: Option<n2216>,
    #[serde(rename = "727")]
    pub n727: Option<n727>,
    #[serde(rename = "725")]
    pub n725: Option<n725>,
    #[serde(rename = "998")]
    pub n998: Option<n998>,
    #[serde(rename = "997")]
    pub n997: Option<n997>,
    #[serde(rename = "996")]
    pub n996: Option<n996>,
    #[serde(rename = "995")]
    pub n995: Option<n995>,
    #[serde(rename = "993")]
    pub n993: Option<n993>,
    #[serde(rename = "1043")]
    pub n1043: Option<n1043>,
    #[serde(rename = "977")]
    pub n977: Option<n9772>,
    #[serde(rename = "1000")]
    pub n1000: Option<n1000>,
    #[serde(rename = "979")]
    pub n979: Option<n9792>,
    #[serde(rename = "980")]
    pub n980: Option<n980>,
    #[serde(rename = "712")]
    pub n712: Option<n712>,
    #[serde(rename = "769")]
    pub n769: Option<n769>,
    #[serde(rename = "896")]
    pub n896: Option<n896>,
    #[serde(rename = "895")]
    pub n895: Option<n895>,
    #[serde(rename = "893")]
    pub n893: Option<n893>,
    #[serde(rename = "894")]
    pub n894: Option<n894>,
    #[serde(rename = "856")]
    pub n856: Option<n856>,
    #[serde(rename = "859")]
    pub n859: Option<n859>,
    #[serde(rename = "858")]
    pub n858: Option<n858>,
    #[serde(rename = "800")]
    pub n800: Option<n800>,
    #[serde(rename = "801")]
    pub n801: Option<n801>,
    #[serde(rename = "802")]
    pub n802: Option<n802>,
    #[serde(rename = "765")]
    pub n765: Option<n765>,
    #[serde(rename = "803")]
    pub n803: Option<n803>,
    #[serde(rename = "770")]
    pub n770: Option<n770>,
    #[serde(rename = "855")]
    pub n855: Option<n855>,
    #[serde(rename = "799")]
    pub n799: Option<n799>,
    #[serde(rename = "768")]
    pub n768: Option<n768>,
    #[serde(rename = "767")]
    pub n767: Option<n767>,
    #[serde(rename = "766")]
    pub n766: Option<n766>,
    #[serde(rename = "826")]
    pub n826: Option<n826>,
    #[serde(rename = "825")]
    pub n825: Option<n825>,
    #[serde(rename = "822")]
    pub n822: Option<n822>,
    #[serde(rename = "824")]
    pub n824: Option<n824>,
    #[serde(rename = "823")]
    pub n823: Option<n823>,
    #[serde(rename = "852")]
    pub n852: Option<n852>,
    #[serde(rename = "853")]
    pub n853: Option<n853>,
    #[serde(rename = "796")]
    pub n796: Option<n796>,
    #[serde(rename = "854")]
    pub n854: Option<n854>,
    #[serde(rename = "798")]
    pub n798: Option<n798>,
    #[serde(rename = "811")]
    pub n811: Option<n811>,
    #[serde(rename = "814")]
    pub n814: Option<n814>,
    #[serde(rename = "813")]
    pub n813: Option<n813>,
    #[serde(rename = "810")]
    pub n810: Option<n810>,
    #[serde(rename = "812")]
    pub n812: Option<n812>,
    #[serde(rename = "679")]
    pub n679: Option<n679>,
    #[serde(rename = "809")]
    pub n809: Option<n809>,
    #[serde(rename = "808")]
    pub n808: Option<n808>,
    #[serde(rename = "815")]
    pub n815: Option<n815>,
    #[serde(rename = "816")]
    pub n816: Option<n816>,
    #[serde(rename = "817")]
    pub n817: Option<n817>,
    #[serde(rename = "775")]
    pub n775: Option<n775>,
    #[serde(rename = "772")]
    pub n772: Option<n772>,
    #[serde(rename = "773")]
    pub n773: Option<n773>,
    #[serde(rename = "774")]
    pub n774: Option<n774>,
    #[serde(rename = "771")]
    pub n771: Option<n771>,
    #[serde(rename = "681")]
    pub n681: Option<n681>,
    #[serde(rename = "682")]
    pub n682: Option<n682>,
    #[serde(rename = "680")]
    pub n680: Option<n680>,
    #[serde(rename = "794")]
    pub n794: Option<n794>,
    #[serde(rename = "795")]
    pub n795: Option<n795>,
    #[serde(rename = "684")]
    pub n684: Option<n684>,
    #[serde(rename = "844")]
    pub n844: Option<n844>,
    #[serde(rename = "683")]
    pub n683: Option<n683>,
    #[serde(rename = "845")]
    pub n845: Option<n845>,
    #[serde(rename = "751")]
    pub n751: Option<n751>,
    #[serde(rename = "672")]
    pub n672: Option<n672>,
    #[serde(rename = "676")]
    pub n676: Option<n676>,
    #[serde(rename = "675")]
    pub n675: Option<n675>,
    #[serde(rename = "674")]
    pub n674: Option<n674>,
    #[serde(rename = "673")]
    pub n673: Option<n673>,
    #[serde(rename = "677")]
    pub n677: Option<n677>,
    #[serde(rename = "708")]
    pub n708: Option<n708>,
    #[serde(rename = "671")]
    pub n671: Option<n671>,
    #[serde(rename = "678")]
    pub n678: Option<n678>,
    #[serde(rename = "670")]
    pub n670: Option<n670>,
    #[serde(rename = "792")]
    pub n792: Option<n792>,
    #[serde(rename = "700")]
    pub n700: Option<n700>,
    #[serde(rename = "698")]
    pub n698: Option<n698>,
    #[serde(rename = "707")]
    pub n707: Option<n707>,
    #[serde(rename = "706")]
    pub n706: Option<n706>,
    #[serde(rename = "793")]
    pub n793: Option<n793>,
    #[serde(rename = "703")]
    pub n703: Option<n703>,
    #[serde(rename = "699")]
    pub n699: Option<n699>,
    #[serde(rename = "704")]
    pub n704: Option<n704>,
    #[serde(rename = "705")]
    pub n705: Option<n705>,
    #[serde(rename = "662")]
    pub n662: Option<n662>,
    #[serde(rename = "709")]
    pub n709: Option<n709>,
    #[serde(rename = "668")]
    pub n668: Option<n668>,
    #[serde(rename = "661")]
    pub n661: Option<n661>,
    #[serde(rename = "710")]
    pub n710: Option<n710>,
    #[serde(rename = "715")]
    pub n715: Option<n715>,
    #[serde(rename = "713")]
    pub n713: Option<n713>,
    #[serde(rename = "711")]
    pub n711: Option<n711>,
    #[serde(rename = "714")]
    pub n714: Option<n714>,
    #[serde(rename = "659")]
    pub n659: Option<n659>,
    #[serde(rename = "716")]
    pub n716: Option<n716>,
    #[serde(rename = "664")]
    pub n664: Option<n664>,
    #[serde(rename = "666")]
    pub n666: Option<n666>,
    #[serde(rename = "665")]
    pub n665: Option<n665>,
    #[serde(rename = "656")]
    pub n656: Option<n656>,
    #[serde(rename = "657")]
    pub n657: Option<n657>,
    #[serde(rename = "667")]
    pub n667: Option<n667>,
    #[serde(rename = "655")]
    pub n655: Option<n655>,
    #[serde(rename = "658")]
    pub n658: Option<n658>,
    #[serde(rename = "646")]
    pub n646: Option<n646>,
    #[serde(rename = "724")]
    pub n724: Option<n724>,
    #[serde(rename = "650")]
    pub n650: Option<n650>,
    #[serde(rename = "648")]
    pub n648: Option<n648>,
    #[serde(rename = "651")]
    pub n651: Option<n651>,
    #[serde(rename = "644")]
    pub n644: Option<n644>,
    #[serde(rename = "652")]
    pub n652: Option<n652>,
    #[serde(rename = "647")]
    pub n647: Option<n647>,
    #[serde(rename = "649")]
    pub n649: Option<n649>,
    #[serde(rename = "645")]
    pub n645: Option<n645>,
    #[serde(rename = "738")]
    pub n738: Option<n738>,
    #[serde(rename = "737")]
    pub n737: Option<n737>,
    #[serde(rename = "790")]
    pub n790: Option<n790>,
    #[serde(rename = "736")]
    pub n736: Option<n736>,
    #[serde(rename = "742")]
    pub n742: Option<n742>,
    #[serde(rename = "741")]
    pub n741: Option<n741>,
    #[serde(rename = "739")]
    pub n739: Option<n739>,
    #[serde(rename = "791")]
    pub n791: Option<n791>,
    #[serde(rename = "740")]
    pub n740: Option<n740>,
    #[serde(rename = "663")]
    pub n663: Option<n663>,
    #[serde(rename = "723")]
    pub n723: Option<n723>,
    #[serde(rename = "722")]
    pub n722: Option<n722>,
    #[serde(rename = "719")]
    pub n719: Option<n719>,
    #[serde(rename = "660")]
    pub n660: Option<n660>,
    #[serde(rename = "721")]
    pub n721: Option<n721>,
    #[serde(rename = "789")]
    pub n789: Option<n789>,
    #[serde(rename = "718")]
    pub n718: Option<n718>,
    #[serde(rename = "720")]
    pub n720: Option<n720>,
    #[serde(rename = "717")]
    pub n717: Option<n717>,
    #[serde(rename = "1900")]
    pub n1900: Option<n1900>,
    #[serde(rename = "1901")]
    pub n1901: Option<n1901>,
    #[serde(rename = "1096")]
    pub n1096: Option<n1096>,
    #[serde(rename = "1844")]
    pub n1844: Option<n1844>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1887 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1345 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2158 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2427 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2159 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2157 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n691 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n690 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n694 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n689 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n696 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n693 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n695 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n692 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n688 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1985 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1898 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n992 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9812 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1889 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1888 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n857 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1072 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1071 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1073 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1673 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1024 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n990 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n991 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1357 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1355 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1068 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1939 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1358 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1356 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n930 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9842 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9892 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9872 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9782 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n936 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n937 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n933 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n938 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n940 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n939 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n934 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n935 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n941 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n932 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2229 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2227 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2226 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2215 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2214 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2213 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2212 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1961 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1962 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1899 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2211 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n728 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n726 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1983 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2216 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n727 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n725 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n998 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n997 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n996 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n995 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n993 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1043 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9772 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1000 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9792 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n980 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n712 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n769 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n896 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n895 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n893 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n894 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n856 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n859 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n858 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n800 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n801 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n802 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n765 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n803 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n770 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n855 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n799 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n768 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n767 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n766 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n826 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n825 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n822 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n824 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n823 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n852 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n853 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n796 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n854 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n798 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n811 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n814 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n813 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n810 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n812 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n679 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n809 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n808 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n815 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n816 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n817 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n775 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n772 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n773 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n774 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n771 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n681 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n682 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n680 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n794 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n795 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n684 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n844 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n683 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n845 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n751 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n672 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n676 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n675 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n674 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n673 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n677 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n708 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n671 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n678 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n670 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n792 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n700 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n698 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n707 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n706 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n793 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n703 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n699 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n704 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n705 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n662 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n709 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n668 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n661 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n710 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n715 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n713 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n711 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n714 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n659 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n716 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n664 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n666 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n665 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n656 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n657 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n667 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n655 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n658 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n646 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n724 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n650 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n648 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n651 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n644 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n652 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n647 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n649 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n645 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n738 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n737 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n790 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n736 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n742 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n741 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n739 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n791 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n740 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n663 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n723 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n722 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n719 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n660 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n721 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n789 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n718 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n720 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n717 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1900 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1901 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1096 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1844 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainedObject2 {
    #[serde(rename = "GUID")]
    pub guid: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Transform")]
    pub transform: Transform3,
    #[serde(rename = "Nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "GMNotes")]
    pub gmnotes: Option<String>,
    #[serde(rename = "AltLookAngle")]
    pub alt_look_angle: AltLookAngle3,
    #[serde(rename = "ColorDiffuse")]
    pub color_diffuse: ColorDiffuse3,
    #[serde(rename = "LayoutGroupSortIndex")]
    pub layout_group_sort_index: Option<i64>,
    #[serde(rename = "Value")]
    pub value: Option<i64>,
    #[serde(rename = "Locked")]
    pub locked: Option<bool>,
    #[serde(rename = "Grid")]
    pub grid: Option<bool>,
    #[serde(rename = "Snap")]
    pub snap: Option<bool>,
    #[serde(rename = "IgnoreFoW")]
    pub ignore_fo_w: Option<bool>,
    #[serde(rename = "MeasureMovement")]
    pub measure_movement: Option<bool>,
    #[serde(rename = "DragSelectable")]
    pub drag_selectable: Option<bool>,
    #[serde(rename = "Autoraise")]
    pub autoraise: Option<bool>,
    #[serde(rename = "Sticky")]
    pub sticky: Option<bool>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<bool>,
    #[serde(rename = "GridProjection")]
    pub grid_projection: Option<bool>,
    #[serde(rename = "HideWhenFaceDown")]
    pub hide_when_face_down: Option<Option<bool>>,
    #[serde(rename = "Hands")]
    pub hands: Option<bool>,
    #[serde(rename = "CustomImage")]
    pub custom_image: Option<CustomImage2>,
    #[serde(rename = "LuaScript")]
    pub lua_script: Option<String>,
    #[serde(rename = "LuaScriptState")]
    pub lua_script_state: Option<String>,
    #[serde(rename = "XmlUI")]
    pub xml_ui: Option<String>,
    #[serde(rename = "CardID")]
    pub card_id: Option<Option<i64>>,
    #[serde(rename = "SidewaysCard")]
    pub sideways_card: Option<Option<bool>>,
    #[serde(rename = "CustomDeck")]
    pub custom_deck: Option<CustomDeck3>,
    #[serde(rename = "ContainedObjects")]
    #[serde(default)]
    pub contained_objects: Option<Vec<Value>>,
    #[serde(rename = "AttachedDecals")]
    #[serde(default)]
    pub attached_decals: Option<Vec<AttachedDecal>>,
    #[serde(rename = "AttachedSnapPoints")]
    pub attached_snap_points: Option<Vec<AttachedSnapPoint2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform3 {
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
    pub pos_z: Option<f64>,
    pub rot_x: Option<f64>,
    pub rot_y: Option<f64>,
    pub rot_z: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltLookAngle3 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorDiffuse3 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomImage2 {
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "ImageSecondaryURL")]
    pub image_secondary_url: Option<String>,
    #[serde(rename = "ImageScalar")]
    pub image_scalar: Option<f64>,
    #[serde(rename = "WidthScale")]
    pub width_scale: Option<f64>,
    #[serde(rename = "CustomTile")]
    pub custom_tile: CustomTile2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomTile2 {
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "Stackable")]
    pub stackable: Option<bool>,
    #[serde(rename = "Stretch")]
    pub stretch: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDeck3 {
    #[serde(rename = "1887")]
    pub n1887: Option<n18872>,
    #[serde(rename = "1345")]
    pub n1345: Option<n13452>,
    #[serde(rename = "991")]
    pub n991: Option<n9912>,
    #[serde(rename = "1900")]
    pub n1900: Option<n19002>,
    #[serde(rename = "1899")]
    pub n1899: Option<n18992>,
    #[serde(rename = "990")]
    pub n990: Option<n9902>,
    #[serde(rename = "992")]
    pub n992: Option<n9922>,
    #[serde(rename = "1898")]
    pub n1898: Option<n18982>,
    #[serde(rename = "981")]
    pub n981: Option<n9813>,
    #[serde(rename = "1889")]
    pub n1889: Option<n18892>,
    #[serde(rename = "1888")]
    pub n1888: Option<n18882>,
    #[serde(rename = "702")]
    pub n702: Option<n702>,
    #[serde(rename = "1072")]
    pub n1072: Option<n10722>,
    #[serde(rename = "1071")]
    pub n1071: Option<n10712>,
    #[serde(rename = "1073")]
    pub n1073: Option<n10732>,
    #[serde(rename = "1673")]
    pub n1673: Option<n16732>,
    #[serde(rename = "1024")]
    pub n1024: Option<n10242>,
    #[serde(rename = "1357")]
    pub n1357: Option<n13572>,
    #[serde(rename = "1355")]
    pub n1355: Option<n13552>,
    #[serde(rename = "999")]
    pub n999: Option<n999>,
    #[serde(rename = "1939")]
    pub n1939: Option<n19392>,
    #[serde(rename = "1358")]
    pub n1358: Option<n13582>,
    #[serde(rename = "1356")]
    pub n1356: Option<n13562>,
    #[serde(rename = "930")]
    pub n930: Option<n9302>,
    #[serde(rename = "984")]
    pub n984: Option<n9843>,
    #[serde(rename = "989")]
    pub n989: Option<n9893>,
    #[serde(rename = "987")]
    pub n987: Option<n9873>,
    #[serde(rename = "978")]
    pub n978: Option<n9783>,
    #[serde(rename = "936")]
    pub n936: Option<n9362>,
    #[serde(rename = "937")]
    pub n937: Option<n9372>,
    #[serde(rename = "933")]
    pub n933: Option<n9332>,
    #[serde(rename = "938")]
    pub n938: Option<n9382>,
    #[serde(rename = "940")]
    pub n940: Option<n9402>,
    #[serde(rename = "939")]
    pub n939: Option<n9392>,
    #[serde(rename = "934")]
    pub n934: Option<n9342>,
    #[serde(rename = "935")]
    pub n935: Option<n9352>,
    #[serde(rename = "941")]
    pub n941: Option<n9412>,
    #[serde(rename = "932")]
    pub n932: Option<n9322>,
    #[serde(rename = "2229")]
    pub n2229: Option<n22292>,
    #[serde(rename = "2227")]
    pub n2227: Option<n22272>,
    #[serde(rename = "2226")]
    pub n2226: Option<n22262>,
    #[serde(rename = "2215")]
    pub n2215: Option<n22152>,
    #[serde(rename = "2214")]
    pub n2214: Option<n22142>,
    #[serde(rename = "2213")]
    pub n2213: Option<n22132>,
    #[serde(rename = "2212")]
    pub n2212: Option<n22122>,
    #[serde(rename = "1890")]
    pub n1890: Option<n1890>,
    #[serde(rename = "728")]
    pub n728: Option<n7282>,
    #[serde(rename = "726")]
    pub n726: Option<n7262>,
    #[serde(rename = "1983")]
    pub n1983: Option<n19832>,
    #[serde(rename = "727")]
    pub n727: Option<n7272>,
    #[serde(rename = "998")]
    pub n998: Option<n9982>,
    #[serde(rename = "997")]
    pub n997: Option<n9972>,
    #[serde(rename = "996")]
    pub n996: Option<n9962>,
    #[serde(rename = "995")]
    pub n995: Option<n9952>,
    #[serde(rename = "1043")]
    pub n1043: Option<n10432>,
    #[serde(rename = "977")]
    pub n977: Option<n9773>,
    #[serde(rename = "979")]
    pub n979: Option<n9793>,
    #[serde(rename = "980")]
    pub n980: Option<n9802>,
    #[serde(rename = "712")]
    pub n712: Option<n7122>,
    #[serde(rename = "769")]
    pub n769: Option<n7692>,
    #[serde(rename = "834")]
    pub n834: Option<n834>,
    #[serde(rename = "835")]
    pub n835: Option<n835>,
    #[serde(rename = "837")]
    pub n837: Option<n837>,
    #[serde(rename = "836")]
    pub n836: Option<n836>,
    #[serde(rename = "827")]
    pub n827: Option<n827>,
    #[serde(rename = "833")]
    pub n833: Option<n833>,
    #[serde(rename = "832")]
    pub n832: Option<n832>,
    #[serde(rename = "695")]
    pub n695: Option<n6952>,
    #[serde(rename = "694")]
    pub n694: Option<n6942>,
    #[serde(rename = "696")]
    pub n696: Option<n6962>,
    #[serde(rename = "765")]
    pub n765: Option<n7652>,
    #[serde(rename = "797")]
    pub n797: Option<n797>,
    #[serde(rename = "689")]
    pub n689: Option<n6892>,
    #[serde(rename = "768")]
    pub n768: Option<n7682>,
    #[serde(rename = "767")]
    pub n767: Option<n7672>,
    #[serde(rename = "766")]
    pub n766: Option<n7662>,
    #[serde(rename = "826")]
    pub n826: Option<n8262>,
    #[serde(rename = "825")]
    pub n825: Option<n8252>,
    #[serde(rename = "822")]
    pub n822: Option<n8222>,
    #[serde(rename = "824")]
    pub n824: Option<n8242>,
    #[serde(rename = "823")]
    pub n823: Option<n8232>,
    #[serde(rename = "793")]
    pub n793: Option<n7932>,
    #[serde(rename = "792")]
    pub n792: Option<n7922>,
    #[serde(rename = "795")]
    pub n795: Option<n7952>,
    #[serde(rename = "794")]
    pub n794: Option<n7942>,
    #[serde(rename = "791")]
    pub n791: Option<n7912>,
    #[serde(rename = "811")]
    pub n811: Option<n8112>,
    #[serde(rename = "814")]
    pub n814: Option<n8142>,
    #[serde(rename = "813")]
    pub n813: Option<n8132>,
    #[serde(rename = "810")]
    pub n810: Option<n8102>,
    #[serde(rename = "812")]
    pub n812: Option<n8122>,
    #[serde(rename = "679")]
    pub n679: Option<n6792>,
    #[serde(rename = "809")]
    pub n809: Option<n8092>,
    #[serde(rename = "808")]
    pub n808: Option<n8082>,
    #[serde(rename = "691")]
    pub n691: Option<n6912>,
    #[serde(rename = "692")]
    pub n692: Option<n6922>,
    #[serde(rename = "690")]
    pub n690: Option<n6902>,
    #[serde(rename = "775")]
    pub n775: Option<n7752>,
    #[serde(rename = "772")]
    pub n772: Option<n7722>,
    #[serde(rename = "773")]
    pub n773: Option<n7732>,
    #[serde(rename = "774")]
    pub n774: Option<n7742>,
    #[serde(rename = "771")]
    pub n771: Option<n7712>,
    #[serde(rename = "681")]
    pub n681: Option<n6812>,
    #[serde(rename = "682")]
    pub n682: Option<n6822>,
    #[serde(rename = "680")]
    pub n680: Option<n6802>,
    #[serde(rename = "687")]
    pub n687: Option<n687>,
    #[serde(rename = "685")]
    pub n685: Option<n685>,
    #[serde(rename = "684")]
    pub n684: Option<n6842>,
    #[serde(rename = "686")]
    pub n686: Option<n686>,
    #[serde(rename = "683")]
    pub n683: Option<n6832>,
    #[serde(rename = "688")]
    pub n688: Option<n6882>,
    #[serde(rename = "751")]
    pub n751: Option<n7512>,
    #[serde(rename = "672")]
    pub n672: Option<n6722>,
    #[serde(rename = "676")]
    pub n676: Option<n6762>,
    #[serde(rename = "675")]
    pub n675: Option<n6752>,
    #[serde(rename = "674")]
    pub n674: Option<n6742>,
    #[serde(rename = "673")]
    pub n673: Option<n6732>,
    #[serde(rename = "677")]
    pub n677: Option<n6772>,
    #[serde(rename = "708")]
    pub n708: Option<n7082>,
    #[serde(rename = "671")]
    pub n671: Option<n6712>,
    #[serde(rename = "678")]
    pub n678: Option<n6782>,
    #[serde(rename = "670")]
    pub n670: Option<n6702>,
    #[serde(rename = "701")]
    pub n701: Option<n701>,
    #[serde(rename = "700")]
    pub n700: Option<n7002>,
    #[serde(rename = "698")]
    pub n698: Option<n6982>,
    #[serde(rename = "707")]
    pub n707: Option<n7072>,
    #[serde(rename = "706")]
    pub n706: Option<n7062>,
    #[serde(rename = "703")]
    pub n703: Option<n7032>,
    #[serde(rename = "699")]
    pub n699: Option<n6992>,
    #[serde(rename = "704")]
    pub n704: Option<n7042>,
    #[serde(rename = "705")]
    pub n705: Option<n7052>,
    #[serde(rename = "662")]
    pub n662: Option<n6622>,
    #[serde(rename = "709")]
    pub n709: Option<n7092>,
    #[serde(rename = "668")]
    pub n668: Option<n6682>,
    #[serde(rename = "661")]
    pub n661: Option<n6612>,
    #[serde(rename = "710")]
    pub n710: Option<n7102>,
    #[serde(rename = "715")]
    pub n715: Option<n7152>,
    #[serde(rename = "713")]
    pub n713: Option<n7132>,
    #[serde(rename = "711")]
    pub n711: Option<n7112>,
    #[serde(rename = "714")]
    pub n714: Option<n7142>,
    #[serde(rename = "659")]
    pub n659: Option<n6592>,
    #[serde(rename = "716")]
    pub n716: Option<n7162>,
    #[serde(rename = "664")]
    pub n664: Option<n6642>,
    #[serde(rename = "666")]
    pub n666: Option<n6662>,
    #[serde(rename = "665")]
    pub n665: Option<n6652>,
    #[serde(rename = "656")]
    pub n656: Option<n6562>,
    #[serde(rename = "657")]
    pub n657: Option<n6572>,
    #[serde(rename = "667")]
    pub n667: Option<n6672>,
    #[serde(rename = "655")]
    pub n655: Option<n6552>,
    #[serde(rename = "658")]
    pub n658: Option<n6582>,
    #[serde(rename = "646")]
    pub n646: Option<n6462>,
    #[serde(rename = "724")]
    pub n724: Option<n7242>,
    #[serde(rename = "650")]
    pub n650: Option<n6502>,
    #[serde(rename = "648")]
    pub n648: Option<n6482>,
    #[serde(rename = "651")]
    pub n651: Option<n6512>,
    #[serde(rename = "644")]
    pub n644: Option<n6442>,
    #[serde(rename = "652")]
    pub n652: Option<n6522>,
    #[serde(rename = "647")]
    pub n647: Option<n6472>,
    #[serde(rename = "649")]
    pub n649: Option<n6492>,
    #[serde(rename = "645")]
    pub n645: Option<n6452>,
    #[serde(rename = "738")]
    pub n738: Option<n7382>,
    #[serde(rename = "737")]
    pub n737: Option<n7372>,
    #[serde(rename = "697")]
    pub n697: Option<n697>,
    #[serde(rename = "736")]
    pub n736: Option<n7362>,
    #[serde(rename = "742")]
    pub n742: Option<n7422>,
    #[serde(rename = "741")]
    pub n741: Option<n7412>,
    #[serde(rename = "739")]
    pub n739: Option<n7392>,
    #[serde(rename = "735")]
    pub n735: Option<n735>,
    #[serde(rename = "740")]
    pub n740: Option<n7402>,
    #[serde(rename = "663")]
    pub n663: Option<n6632>,
    #[serde(rename = "723")]
    pub n723: Option<n7232>,
    #[serde(rename = "722")]
    pub n722: Option<n7222>,
    #[serde(rename = "719")]
    pub n719: Option<n7192>,
    #[serde(rename = "660")]
    pub n660: Option<n6602>,
    #[serde(rename = "721")]
    pub n721: Option<n7212>,
    #[serde(rename = "654")]
    pub n654: Option<n654>,
    #[serde(rename = "718")]
    pub n718: Option<n7182>,
    #[serde(rename = "720")]
    pub n720: Option<n7202>,
    #[serde(rename = "717")]
    pub n717: Option<n7172>,
    #[serde(rename = "1901")]
    pub n1901: Option<n19012>,
    #[serde(rename = "2216")]
    pub n2216: Option<n22162>,
    #[serde(rename = "1844")]
    pub n1844: Option<n18442>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n18872 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n13452 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9912 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n19002 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n18992 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9902 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9922 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n18982 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9813 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n18892 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n18882 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n702 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10722 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10712 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10732 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n16732 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10242 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n13572 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n13552 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n999 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n19392 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n13582 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n13562 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9302 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9843 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9893 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9873 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9783 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9362 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9372 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9332 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9382 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9402 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9392 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9342 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9352 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9412 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9322 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22292 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22272 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22262 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22152 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22142 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22132 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22122 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1890 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7282 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7262 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n19832 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7272 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9982 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9972 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9962 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9952 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10432 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9773 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9793 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9802 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7122 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7692 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n834 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n835 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n837 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n836 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n827 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n833 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n832 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6952 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6942 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6962 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7652 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n797 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6892 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7682 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7672 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7662 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8262 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8252 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8222 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8242 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8232 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7932 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7922 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7952 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7942 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7912 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8112 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8142 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8132 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8102 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8122 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6792 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8092 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8082 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6912 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6922 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6902 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7752 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7722 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7732 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7742 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7712 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6812 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6822 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6802 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n687 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n685 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6842 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n686 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6832 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6882 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7512 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6722 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6762 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6752 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6742 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6732 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6772 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7082 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6712 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6782 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6702 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n701 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7002 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6982 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7072 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7062 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7032 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6992 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7042 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7052 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6622 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7092 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6682 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6612 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7102 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7152 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7132 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7112 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7142 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6592 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7162 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6642 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6662 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6652 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6562 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6572 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6672 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6552 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6582 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6462 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7242 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6502 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6482 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6512 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6442 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6522 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6472 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6492 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6452 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7382 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7372 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n697 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7362 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7422 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7412 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7392 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n735 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7402 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6632 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7232 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7222 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7192 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6602 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7212 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n654 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7182 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7202 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7172 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n19012 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22162 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n18442 {
    #[serde(rename = "FaceURL")]
    pub face_url: Option<String>,
    #[serde(rename = "BackURL")]
    pub back_url: Option<String>,
    #[serde(rename = "NumWidth")]
    pub num_width: Option<i64>,
    #[serde(rename = "NumHeight")]
    pub num_height: Option<i64>,
    #[serde(rename = "BackIsHidden")]
    pub back_is_hidden: Option<bool>,
    #[serde(rename = "UniqueBack")]
    pub unique_back: Option<bool>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedDecal {
    #[serde(rename = "Transform")]
    pub transform: Option<Transform4>,
    #[serde(rename = "CustomDecal")]
    pub custom_decal: Option<CustomDecal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform4 {
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
    pub pos_z: Option<f64>,
    pub rot_x: Option<f64>,
    pub rot_y: Option<f64>,
    pub rot_z: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDecal {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedSnapPoint2 {
    #[serde(rename = "Position")]
    pub position: Option<Position4>,
    #[serde(rename = "Rotation")]
    pub rotation: Option<Rotation4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position4 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation4 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomImage3 {
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "ImageSecondaryURL")]
    pub image_secondary_url: Option<String>,
    #[serde(rename = "ImageScalar")]
    pub image_scalar: Option<f64>,
    #[serde(rename = "WidthScale")]
    pub width_scale: Option<f64>,
    #[serde(rename = "CustomTile")]
    pub custom_tile: Option<CustomTile3>,
    #[serde(rename = "CustomToken")]
    pub custom_token: Option<CustomToken2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomTile3 {
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "Stackable")]
    pub stackable: Option<bool>,
    #[serde(rename = "Stretch")]
    pub stretch: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomToken2 {
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "MergeDistancePixels")]
    pub merge_distance_pixels: Option<f64>,
    #[serde(rename = "StandUp")]
    pub stand_up: Option<bool>,
    #[serde(rename = "Stackable")]
    pub stackable: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedDecal2 {
    #[serde(rename = "Transform")]
    pub transform: Option<Transform5>,
    #[serde(rename = "CustomDecal")]
    pub custom_decal: Option<CustomDecal2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform5 {
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
    pub pos_z: Option<f64>,
    pub rot_x: Option<f64>,
    pub rot_y: Option<f64>,
    pub rot_z: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDecal2 {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotationValue {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Rotation")]
    pub rotation: Option<Rotation5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation5 {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomMesh2 {
    #[serde(rename = "MeshURL")]
    pub mesh_url: Option<String>,
    #[serde(rename = "DiffuseURL")]
    pub diffuse_url: Option<String>,
    #[serde(rename = "NormalURL")]
    pub normal_url: Option<String>,
    #[serde(rename = "ColliderURL")]
    pub collider_url: Option<String>,
    #[serde(rename = "Convex")]
    pub convex: Option<bool>,
    #[serde(rename = "MaterialIndex")]
    pub material_index: Option<i64>,
    #[serde(rename = "TypeIndex")]
    pub type_index: Option<i64>,
    #[serde(rename = "CustomShader")]
    pub custom_shader: Option<CustomShader2>,
    #[serde(rename = "CastShadows")]
    pub cast_shadows: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomShader2 {
    #[serde(rename = "SpecularColor")]
    pub specular_color: Option<SpecularColor2>,
    #[serde(rename = "SpecularIntensity")]
    pub specular_intensity: Option<f64>,
    #[serde(rename = "SpecularSharpness")]
    pub specular_sharpness: Option<f64>,
    #[serde(rename = "FresnelStrength")]
    pub fresnel_strength: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecularColor2 {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bag2 {
    #[serde(rename = "Order")]
    pub order: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicsMaterial2 {
    #[serde(rename = "StaticFriction")]
    pub static_friction: Option<f64>,
    #[serde(rename = "DynamicFriction")]
    pub dynamic_friction: Option<f64>,
    #[serde(rename = "Bounciness")]
    pub bounciness: Option<f64>,
    #[serde(rename = "FrictionCombine")]
    pub friction_combine: Option<i64>,
    #[serde(rename = "BounceCombine")]
    pub bounce_combine: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rigidbody2 {
    #[serde(rename = "Mass")]
    pub mass: Option<f64>,
    #[serde(rename = "Drag")]
    pub drag: Option<f64>,
    #[serde(rename = "AngularDrag")]
    pub angular_drag: Option<f64>,
    #[serde(rename = "UseGravity")]
    pub use_gravity: Option<bool>,
}
