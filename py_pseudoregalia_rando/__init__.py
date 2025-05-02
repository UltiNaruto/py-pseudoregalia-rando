from . import rust, version


def patch_game(config_str: str, game_path: str):
    return rust.patch_game(config_str, game_path)


__version__ = version.version
VERSION = version.version