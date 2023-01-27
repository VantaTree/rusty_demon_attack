use macroquad::prelude::*;
use macroquad::audio::{Sound, PlaySoundParams, load_sound, play_sound};
use std::collections::HashMap;

use crate::enemy::{EnemyColor, EnemyType};

#[derive(PartialEq, Eq, Hash)]
pub enum SoundIdentifier {
    EnemyShoot,
    EnemyOuch,
    PlayerOuch,
    PlayerShoot,
    SpawnMini,
    Spawn,
    Warning,
    WaveCleared,
}

pub struct Resources {
    pub demons_normal_purple: Vec<Texture2D>,
    pub demons_normal_green: Vec<Texture2D>,
    pub demons_normal_red: Vec<Texture2D>,
    pub demons_mini_purple: Vec<Texture2D>,
    pub demons_mini_green: Vec<Texture2D>,
    pub demons_mini_red: Vec<Texture2D>,

    pub demon_missile: Texture2D,
    pub player_missile: Texture2D,
    pub player: Texture2D,
    pub player_explotion: Texture2D,
    pub ground_bg: Texture2D,
    pub life: Texture2D,

    pub font: Font,

    pub sounds: HashMap<SoundIdentifier, Sound>,
}

impl Resources {
    pub fn new(
        demon_missile: Texture2D,
        player_missile: Texture2D,
        player: Texture2D,
        player_explotion: Texture2D,
        ground_bg: Texture2D,
        life: Texture2D,
        font: Font,
    ) -> Self {
        Resources {
            demons_normal_purple: Vec::<Texture2D>::new(),
            demons_normal_green: Vec::<Texture2D>::new(),
            demons_normal_red: Vec::<Texture2D>::new(),
            demons_mini_purple: Vec::<Texture2D>::new(),
            demons_mini_green: Vec::<Texture2D>::new(),
            demons_mini_red: Vec::<Texture2D>::new(),
            demon_missile,
            player_missile,
            player,
            player_explotion,
            ground_bg,
            life,
            font,
            sounds: HashMap::new(),
        }
    }

    pub async fn load_sound(&mut self, file_path: &str, identifier: SoundIdentifier) {
        let sound = load_sound(file_path).await.unwrap();
        self.sounds.insert(identifier, sound);
    }

    pub fn play_sfx(&self, identifier: SoundIdentifier, volume: f32) {
        if let Some(sound) = self.sounds.get(&identifier) {
            play_sound(*sound, PlaySoundParams{looped:false, volume});
            debug!("DEBUG:: sound played");
        }
    }

    pub async fn load_texture(
        &mut self,
        file_name: &str,
        enemy_color: EnemyColor,
        enemy_type: EnemyType,
    ) -> Result<(), FileError> {
        let texture: Texture2D = load_texture(file_name).await?;
        texture.set_filter(FilterMode::Nearest);
        let texture_vec = match enemy_type {
            EnemyType::Normal => match enemy_color {
                EnemyColor::Purple => &mut self.demons_normal_purple,
                EnemyColor::Green => &mut self.demons_normal_green,
                EnemyColor::Red => &mut self.demons_normal_red,
            },
            EnemyType::Mini => match enemy_color {
                EnemyColor::Purple => &mut self.demons_mini_purple,
                EnemyColor::Green => &mut self.demons_mini_green,
                EnemyColor::Red => &mut self.demons_mini_red,
            },
        };
        texture_vec.push(texture);
        Ok(())
    }
    pub fn rand_enemy_normal(&self, enemy_color: EnemyColor) -> Texture2D {
        let normal_list = match enemy_color {
            EnemyColor::Purple => &self.demons_normal_purple,
            EnemyColor::Green => &self.demons_normal_green,
            EnemyColor::Red => &self.demons_normal_red,
        };
        normal_list[rand::gen_range(0, normal_list.len())]
    }

    pub fn rand_enemy_mini(&self, enemy_color: EnemyColor) -> Texture2D {
        let mini_list = match enemy_color {
            EnemyColor::Purple => &self.demons_mini_purple,
            EnemyColor::Green => &self.demons_mini_green,
            EnemyColor::Red => &self.demons_mini_red,
        };
        mini_list[rand::gen_range(0, mini_list.len())]
    }
}

pub async fn load_resources(game_render_target: RenderTarget) -> Resources {
    let texture_player: Texture2D = load_texture("resources/player.png").await.unwrap();
    let texture_player_explotion: Texture2D = load_texture("resources/player_explotion.png")
        .await
        .unwrap();
    let texture_player_missile: Texture2D =
        load_texture("resources/player_missile.png").await.unwrap();
    let texture_demon_missile: Texture2D =
        load_texture("resources/demon_missile.png").await.unwrap();
    let texture_ground_bg: Texture2D = load_texture("resources/ground_bg.png").await.unwrap();
    let texture_life: Texture2D = load_texture("resources/life.png").await.unwrap();

    // set all textures filter mode to nearest
    for texture in [
        texture_player,
        texture_player_explotion,
        texture_player_missile,
        texture_demon_missile,
        texture_ground_bg,
        texture_life,
        game_render_target.texture,
    ]
    .iter()
    {
        texture.set_filter(FilterMode::Nearest);
    }

    let font = load_ttf_font("resources/Kenney Pixel Square.ttf")
        .await
        .unwrap();
    let mut resources = Resources::new(
        texture_demon_missile,
        texture_player_missile,
        texture_player,
        texture_player_explotion,
        texture_ground_bg,
        texture_life,
        font,
    );

    {
        use EnemyColor::{Green, Purple, Red};
        use EnemyType::{Mini, Normal};
        resources
            .load_texture("resources/demon_mini_green_1.png", Green, Mini)
            .await
            .unwrap();
        resources
            .load_texture("resources/demon_mini_red_1.png", Red, Mini)
            .await
            .unwrap();
        resources
            .load_texture("resources/demon_mini_purple_1.png", Purple, Mini)
            .await
            .unwrap();
        resources
            .load_texture("resources/demon_normal_green_1.png", Green, Normal)
            .await
            .unwrap();
        resources
            .load_texture("resources/demon_normal_green_2.png", Green, Normal)
            .await
            .unwrap();
        resources
            .load_texture("resources/demon_normal_purple_1.png", Purple, Normal)
            .await
            .unwrap();
        resources
            .load_texture("resources/demon_normal_purple_2.png", Purple, Normal)
            .await
            .unwrap();
        resources
            .load_texture("resources/demon_normal_red_1.png", Red, Normal)
            .await
            .unwrap();
    }
    {
        use SoundIdentifier::*;

        resources.load_sound("resources/sounds/enemy_shoot.wav", EnemyShoot).await;
        resources.load_sound("resources/sounds/player_shoot.wav", PlayerShoot).await;
        resources.load_sound("resources/sounds/spawn.wav", Spawn).await;
        resources.load_sound("resources/sounds/player_ouch.wav", PlayerOuch).await;
        resources.load_sound("resources/sounds/enemy_ouch.wav", EnemyOuch).await;
        resources.load_sound("resources/sounds/spawn_mini.wav", SpawnMini).await;
        resources.load_sound("resources/sounds/warning.wav", Warning).await;
        resources.load_sound("resources/sounds/wave_cleared.wav", WaveCleared).await;
    }
    resources
}
