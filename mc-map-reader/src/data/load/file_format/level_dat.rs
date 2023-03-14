use crate::data::file_format::level_dat::*;

try_from_tag_for_module![{
    LevelDat => [
        "allowCommands": set_allow_commands,
        "BorderCenterX": set_border_center_x,
        "BorderCenterZ": set_border_center_z,
        "BorderDamagePerBlock": set_border_damage_per_block,
        "BorderSize": set_border_size,
        "BorderSafeZone": set_border_safe_zone,
        "BorderSizeLerpTarget": set_border_size_lerp_target,
        "BorderSizeLerpTime": set_border_size_lerp_time,
        "BorderWarningBlocks": set_border_warning_blocks,
        "clearWeatherTime": set_clear_weather_time,
        "CustomBossEvents": set_custom_boss_events,
        "DataPacks": set_data_packs,
        "DataVersion": set_data_version,
        "DayTime": set_day_time,
        "Difficulty": set_difficulty,
        "DifficultyLocked": set_difficulty_locked,
        "DimensionData": set_dimension_data,
        "GameRules": set_game_rules,
        "WorldGenSettings": set_world_gen_settings,
        "GameType": set_game_type,
        "generatorName": set_generator_name,
        "generatorOptions": set_generator_options,
        "generatorVersion": set_generator_version,
        "hardcore": set_hardcore,
        "initialized": set_initialized,
        "LastPlayed": set_last_played,
        "LevelName": set_level_name,
        "MapFeatures": set_map_features,
        "Player": set_player,
        "raining": set_raining,
        "rainTime": set_rain_time,
        "RandomSeed": set_random_seed,
        "SizeOnDisk": set_size_on_disk,
        "SpawnX": set_spawn_x,
        "SpawnY": set_spawn_y,
        "SpawnZ": set_spawn_z,
        "thundering": set_thundering,
        "thunderTime": set_thunder_time,
        "Time": set_time,
        "version": set_version,
        "Version": set_version_info,
        "WanderingTraderId": set_wandering_trader_id,
        "WanderingTraderSpawnChance": set_wandering_trader_spawn_chance,
        "WanderingTraderSpawnDelay": set_wandering_trader_spawn_delay,
        "WasModded": set_was_modded
    ]
},{ CustomBossEvent => [
    "Players": set_players,
    "Color": set_color,
    "CreateWorldFog": set_create_world_fog,
    "DarkenScreen": set_darken_screen,
    "Max": set_max,
    "Value": set_value,
    "Name": set_name,
    "Overlay": set_overlay,
    "PlayBossMusic": set_play_boss_music,
    "Visible": set_visible
]},{
DataPacks => [
    "Disabled": set_disabled,
    "Enabled": set_enabled
]},{
WorldGenSettings => [
    "bonus_chest": set_bonus_chest,
    "dimensions": set_dimensions,
    "seed": set_seed,
    "generate_features": set_generate_features
]},{

Version => [
    "Id": set_id,
    "Name": set_name,
    "Series": set_series,
    "Snapshot": set_snapshot
]}];
